use bevy::prelude::*;
use bevy_async_task::*;
use bevy_mod_picking::prelude::*;

use cute_dnd_dice;

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
            .add_event::<CreateMapFromFile>()
            .add_systems(
                Update,
                recieve_dragging_tokens.before(orders::recieve_orders),
            )
            .add_event::<CreateTokenFromData>()
            .add_systems(Update, create_token_from_data)
        ;
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

pub struct MapFile {
    file: Option<Vec<u8>>,
    name: String,
}

#[derive(Event, Resource, Clone)]
pub struct CreateMapFromFile {
    pub name: String,
}

//Poll to see if the user has selected a path
pub fn poll_for_map(
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
    mut bank: Option<ResMut<bank::Bank>>,
    mut register_event: EventWriter<files::RegisterMap>,
    mut poll_maps: AsyncTaskRunner<MapFile>,
    mut ev_load_map: EventReader<CreateMapFromFile>,
) {
    //Make sure the bank is ready
    let Some(ref mut bank) = bank else {
        return;
    };

    let mut load_event: Option<CreateMapFromFile> = None;
    for ev in ev_load_map.read() {
        load_event = Some(ev.clone());
    }

    match poll_maps.poll() {
        AsyncTaskStatus::Idle => {
            if let Some(ev) = load_event {
                let task = async move{
                    let handle = AsyncFileDialog::new()
                        //.add_filter("image", &["png", "jpg"]) TODO Add img maps
                        .add_filter("universalVTT", &["dd2vtt", "json"])
                        .pick_file().await;
                    let Some(handle) = handle else {
                        return MapFile{
                            file: None,
                            name: ev.name.clone(),
                        };
                    };
                    MapFile {
                        file: Some(handle.read().await),
                        name: ev.name.clone(),
                    }
                };
                println!("Started Map Loading");
                poll_maps.start(task);
            }
        },
        AsyncTaskStatus::Pending => {
        },
        AsyncTaskStatus::Finished(file) => {
            //Make sure it's like a real thing
            let Some(contents) = file.file else {
                return;
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
                return;
            };

            //Insert the file data into the bank
            let data = serde_json::to_vec(&data).ok().unwrap();
            let load_identifier = bank.store(data.into());

            register_event.send(
                files::RegisterMap{
                    load_identifier: load_identifier.clone(),
                    name: file.name.to_string(),
                }
            );
            create_map(
                load_identifier.clone(),
                &mut ev_client,
            );
        }
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

#[derive(Event)]
pub struct CreateTokenFromData{
    pub data: tokens::TokenData,
}

pub fn create_token_from_data(
    mut ev_create: EventReader<CreateTokenFromData>,
    mut bank: ResMut<bank::Bank>,
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
    mut register_event: EventWriter<files::RegisterToken>,
) {
    for ev in ev_create.read() {
        let data = &ev.data;
        //Insert the file data into the bank
        let data_serialized = serde_json::to_vec(data).ok().unwrap();
        let load_identifier = bank.store(data_serialized.into());

        register_event.send(
            files::RegisterToken{
                load_identifier: load_identifier.clone(),
                name: data.name.to_string(),
            }
        );
        create_token(
            load_identifier.clone(),
            &mut ev_client,
        );
    }
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

pub fn send_message(
    text: String,
    ev_client: &mut EventWriter<networking::ClientCommandEvent>,
    local_peer_id: Res<networking::LocalPeerId>,
) {
    let processed = process_dice(text.clone());
    let mut roll = false;
    let text = if processed.is_some() {
        roll = true;
        format!("Rolled: {}", processed.unwrap())
    } else {
        text
    };

    ev_client.send(
        networking::ClientCommandEvent {
            order: crate::orders::OrderEvent {
                command: crate::orders::Command::Message(
                    crate::ui::RecieveMessage{
                        text: text.clone(),
                        from: local_peer_id.id,
                        roll,
                    }
                )
            },
            reliability: networking::NetworkReliability::Reliable,
        }
    )
}

fn process_dice(text: String) -> Option<String> {
    let split: Vec<&str> = text.split(" ").collect();
    if split.starts_with(&["/roll"]) {
        Some(cute_dnd_dice::Roll::from_str(&split[1..].concat()).ok()?.roll().to_string())
    } else {
        None
    }
}
