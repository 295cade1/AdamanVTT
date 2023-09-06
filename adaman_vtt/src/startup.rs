use bevy::prelude::*;

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup)
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
