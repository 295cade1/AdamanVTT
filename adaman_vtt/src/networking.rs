use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use bytemuck::*;

use crate::orders;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, open_socket)
        .add_systems(PreUpdate, deal_with_connections)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, recieve_networked_events)
        .add_systems(Update, send_networked_events);
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


struct ValidationHash(u8);

enum NetworkReliability {
    Reliable(ValidationHash),
    Unreliable(ValidationHash),
}

#[derive(Event)]
struct NetworkedEvent {
    pub command: orders::Order,
    pub reliability: NetworkReliability,
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut ev_networked: EventWriter<NetworkedEvent>,
) {

}

fn send_networked_events(
    mut ev_networked: EventReader<NetworkedEvent>,
    mut connection: ResMut<MatchboxSocket<MultipleChannels>>,
) {
    for ev in ev_networked.iter() {
        let ids = Vec::from_iter(connection.connected_peers());
        for peer_id in ids {
            let arr: [u8; 1] = [5];
            connection.get_channel(0).unwrap().send(Box::new(arr), peer_id);
        }
    }
}

fn recieve_networked_events(
    mut connection: ResMut<MatchboxSocket<MultipleChannels>>,
) {
    let recieved = connection.get_channel(0).unwrap().receive();
    for (peer_id, packet)  in recieved {
        let isfive = match packet.first() {
            None => false,
            Some(&x) => x == 5,
        };
        println!("Recieved from: {peer_id} : {isfive}")
    }
}
