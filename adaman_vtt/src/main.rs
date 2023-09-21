use bevy::prelude::*;
use bevy_remote_asset::RemoteAssetPlugin;

//All modules
mod startup;
mod networking;
mod orders;
mod tokens;

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
      .add_plugins(orders::OrdersPlugin)
      .add_systems(Update, test_input)
      .run();
}

fn test_input(
  keys: Res<Input<KeyCode>>,
  mut ev_client: EventWriter<networking::ClientCommandEvent>,
) {
  if keys.just_pressed(KeyCode::Numpad1) {
    ev_client.send(networking::ClientCommandEvent{
      order: orders::OrderEvent{
        command: orders::Command::Move(orders::MoveCommand{
          x: 1.,
          y: 1.,
          id: tokens::TokenID(5),
        })
      },
      reliability: networking::NetworkReliability::Reliable,
    })
  }
  if keys.just_pressed(KeyCode::Numpad2) {
    ev_client.send(networking::ClientCommandEvent{
      order: orders::OrderEvent{
        command: orders::Command::Move(orders::MoveCommand{
          x: 5.,
          y: 5.,
          id: tokens::TokenID(5),
        })
      },
      reliability: networking::NetworkReliability::Reliable,
    })
  }
}
