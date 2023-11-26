use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_mod_picking::prelude::*;

use futures_lite::future;
use rfd::AsyncFileDialog;

use crate::maps;
use crate::tokens;
use crate::networking;
use crate::bank;
use crate::orders;
use crate::dd2vtt;
use crate::fileload;
use crate::files;
use crate::encounters;

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
    tokens: Query<(&tokens::TokenId, &Transform)>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_q.single();

    let mut dict = std::collections::HashMap::<tokens::TokenId, (f32, f32)>::new();
    for drag_ev in ev_drag.read() {
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

#[derive(Component)]
pub struct MapFile {
    file: Task<Option<Vec<u8>>>,
    name: String,
    //x: f32,
    //y: f32,
}

pub fn create_map_from_file(mut commands: Commands, name: String) {
    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(async move{
        let handle = AsyncFileDialog::new()
            .add_filter("image", &["png", "jpg"])
            .add_filter("universalVTT", &["dd2vtt", "json"])
            .pick_file().await;
        let Some(handle) = handle else {
            return None;
        };
        Some(handle.read().await)
    });
    commands.spawn(MapFile {file: task, name});
}

//Poll to see if the user has selected a path
pub fn poll_for_map(
    mut commands: Commands, 
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
    mut tasks: Query<(Entity, &mut MapFile)>,
    mut bank: Option<ResMut<bank::Bank>>,
    mut register_event: EventWriter<files::RegisterMap>,
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
        let Some(contents) = result else {
            continue;
        };
        let contents = String::from_utf8(contents).unwrap();
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

        //Insert the file data into the bank
        let data = serde_json::to_vec(&data).ok().unwrap();
        let load_identifier = bank.store(data.into());

        register_event.send(
            files::RegisterMap{
                load_identifier: load_identifier.clone(),
                name: selected_file.name.clone(),
            }
        );
        create_map(
            load_identifier.clone(),
            &mut ev_client,
        )
    }
}

pub fn create_map(
    load_identifier: fileload::LoadIdentifier,
    ev_client: &mut EventWriter<networking::ClientCommandEvent>,
) {
    //Send the packet to the other peers to have them create the map
    ev_client.send(networking::ClientCommandEvent {
        order: orders::OrderEvent {
            command: orders::Command::CreateMap(orders::CreateMapCommand {
                x: 0.,
                y: 0.,
                data_id: load_identifier.clone(),
                map_id: maps::get_new_id(),
            }),
        },
        reliability: networking::NetworkReliability::Reliable,
    });
}

pub fn load_encounter(
    load_identifier: fileload::LoadIdentifier,
    ev_client: &mut EventWriter<networking::ClientCommandEvent>,
) {
    //Send the packet to the other peers to have them create the map
    ev_client.send(networking::ClientCommandEvent {
        order: orders::OrderEvent {
            command: orders::Command::LoadEncounter(orders::LoadEncounterCommand {
                load_identifier: load_identifier.clone(),
            }),
        },
        reliability: networking::NetworkReliability::Reliable,
    });
}

pub fn create_token(
    load_identifier: fileload::LoadIdentifier,
    ev_client: &mut EventWriter<networking::ClientCommandEvent>,
) {
    ev_client.send(networking::ClientCommandEvent {
        order: orders::OrderEvent {
            command: orders::Command::CreateToken(orders::CreateTokenCommand {
                x: 0.,
                y: 0.,
                id: tokens::get_new_id(),
                load_identifier, 
            }),
        },
        reliability: networking::NetworkReliability::Reliable,
    });
}

pub fn save_encounter(
    mut ev_save_encounter: EventWriter<encounters::EncounterSave>,
    name: String,
) {
    ev_save_encounter.send(encounters::EncounterSave{
        name,
    });
}
