use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use bevy_matchbox::prelude::*;
use std::cmp::min;
use std::sync::Arc;

use crate::fileload;
use crate::bank;
use crate::networking;
use crate::orders;

pub struct FileTransfer;

impl Plugin for FileTransfer {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadQueue::new())
            .add_systems(Update, handle_load_queue)
            .add_systems(Update, download_file)
            .add_systems(Update, complete_download)
            .add_event::<UploadRequest>()
            .add_systems(Update, lock_upload)
            .insert_resource(DownloadState{state: None});
    }
}

#[derive(Resource)]
pub struct LoadQueue{
    queue: VecDeque<fileload::LoadRequest>,
}

impl LoadQueue {
    fn new() -> LoadQueue {
        LoadQueue {
            queue: VecDeque::new(),
        }
    }
    pub fn add(&mut self, request: fileload::LoadRequest) {
        self.queue.push_back(request);
    }
}

pub fn handle_load_queue(
    mut queue: ResMut<LoadQueue>,
    mut download: ResMut<DownloadState>,
) {
    match download.state {
        None => {
            if let Some(new_download) = queue.queue.pop_front() {
                download.state = Some(FileDownload::new(new_download));
            }
        },
        Some(_) => (),
    }
}

#[derive(Resource)]
pub struct DownloadState{
    pub state: Option<FileDownload>,
}

pub struct FileDownload{
    request: fileload::LoadRequest,
    peers: Vec<UploadingPeer>,
    sections: Vec<DataSectionIdentifier>,
    data: Arc<Vec<u8>>,
}

const REQUEST_BYTES: usize = 1024;

impl FileDownload{
    fn new(value: fileload::LoadRequest) -> Self {
        let mut sections = Vec::<DataSectionIdentifier>::new();

        for i in 0..(value.id.size % REQUEST_BYTES) {
            let start = i;
            let end = min(i + REQUEST_BYTES, value.id.size);
            sections.push(DataSectionIdentifier{
                index: i,
                start,
                end,
                data_id: value.id.data_id,
            })
        }

        let mut data = Vec::<u8>::with_capacity(value.id.size);
        for _i in 0..value.id.size {
            data.push(0);
        }
        
        FileDownload{
            request: value,
            peers: Vec::<UploadingPeer>::new(),
            data: data.into(),
            sections,
        }
    }

    fn is_done(&self) -> bool {
        if !self.sections.is_empty() {
            return false;
        }
        for peer in self.peers.iter() {
            match &peer.current_request {
                None => (),
                Some(_) => {return false},
            }
        }
        true
    }
}

struct UploadingPeer{
    id: PeerId,
    current_request: Option<DataSectionIdentifier>,
}

pub struct DownloadedSection {
    pub data: Vec<u8>,
    pub id: DataSectionIdentifier,
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct DataSectionIdentifier{
    pub index: usize,
    pub start: usize,
    pub end: usize,
    pub data_id: bank::DataId,
}

pub fn complete_download(
    mut download: ResMut<DownloadState>,
    mut ev_load: EventWriter<fileload::LoadRequest>,
    mut bank: ResMut<bank::Bank>
) {
    //If there is a FileDownload
    let state = match &download.state {
        None => return,
        Some(x) => x, 
    };

    if state.is_done() {
        bank.insert_data(&state.request.id.data_id, state.data.clone());
        ev_load.send(state.request.clone());
        download.state = None;
    }
}

pub fn download_file(
    mut download: ResMut<DownloadState>,
    mut ev_networked: EventWriter<networking::NetworkedCommandEvent>,
) {
    //If there is a FileDownload
    let download = match &mut download.state {
        None => return,
        Some(x) => x, 
    };

    //If there are any peers that aren't pending data
    for peer in download.peers.iter_mut() {
        if peer.current_request.is_none() && !download.sections.is_empty() { 
            let Some(section) = download.sections.pop() else {
                continue;
            };
            peer.current_request = Some(section.clone());
            //Request part of file
            //Request a section that isn't pending (unless there are no unloaded sections)
            ev_networked.send(networking::NetworkedCommandEvent{
                order: orders::OrderEvent{
                    command: orders::Command::RequestData(orders::RequestDataCommand{
                        section,
                    })
                },
                reliability: networking::NetworkReliability::Reliable,
                peer_id: networking::RecepientPeer::Peer(peer.id),
            })
        }
    } 
}

#[derive(Event)]
pub struct UploadRequest{
    pub request: fileload::LoadRequest,
}

pub fn lock_upload(
    mut ev_upload_request: EventReader<UploadRequest>,
    mut upload_state: ResMut<UploadState>,
) {
    for ev in ev_upload_request.iter() {
        if upload_state.state.is_none() {
            //upload_state.state
        }
    }
}

#[derive(Resource)]
pub struct UploadState{
    state: Option<FileUpload>,
}

pub struct FileUpload{
    target_peer_id: PeerId,
    data_id: bank::DataId,
}
