use bevy::prelude::*;
use bevy_matchbox::prelude::*;

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Startup, open_socket)
        .add_systems(PreUpdate, deal_with_connections)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, recieve)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
  }
}

fn setup(mut commands: Commands,
            mut meshes: ResMut<Assets<Mesh>>,
            mut images: ResMut<Assets<Image>>,
            asset_server: Res<AssetServer>,
            mut materials: ResMut<Assets<StandardMaterial>>,
        ) {
    let camera_bundle = Camera3dBundle {
        transform: Transform::from_xyz(0., 100., 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    };
    commands.spawn(camera_bundle);

    let tex = Some(asset_server.load("https://i.pinimg.com/originals/27/2d/7e/272d7e20f512f3bc24713248ce626b5d.jpg"));

    let bg_quad = shape::Quad {
        size: Vec2{x: 50.,y: 50.},
        flip: false,
    };

    commands.spawn(PbrBundle {
        mesh: meshes.add(bg_quad.into()),
        material: materials.add(StandardMaterial{
            base_color_texture: tex,
          ..default()
        }),
        transform: Transform::from_xyz(0., 0., 0.).looking_at(Vec3::new(0., -1., 0.), Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
          point_light: PointLight {
              intensity: 9000.0,
              range: 100.,
              shadows_enabled: true,
              ..default()
          },
          transform: Transform::from_xyz(8.0, 16.0, 8.0),
          ..default()
      });
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

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut connection: ResMut<MatchboxSocket<MultipleChannels>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let ids = Vec::from_iter(connection.connected_peers());
        for peer_id in ids {
            let arr: [u8; 1] = [5];
            connection.get_channel(0).unwrap().send(Box::new(arr), peer_id);
        }
    }
}

fn recieve(
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
