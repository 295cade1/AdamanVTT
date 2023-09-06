use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use bevy::log::info;

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Startup, open_socket)
        .add_systems(Update, deal_with_connections)
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
    let room_url = "ws://127.0.0.1:3536/adamantvtt?next=10";

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
    for peerID, peerState in updated_peers {
        unimplemented!();
    }
}
