use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_stdvec};

use crate::orders;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, open_socket)
        .add_event::<NetworkedCommandEvent>()
        .add_event::<ClientCommandEvent>()
        .add_systems(Update, deal_with_connections)
        .add_systems(Update, split_client_events.before(orders::recieve_orders))
        .add_systems(Update, send_networked_events.after(split_client_events).after(deal_with_connections))
        .add_systems(Update, recieve_networked_events.after(deal_with_connections).before(orders::recieve_orders));
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

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum NetworkReliability {
    Reliable,
    Unreliable,
}

#[derive(Event, Serialize, Deserialize)]
struct NetworkedCommandEvent{
    pub order: orders::OrderEvent,
    pub reliability: NetworkReliability,
}

#[derive(Serialize, Deserialize)]
struct NetworkPacket{
    pub order: orders::OrderEvent,
}

#[derive(Event)]
pub struct ClientCommandEvent{
    pub order: orders::OrderEvent,
    pub reliability: NetworkReliability,
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
            order: ev.order,
            reliability: ev.reliability,
        });
        ev_order.send(ev.order);
    }
}

fn send_networked_events(
    mut ev_networked: EventReader<NetworkedCommandEvent>,
    mut connection: ResMut<MatchboxSocket<MultipleChannels>>,
) {
    for ev in ev_networked.iter() {
        let ids = Vec::from_iter(connection.connected_peers());
        for peer_id in ids {
            let packet = NetworkPacket{
                order: ev.order,
            };
            let arr = to_stdvec(&packet).unwrap().into_boxed_slice();
            let channel = match ev.reliability {
                NetworkReliability::Reliable => 0,
                NetworkReliability::Unreliable => 1,
            };
            connection.get_channel(channel).unwrap().send(arr, peer_id);
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
        let remote_order = from_bytes::<NetworkPacket>(&packet).unwrap();
        ev_order.send(remote_order.order);
        println!("Recieved from: {peer_id}");
    }
    //Unreliable
    let recieved = connection.get_channel(1).unwrap().receive();
    for (peer_id, packet) in recieved {
        let remote_order = from_bytes::<NetworkPacket>(&packet).unwrap();
        ev_order.send(remote_order.order);
        println!("Recieved from: {peer_id}");
    }
}
