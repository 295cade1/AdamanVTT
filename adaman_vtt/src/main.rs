
use bevy::prelude::*;
//use bevy_web_asset::WebAssetPlugin;
//use bevy_matchbox::prelude::*;

mod startup;

fn main() {
  App::new()
      .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          fit_canvas_to_parent: true,
          prevent_default_event_handling: false,
          ..default()
      }),
        ..default()
      }))
      .add_plugins(startup::GameStartPlugin)
      .run();
}
