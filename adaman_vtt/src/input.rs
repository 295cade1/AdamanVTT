use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_mod_picking::prelude::*;

use futures_lite::future;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

use crate::baseplate;
use crate::maps;
use crate::networking;
use crate::bank;
use crate::orders;
use crate::dd2vtt;
use crate::fileload;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TokenDragEvent>()
            .add_systems(Update, poll_for_map)
            .add_systems(
                Update,
                recieve_dragging_tokens.before(orders::recieve_orders),
            );
    }
}

#[derive(Event)]
pub struct TokenDragEvent {
    pub input: ListenerInput<Pointer<Drag>>,
}

impl From<ListenerInput<Pointer<Drag>>> for TokenDragEvent {
    fn from(input: ListenerInput<Pointer<Drag>>) -> TokenDragEvent {
        TokenDragEvent { input }
    }
}

fn recieve_dragging_tokens(
    mut ev_drag: EventReader<TokenDragEvent>,
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
    tokens: Query<(&baseplate::ID, &Transform)>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_q.single();

    let mut dict = std::collections::HashMap::<baseplate::ID, (f32, f32)>::new();
    for drag_ev in ev_drag.iter() {
        if let Ok(token) = tokens.get(drag_ev.input.listener()) {
            dict.insert(
                *token.0,
                (
                    drag_ev.input.pointer_location.position.x,
                    drag_ev.input.pointer_location.position.y,
                ),
            );
        }
    }

    for token in dict.iter() {
        if let Some(new_pos) = get_plane_intersection(
            camera
                .viewport_to_world(camera_transform, Vec2::new(token.1 .0, token.1 .1))
                .unwrap(),
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
        ) {
            ev_client.send(networking::ClientCommandEvent {
                order: orders::OrderEvent {
                    command: orders::Command::Move(orders::MoveCommand {
                        id: *token.0,
                        x: new_pos.x,
                        y: new_pos.z,
                    }),
                },
                reliability: networking::NetworkReliability::Reliable,
            })
        }
    }
}

fn get_plane_intersection(ray: Ray, plane_origin: Vec3, plane_normal: Vec3) -> Option<Vec3> {
    let ray_dir = ray.direction;
    let dot = plane_normal.dot(ray_dir);
    if dot.abs() > 1e-6 {
        //# The factor of the point between p0 -> p1 (0 - 1)
        //# if 'fac' is between (0 - 1) the point intersects with the segment.
        //# Otherwise:
        //#  < 0.0: behind p0.
        //#  > 1.0: infront of p1.
        let w = ray.origin - plane_origin;
        let fac = -plane_normal.dot(w) / dot;
        let u = ray_dir * fac;
        Some(ray.origin + u)
    } else {
        Option::None
    }
}

//UI funcs
pub fn create_token(x: f32, y: f32, url: Option<&str>) -> networking::ClientCommandEvent {
    let url = url.unwrap_or("https://api.open5e.com/static/img/monsters/hezrou.png");
    networking::ClientCommandEvent {
        order: orders::OrderEvent {
            command: orders::Command::CreateToken(orders::CreateTokenCommand {
                x,
                y,
                id: baseplate::ID(baseplate::get_new_id()),
                url: url.to_string(),
            }),
        },
        reliability: networking::NetworkReliability::Reliable,
    }
}

#[derive(Component)]
pub struct MapFile {
    file: Task<Option<PathBuf>>,
    //x: f32,
    //y: f32,
}

pub fn create_map_from_file(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(async move {
        FileDialog::new()
            .add_filter("image", &["png", "jpg"])
            .add_filter("universalVTT", &["dd2vtt", "json"])
            .pick_file()
    });
    commands.spawn(MapFile { file: task});
}

//Poll to see if the user has selected a path
pub fn poll_for_map(
    mut commands: Commands, 
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
    mut tasks: Query<(Entity, &mut MapFile)>,
    mut bank: Option<ResMut<bank::Bank>>
) {
    //Make sure the bank is ready
    let Some(ref mut bank) = bank else {
        return;
    };
    for (entity, mut selected_file) in tasks.iter_mut() {
        //Poll the file selector entity to see if the user has selected a file yet
        let Some(result) = future::block_on(future::poll_once(&mut selected_file.file)) else {
            continue;
        };

        //println!("{:?}", result);
        commands.entity(entity).remove::<MapFile>();

        //Make sure it's like a real thing
        let Some(path) = result else {
            continue;
        };

        //println!("{:?}", path);

        //Read the file
        let Ok(contents) = fs::read_to_string(path) else {
            continue;
        };

        //println!("{:?}", contents);

        //Deserialize it into the RawMapData
        let mut data: Option<maps::MapData> = None;
        if let Ok(deserialized) = serde_json::from_str::<dd2vtt::DD2VTT>(contents.as_str()) {
            data = Some(deserialized.into());
        };

        //Make sure we found some data
        let Some(data) = data else {
            continue;
        };

        let id = bank::get_new_id();
        //Insert the file data into the bank
        let data = serde_json::to_vec(&data).ok().unwrap();

        let size = data.len();
        let hash = data.reflect_hash().expect("Unable to hash vec<u8>");

        bank.insert_data(&id, data.into());

        //Send the packet to the other peers to have them create the map
        ev_client.send(networking::ClientCommandEvent {
            order: orders::OrderEvent {
                command: orders::Command::CreateMap(orders::CreateMapCommand {
                    x: 0.,
                    y: 0.,
                    data_id: fileload::LoadIdentifier{
                        data_id: id,
                        size,
                        hash,
                    },
                    map_id: maps::get_new_id(),
                }),
            },
            reliability: networking::NetworkReliability::Reliable,
        });
    }
}
