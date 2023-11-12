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
            .add_event::<SendUploadRequest>()
            .add_systems(Update, send_upload_requests)
            .add_event::<SuccessfulUploadLock>()
            .add_systems(Update, recieve_successful_lock)
            .add_event::<DataRequest>()
            .add_systems(Update, recieve_data_request)
            .add_event::<IncomingDownload>()
            .add_systems(Update, recieve_data)
            .insert_resource(UploadState{state: None})
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
    mut upload_requests: EventWriter<SendUploadRequest>,
) {
    match download.state {
        None => {
            if let Some(new_download) = queue.queue.pop_front() {
                upload_requests.send(SendUploadRequest{
                    recipient: networking::RecepientPeer::All,
                    load_id: new_download.id.clone(),
                });
                let size = new_download.id.size;
                println!("Downloading file: {size}");
                download.state = Some(FileDownload::new(new_download));
            }
        },
        Some(_) => (),
    }
}

#[derive(Event)]
pub struct SendUploadRequest{
    recipient: networking::RecepientPeer,
    load_id: fileload::LoadIdentifier,
}

pub fn send_upload_requests(
    mut ev_send_upload_request: EventReader<SendUploadRequest>,
    mut ev_networked: EventWriter<networking::NetworkedCommandEvent>,
    local_peer_id: Option<Res<networking::LocalPeerId>>
){
    let Some(local_peer_id) = local_peer_id else {
        return
    };
    for ev in ev_send_upload_request.iter() {
        ev_networked.send(networking::NetworkedCommandEvent{
            order: orders::OrderEvent{
                command: orders::Command::RequestUploadLock(orders::RequestUploadLockCommand{
                    load_id: ev.load_id.clone(),
                    peer_id: local_peer_id.id,
                })
            },
            reliability: networking::NetworkReliability::Reliable,
            peer_id: ev.recipient,
        });
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
    data: Vec<u8>,
}

const REQUEST_BYTES: usize = 8192;

impl FileDownload{
    fn new(value: fileload::LoadRequest) -> Self {
        let mut sections = Vec::<DataSectionIdentifier>::new();

        for i in 0..=(value.id.size / REQUEST_BYTES) {
            let start = i * REQUEST_BYTES;
            let end = min(start + REQUEST_BYTES, value.id.size);
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

#[derive(Event, Serialize, Deserialize, Clone)]
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
        bank.insert_data(&state.request.id.data_id, state.data.clone().into());
        ev_load.send(state.request.clone());
        download.state = None;
    }
}

pub fn download_file(
    mut download: ResMut<DownloadState>,
    mut ev_networked: EventWriter<networking::NetworkedCommandEvent>,
    local_peer_id: Option<Res<networking::LocalPeerId>>,
) {
    //If there is a FileDownload
    let download = match &mut download.state {
        None => return,
        Some(x) => x, 
    };

    let Some(local_peer_id) = local_peer_id else {
        return;
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
                        peer_id: local_peer_id.id,
                    })
                },
                reliability: networking::NetworkReliability::Reliable,
                peer_id: networking::RecepientPeer::Peer(peer.id),
            })
        }
    } 
}

#[derive(Event)]
pub struct SuccessfulUploadLock{
    pub peer_id: PeerId,
}

pub fn recieve_successful_lock(
    mut ev_successful_upload_lock: EventReader<SuccessfulUploadLock>,
    mut download: ResMut<DownloadState>,
) {
    let Some(ref mut download) = download.state else {
        return
    };
    for ev in ev_successful_upload_lock.iter() {
        download.peers.push(UploadingPeer{
            id: ev.peer_id,
            current_request: None,
        });
    }
}

#[derive(Event)]
pub struct UploadRequest{
    pub load_id: fileload::LoadIdentifier,
    pub peer_id: PeerId,
}

