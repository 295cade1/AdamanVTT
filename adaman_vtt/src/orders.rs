use bevy::prelude::*;

pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<OrderEvent>()
  }
}
#[derive(Event)]
pub struct OrderEvent {
  pub command: Command,
} 

struct Position(f32, f32);

pub enum Command {
    Move(Position),
}

