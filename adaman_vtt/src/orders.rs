use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::tokens;

pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<OrderEvent>()
        .add_systems(Update, recieve_orders);
  }
}
#[derive(Event, Serialize, Deserialize)]
pub struct OrderEvent {
  pub command: Command,
  pub token_id: tokens::TokenID
} 

#[derive(Serialize, Deserialize)]
pub enum Command {
  ModifyToken(tokens::TokenBundle)
}

pub fn recieve_orders(
  mut ev_orders: EventReader<OrderEvent>,
  mut cmds: Commands,
) {
  let mut relevant_ids = std::collections::hash_set::HashSet::<tokens::TokenID>::new();
  for ord in ev_orders.iter(){
    relevant_ids.insert(ord.token_id);
  }
}