pub fn lock_upload(
    mut ev_upload_request: EventReader<UploadRequest>,
    mut upload_state: ResMut<UploadState>,
    mut ev_networked: EventWriter<networking::NetworkedCommandEvent>,
    bank: Res<bank::Bank>,
    local_peer_id: Option<Res<networking::LocalPeerId>>
) {
    let Some(local_peer_id) = local_peer_id else {
        return
    };
    for ev in ev_upload_request.iter() {
        if upload_state.state.is_none() {
            let Some(file_data) = bank.request_data(&ev.load_id.data_id) else {
                continue;
            };
            upload_state.state = Some(FileUpload{
                target_peer_id: ev.peer_id,
                file: file_data.clone(),
            });
            ev_networked.send(
                networking::NetworkedCommandEvent{
                    peer_id: networking::RecepientPeer::Peer(ev.peer_id),
                    reliability: networking::NetworkReliability::Reliable,
                    order: orders::OrderEvent{
                        command: orders::Command::SuccessfulUploadLock(
                            orders::SuccessfulUploadLockedCommand{
                                peer_id: local_peer_id.id,
                            }
                        )
                    }
                }
            )
        }
    }
}

#[derive(Resource)]
pub struct UploadState{
    state: Option<FileUpload>,
}

pub struct FileUpload{
    target_peer_id: PeerId,
    file: Arc<Vec<u8>>,
}

#[derive(Event)]
pub struct DataRequest{
    pub peer_id: PeerId,
    pub section: DataSectionIdentifier,
}

pub fn recieve_data_request(
    upload_state: ResMut<UploadState>,
    mut ev_data_request: EventReader<DataRequest>,
    mut ev_networked: EventWriter<networking::NetworkedCommandEvent>,
    local_peer_id: Option<Res<networking::LocalPeerId>>
){
    for ev in ev_data_request.iter() {
        let Some(ref upload) = upload_state.state else {
            println!("No upload state");
            return;
        };

        let Some(ref local_peer_id) = local_peer_id else {
            return;
        };

        if ev.peer_id != upload.target_peer_id {
            let wrong_peer_id = ev.peer_id;
            let correct_peer_id = upload.target_peer_id;
            println!("Data request from wrong peer id: {wrong_peer_id} Locked to: {correct_peer_id}");
        }

        let Some(data) = upload.file.get(ev.section.start..ev.section.end) else {
            println!("requested data outside bounds. File Size: {:?} : Requested: {:?} - {:?} : Index: {:?}", upload.file.len(), ev.section.start, ev.section.end, ev.section.index);
            continue;
        };
        ev_networked.send(
            networking::NetworkedCommandEvent{
                peer_id: networking::RecepientPeer::Peer(ev.peer_id),
                reliability: networking::NetworkReliability::Reliable,
                order: orders::OrderEvent{
                    command: orders::Command::RecieveData(
                        orders::RecieveDataCommand{
                            peer_id: local_peer_id.id,
                            data: DownloadedSection {
                                data: data.to_vec(),
                                id: ev.section.clone(),
                            },
                        }
                    )

                }
            }
        );
    }
}

#[derive(Event)]
pub struct IncomingDownload{
    pub downloaded_section: Arc<DownloadedSection>,
    pub peer_id: PeerId,
}

pub fn recieve_data(
    mut download: ResMut<DownloadState>,
    mut ev_incoming_downloads: EventReader<IncomingDownload>,
) {
    for ev in ev_incoming_downloads.iter() {
        let Some(ref mut download) = download.state else {
            println!("Recieved incoming data with no download");
            return;
        };

        for i in ev.downloaded_section.id.start.. ev.downloaded_section.id.end {
            let data_index = i - ev.downloaded_section.id.start;
            let _ = std::mem::replace(&mut download.data[i], ev.downloaded_section.data[data_index]);
        }
        //For each incoming set of data
        for peer in download.peers.iter_mut() {
            //Remove the request from that peer, as it's done
            if peer.id == ev.peer_id {
                peer.current_request = None;
            }
        }
    }
}
