use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    }
}

fn setup(mut commands: Commands) {
    let camera_bundle = (
        Camera3dBundle {
            transform: Transform::from_xyz(0., 100., 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            projection: PerspectiveProjection { ..default() }.into(),
            ..default()
        },
        RaycastPickCamera::default(),
    );
    commands.spawn(camera_bundle);

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
