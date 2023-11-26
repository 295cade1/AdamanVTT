use bevy::prelude::*;
use bevy::window::RequestRedraw;
use serde::{Deserialize, Serialize};
use bevy_matchbox::prelude::PeerId;
use std::sync::Arc;

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
            .add_systems(Update, recieve_data_request.after(recieve_orders))

            .add_event::<RequestUploadLockCommand>()
            .add_systems(Update, recieve_upload_request.after(recieve_orders))

            .add_event::<SuccessfulUploadLockedCommand>()
            .add_systems(Update, recieve_successful_upload_lock.after(recieve_orders))

            .add_event::<RecieveDataCommand>()
            .add_systems(Update, recieve_recieve_data.after(recieve_orders))

            .add_event::<UnlockUploadCommand>()
            .add_systems(Update, recieve_unlock_upload.after(recieve_orders))

            .add_event::<UploadAvailableCommand>()
            .add_systems(Update, recieve_upload_available.after(recieve_orders))

            .add_event::<CreateMapCommand>()
            .add_systems(Update, recieve_create_map.after(recieve_orders))

            .add_event::<LoadEncounterCommand>()
            .add_systems(Update, recieve_load_encounter.after(recieve_orders))
        ;
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
    LoadEncounter(LoadEncounterCommand),
    RequestData(RequestDataCommand),
    RequestUploadLock(RequestUploadLockCommand),
    SuccessfulUploadLock(SuccessfulUploadLockedCommand),
    RecieveData(RecieveDataCommand),
    UnlockUpload(UnlockUploadCommand),
    UploadAvailable(UploadAvailableCommand),
}

#[allow(clippy::too_many_arguments)]
pub fn recieve_orders(
    mut ev_orders: EventReader<OrderEvent>,
    mut ev_move: EventWriter<MoveCommand>,
    mut ev_create_token: EventWriter<CreateTokenCommand>,
    mut ev_create_map: EventWriter<CreateMapCommand>,
    mut ev_load_encounter: EventWriter<LoadEncounterCommand>,
    mut ev_request_data: EventWriter<RequestDataCommand>,
    mut ev_request_upload_lock: EventWriter<RequestUploadLockCommand>,
    mut ev_successful_upload_locked: EventWriter<SuccessfulUploadLockedCommand>,
    mut ev_recieve_data: EventWriter<RecieveDataCommand>,
    mut ev_unlock_upload: EventWriter<UnlockUploadCommand>,
    mut ev_upload_available: EventWriter<UploadAvailableCommand>,
) {
    for ord_ev in ev_orders.read() {
        //match &ord_ev.command {
            //Command::Move(_cmd) => println!("Move"),
            //Command::CreateToken(_cmd) => println!("Create Token"),
            //Command::CreateMap(_cmd) => println!("Create Map"),
            //Command::RequestData(_cmd) => println!("Request Data"),
            //Command::RequestUploadLock(_cmd) => println!("Request Lock"),
            //Command::SuccessfulUploadLock(_cmd) => println!("Successful Upload Lock"),
            //Command::RecieveData(_cmd) => println!("Recieve Data"),
        //}
        match &ord_ev.command {
            Command::Move(cmd) => ev_move.send(*cmd),
            Command::CreateToken(cmd) => ev_create_token.send(cmd.clone()),
            Command::CreateMap(cmd) => ev_create_map.send(cmd.clone()),
            Command::LoadEncounter(cmd) => ev_load_encounter.send(cmd.clone()),
            Command::RequestData(cmd) => ev_request_data.send(cmd.clone()),
            Command::RequestUploadLock(cmd) => ev_request_upload_lock.send(cmd.clone()),
            Command::SuccessfulUploadLock(cmd) => ev_successful_upload_locked.send(cmd.clone()),
            Command::RecieveData(cmd) => ev_recieve_data.send(cmd.clone()),
            Command::UnlockUpload(cmd) => ev_unlock_upload.send(cmd.clone()),
            Command::UploadAvailable(cmd) => ev_upload_available.send(cmd.clone()),
        }
    }
}

#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct MoveCommand {
    pub x: f32,
    pub y: f32,
    pub id: tokens::TokenId,
}

