use bevy::prelude::*;
use std::sync::Arc;
use serde::Deserialize;
use serde::Serialize;

use crate::bank;
use crate::maps;
use crate::filetransfer;

pub struct FileLoad;

impl Plugin for FileLoad {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadRequest>()
            .add_systems(Update, recieve_request)
            .add_systems(Update, process_successful_load)
            .add_event::<SuccessfulLoad>();
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoadIdentifier{
    pub data_id: bank::DataId,
    pub size: usize,
    pub hash: u64,
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct LoadRequest {
    pub id: LoadIdentifier,
    pub endpoint: FileEndpoint,
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct SuccessfulLoad {
    request: LoadRequest,
    data: Arc<Vec<u8>>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum FileEndpoint {
    Map(maps::MapId),
}

pub fn recieve_request(
    mut ev_load: EventReader<LoadRequest>,
    mut ev_success: EventWriter<SuccessfulLoad>,
    mut load_queue: ResMut<filetransfer::LoadQueue>,
    bank: Res<bank::Bank>
) {
    for ld_ev in ev_load.read() {
        if let Some(data) = bank.request_data(&ld_ev.id.data_id) {
            ev_success.send(SuccessfulLoad{
                request: ld_ev.clone(),
                data: data.clone(),
            })
        }else{
            //If we don't have it in the bank, put it into the queue to load from the network
            load_queue.add(ld_ev.clone());
        }
    }
}

pub fn process_successful_load(
    mut ev_success: EventReader<SuccessfulLoad>,
    mut ev_map_load: EventWriter<maps::MapLoad>,
) {
    for succ_ev in ev_success.read() {
        match succ_ev.request.endpoint {
            FileEndpoint::Map(id) => ev_map_load.send(maps::MapLoad{
                map_id: id,
                data: succ_ev.data.clone(),
            })
        }
    }
}
