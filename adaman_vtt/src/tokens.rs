use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Component, Eq, Hash, PartialEq)]
pub struct TokenID(pub u32);

//TODO deal with possible duplicate ID issues

#[derive(Bundle)]
pub struct TokenBundle {
  pub token_id: TokenID,
  #[bundle()]
  pub pbr: PbrBundle,
}
