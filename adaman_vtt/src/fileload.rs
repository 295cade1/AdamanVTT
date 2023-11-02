use bevy::prelude::*;
use std::sync::Arc;
use serde::Deserialize;
use serde::Serialize;

use crate::bank;
use crate::maps;

pub struct FileLoad;

impl Plugin for FileLoad {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadRequest>()
            .add_systems(Update, recieve_request)
            .add_systems(Update, process_successful_load)
            .add_event::<SuccessfulLoad>();
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct LoadRequest {
    data_id: bank::DataId,
    endpoint: FileEndpoint,
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct SuccessfulLoad {
    data_id: bank::DataId,
    endpoint: FileEndpoint,
    data: Arc<Vec<u8>>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum FileEndpoint {
    Map(maps::MapId),
}


pub fn recieve_request(
    mut ev_load: EventReader<LoadRequest>,
    mut ev_success: EventWriter<SuccessfulLoad>,
    bank: Res<bank::Bank>
) {
    for ld_ev in ev_load.iter() {
        if let Some(data) = bank.request_data(&ld_ev.data_id) {
            ev_success.send(SuccessfulLoad{
                data_id: ld_ev.data_id,
                endpoint: ld_ev.endpoint,
                data: data.clone(),
            })
        }else{
            //TODO Network Loading
        }
    }
}

pub fn process_successful_load(
    mut ev_success: EventReader<SuccessfulLoad>,
    mut ev_map_load: EventWriter<maps::MapLoad>,
) {
    for succ_ev in ev_success.iter() {
        match succ_ev.endpoint {
            FileEndpoint::Map(id) => ev_map_load.send(maps::MapLoad{
                map_id: id,
                data: succ_ev.data.clone(),
            })
        }
    }
}
