use bevy::prelude::*;
use bevy_web_asset::

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
  }
}

fn setup(mut commands: Commands) {
  let camera_bundle = Camera3dBundle::default();
  commands.spawn(camera_bundle);
}
