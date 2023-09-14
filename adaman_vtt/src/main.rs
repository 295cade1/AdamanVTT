use bevy::prelude::*;
use bevy_remote_asset::RemoteAssetPlugin;

//All modules
mod startup;
mod networking;
mod orders;

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
      .add_plugins(networking::NetworkingPlugin)
      .run();
}
