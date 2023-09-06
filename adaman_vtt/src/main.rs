
use bevy::prelude::*;
use bevy_remote_asset::RemoteAssetPlugin;
//use bevy_matchbox::prelude::*;

mod startup;

fn main() {
  App::new() 
      .add_plugins(RemoteAssetPlugin)
      .add_plugins(
        DefaultPlugins.set(WindowPlugin {
          primary_window: Some(Window {
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
          ..default()
        })
      )
      .add_plugins(startup::GameStartPlugin)
      .run();
}
