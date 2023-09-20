use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Clone, Copy, Component, Eq, Hash, PartialEq)]
pub struct TokenID(u32);

//TODO deal with possible duplicate ID issues

#[derive(Bundle, Serialize, Deserialize)]
pub struct TokenBundle {
  token_id: TokenID,
  #[bundle()]
  pbr: PbrBundle,
}
