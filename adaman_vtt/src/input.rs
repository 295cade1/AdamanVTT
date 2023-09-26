
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
  fn build(&self, app: &mut App) {
    app .add_systems(Update, test_input);
  }
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
    ev_client.send(
      networking::ClientCommandEvent{
        order: orders::OrderEvent{
          command: orders::Command::Move(orders::MoveCommand{
            x: 5.,
            y: 5.,
            id: tokens::TokenID(5),
          })
        },
        reliability: networking::NetworkReliability::Reliable,
      }
    )
  }
}
