use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::baseplate;
use crate::maps;
use crate::tokens;
use crate::fileload;
use crate::filetransfer;

pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrderEvent>()
            .add_systems(Update, recieve_orders)
            .add_event::<MoveCommand>()
            .add_systems(Update, recieve_move.after(recieve_orders))
            .add_event::<CreateTokenCommand>()
            .add_systems(Update, recieve_create_token.after(recieve_orders))
            .add_event::<RequestDataCommand>()
            .add_systems(Update, recieve_request_data.after(recieve_orders))
            .add_event::<CreateMapCommand>()
            .add_systems(Update, recieve_create_map.after(recieve_orders));
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct OrderEvent {
    pub command: Command,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Command {
    Move(MoveCommand),
    CreateToken(CreateTokenCommand),
    CreateMap(CreateMapCommand),
    RequestData(RequestDataCommand),
}

pub fn recieve_orders(
    mut ev_orders: EventReader<OrderEvent>,
    mut ev_move: EventWriter<MoveCommand>,
    mut ev_create_token: EventWriter<CreateTokenCommand>,
    mut ev_create_map: EventWriter<CreateMapCommand>,
    mut ev_request_data: EventWriter<RequestDataCommand>,
) {
    for ord_ev in ev_orders.iter() {
        match &ord_ev.command {
            Command::Move(cmd) => ev_move.send(*cmd),
            Command::CreateToken(cmd) => ev_create_token.send(cmd.clone()),
            Command::CreateMap(cmd) => ev_create_map.send(cmd.clone()),
            Command::RequestData(cmd) => ev_request_data.send(cmd.clone()),
        }
    }
}

#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct MoveCommand {
    pub x: f32,
    pub y: f32,
    pub id: baseplate::ID,
}

fn recieve_move(
    mut ev_move: EventReader<MoveCommand>,
    mut tokens: Query<(&baseplate::ID, &mut Transform)>,
) {
    for mov_ev in ev_move.iter() {
        for mut token in tokens.iter_mut() {
            if token.0 .0 == mov_ev.id.0 {
                token.1.translation.x = mov_ev.x;
                token.1.translation.z = mov_ev.y;
            }
        }
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct CreateTokenCommand {
    pub x: f32,
    pub y: f32,
    pub id: baseplate::ID,
    pub url: String,
}

fn recieve_create_token(
    mut ev_create_token: EventReader<CreateTokenCommand>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_create_token.iter() {
        commands.spawn(tokens::TokenBundle::new(
            ev.id,
            Vec3::new(ev.x, 0.1, ev.y),
            &mut meshes,
            &mut materials,
            &asset_server,
        ));
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct CreateMapCommand {
    pub x: f32,
    pub y: f32,
    pub map_id: maps::MapId,
    pub data_id: fileload::LoadIdentifier,
}

fn recieve_create_map(
    mut ev_create_map: EventReader<CreateMapCommand>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_load: EventWriter<fileload::LoadRequest>,
) {
    for ev in ev_create_map.iter() {
        commands.spawn(maps::MapBundle::new(
            ev.map_id,
            Vec3::new(ev.x, 0., ev.y),
            &mut meshes,
            &mut materials,
        ));
        ev_load.send(
            fileload::LoadRequest{
                id: ev.data_id.clone(),
                endpoint: fileload::FileEndpoint::Map(ev.map_id),
            }
        )
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct RequestDataCommand {
    pub section: filetransfer::DataSectionIdentifier,
}

fn recieve_request_data(
    mut ev_request_data: EventReader<RequestDataCommand>,
) {
    for ev in ev_request_data.iter() {
        let start = ev.section.start;
        let end = ev.section.end;
        println!("Requested Data {start} - {end}");
    }
}
