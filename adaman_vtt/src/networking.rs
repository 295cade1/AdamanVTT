use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use bytemuck::*;

use crate::orders;
use crate::tokens;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, (open_socket, init_hashtable))
        .add_event::<NetworkedCommandEvent>()
        .add_event::<ClientCommandEvent>()
        .add_systems(Update, deal_with_connections)
        .add_systems(Update, split_client_events)
        .add_systems(Update, send_networked_events.after(split_client_events).after(deal_with_connections))
        .add_systems(Update, recieve_networked_events.after(deal_with_connections));
  }
}
fn open_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/adamantvtt";

    let socket: MatchboxSocket<MultipleChannels> = WebRtcSocketBuilder::new(room_url)
        .add_channel(ChannelConfig::reliable())
        .add_channel(ChannelConfig::unreliable())
        .into();

    commands.insert_resource(socket);
}

type ValidationHash = u8;

#[derive(Resource)]
struct TokenHashes {
    hashes: std::collections::HashMap<tokens::TokenID, ValidationHash>
}

fn init_hashtable(mut commands: Commands) {
    let hash: TokenHashes = TokenHashes{
        hashes: std::collections::HashMap::<tokens::TokenID, ValidationHash>::new(),
    };
    commands.insert_resource(hash);
}

fn deal_with_connections(mut connection: ResMut<MatchboxSocket<MultipleChannels>>){
    let updated_peers = match connection.try_update_peers() {
        Err(_x) => panic!("TODO DEAL WITH DISCONNECTED"),
        Ok(x) => x,
    };
    for (peer_id, peer_state) in updated_peers {
        let peer_state = match peer_state {
            PeerState::Connected => "Connected",
            PeerState::Disconnected => "Disconnected",
        };
        println!("{peer_id} : {peer_state}");
    }
}

enum NetworkReliability {
    Reliable,
    Unreliable,
}

#[derive(Event)]
struct NetworkedCommandEvent{
    pub command: order::Command,
    pub reliability: NetworkReliability,
    pub validation: ValidationHash,
}

#[derive(Event)]
struct ClientCommandEvent{
    pub command: order::Command,
}

//Split the events from the client into events to be networked 
//and events to be played on the local machine
fn split_client_events(
    mut ev_client: EventReader<ClientCommandEvent>,
    mut ev_networked: EventWriter<NetworkedCommandEvent>,
    mut ev_order: EventWriter<orders::OrderEvent>,
) {
    for ev in ev_client.iter() {
        ev_networked.send(NetworkedCommandEvent{
            command: ev.command,
            reliability: NetworkReliability::Reliable,
            validation: 0,
        });
        ev_order.send(orders::OrderEvent {
            command: ev.command
        });
    }
}

fn send_networked_events(
    mut ev_networked: EventReader<NetworkedCommandEvent>,
    mut connection: ResMut<MatchboxSocket<MultipleChannels>>,
) {
    for ev in ev_networked.iter() {
        let ids = Vec::from_iter(connection.connected_peers());
        for peer_id in ids {
            let arr = bytes_of(ev);
            let channel = match ev.reliability {
                NetworkReliability::Reliable => 0,
                NetworkReliability::Unreliable => 1,
            };
            connection.get_channel(channel).unwrap().send(Box::new(arr), peer_id);
        }
    }
}

fn recieve_networked_events(
    mut connection: ResMut<MatchboxSocket<MultipleChannels>>,
    mut ev_order: EventWriter<orders::OrderEvent>,
) {
    //Reliable
    let recieved = connection.get_channel(0).unwrap().receive();
    for (peer_id, packet) in recieved {
        println!("Recieved from: {peer_id}");
    }
    //Unreliable
    let recieved = connection.get_channel(1).unwrap().receive();
    for (peer_id, packet) in recieved {
        println!("Recieved from: {peer_id}");
    }
}
