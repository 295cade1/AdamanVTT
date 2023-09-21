use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::tokens;

pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<OrderEvent>()
        .add_systems(Update, recieve_orders)
        .add_event::<MoveCommand>()
        .add_systems(Update, recieve_move.after(recieve_orders));
  }
}
#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct OrderEvent {
  pub command: Command,
} 

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Command {
  Move(MoveCommand),
}

pub fn recieve_orders(
  mut ev_orders: EventReader<OrderEvent>,
  mut ev_move: EventWriter<MoveCommand>,
) {
  for ord_ev in ev_orders.iter() {
    match ord_ev.command {
      Command::Move(cmd) => ev_move.send(cmd),
    }
  }
}

#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct MoveCommand {
  pub x: f32,
  pub y: f32,
  pub id: tokens::TokenID,
}

fn recieve_move(
  mut ev_move: EventReader<MoveCommand>,
  mut tokens: Query<(&tokens::TokenID, &mut Transform)>
) {
  for mut mov_ev in ev_move.iter() {
    for mut token in tokens.iter_mut() {
      if token.0.0 == mov_ev.id.0 {
        token.1.translation.x = mov_ev.x;
        token.1.translation.z = mov_ev.y;
      }
    }
  }
}