fn recieve_move(
    mut ev_move: EventReader<MoveCommand>,
    mut tokens: Query<(&tokens::TokenId, &mut Transform)>,
    mut event: EventWriter<RequestRedraw>,
) {
    for mov_ev in ev_move.read() {
        for mut token in tokens.iter_mut() {
            if token.0 .0 == mov_ev.id.0 {
                token.1.translation.x = mov_ev.x;
                token.1.translation.z = mov_ev.y;
                event.send(RequestRedraw)
            }
        }
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct CreateTokenCommand {
    pub x: f32,
    pub y: f32,
    pub id: tokens::TokenId,
    pub load_identifier: fileload::LoadIdentifier,
}

fn recieve_create_token(
    mut ev_create_token: EventReader<CreateTokenCommand>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_create_token.read() {
        commands.spawn(tokens::TokenBundle::new(
            ev.id,
            ev.load_identifier.clone(),
            Vec3::new(ev.x, 0.01, ev.y),
            &mut meshes,
            &mut materials,
        ));
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct RequestUploadLockCommand {
    pub load_id: fileload::LoadIdentifier,
    pub peer_id: PeerId,
}

fn recieve_upload_request(
    mut ev_order: EventReader<RequestUploadLockCommand>,
    mut ev_pass: EventWriter<filetransfer::UploadRequest>,
) {
    for ev in ev_order.read() {
        ev_pass.send(filetransfer::UploadRequest{
            load_id: ev.load_id.clone(),
            peer_id: ev.peer_id,
        })
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct SuccessfulUploadLockedCommand {
    pub peer_id: PeerId,
}

fn recieve_successful_upload_lock(
    mut ev_order: EventReader<SuccessfulUploadLockedCommand>,
    mut ev_pass: EventWriter<filetransfer::SuccessfulUploadLock>,
) {
    for ev in ev_order.read() {
        ev_pass.send(filetransfer::SuccessfulUploadLock{
            peer_id: ev.peer_id,
        });
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct RequestDataCommand {
    pub peer_id: PeerId,
    pub section: filetransfer::DataSectionIdentifier,
}

fn recieve_data_request(
    mut ev_request_data: EventReader<RequestDataCommand>,
    mut ev_data_request: EventWriter<filetransfer::DataRequest>,
) {
    for ev in ev_request_data.read() {
        ev_data_request.send(
            filetransfer::DataRequest{
                peer_id: ev.peer_id,
                section: ev.section.clone(),
            }
        )
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct RecieveDataCommand {
    pub peer_id: PeerId,
    pub data: filetransfer::DownloadedSection,
}

fn recieve_recieve_data(
    mut ev_recieve_data: EventReader<RecieveDataCommand>,
    mut ev_incoming_download: EventWriter<filetransfer::IncomingDownload>,
) {
    for ev in ev_recieve_data.read() {
        ev_incoming_download.send(
            filetransfer::IncomingDownload{
                downloaded_section: Arc::new(ev.data.clone()),
                peer_id: ev.peer_id,
            }
        );
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct UnlockUploadCommand;

fn recieve_unlock_upload(
    mut ev_order: EventReader<UnlockUploadCommand>,
    mut ev_pass: EventWriter<filetransfer::UnlockUpload>,
) {
    for _ev in ev_order.read() {
        ev_pass.send(filetransfer::UnlockUpload);
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct UploadAvailableCommand {
    pub peer_id: PeerId,
}

fn recieve_upload_available(
    mut ev_order: EventReader<UploadAvailableCommand>,
    mut ev_pass: EventWriter<filetransfer::UploadAvailable>,
) {
    for ev in ev_order.read() {
        ev_pass.send(filetransfer::UploadAvailable{
            peer_id: ev.peer_id,
        });
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
    for ev in ev_create_map.read() {
        commands.spawn(maps::MapBundle::new(
            ev.map_id,
            ev.data_id.clone(),
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
pub struct LoadEncounterCommand {
    pub load_identifier: fileload::LoadIdentifier,
}

fn recieve_load_encounter(
    mut ev_load_encounter: EventReader<LoadEncounterCommand>,
    mut ev_load: EventWriter<fileload::LoadRequest>,
) {
    for ev in ev_load_encounter.read() {
        ev_load.send(
            fileload::LoadRequest{
                id: ev.load_identifier.clone(),
                endpoint: fileload::FileEndpoint::Encounter,
            }
        )
    }
}

