
pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app .add_systems(Update, moving_token_input);
  }
}

fn moving_token_input(
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
}
