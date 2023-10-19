use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::tokens;
use crate::maps;
use crate::baseplate;

pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<OrderEvent>()
        .add_systems(Update, recieve_orders)
        .add_event::<MoveCommand>()
        .add_systems(Update, recieve_move.after(recieve_orders))
        .add_event::<CreateTokenCommand>()
        .add_systems(Update, recieve_create_token.after(recieve_orders))
        .add_event::<CreateMapCommand>()
        .add_systems(Update, recieve_create_map.after(recieve_orders));
  }
}
#[derive(Event, Serialize, Deserialize, Clone)]
pub struct OrderEvent {
  pub command: Command,
} 

#[derive(Serialize, Deserialize, Clone)]
pub enum Command {
  Move(MoveCommand),
  CreateToken(CreateTokenCommand),
  CreateMap(CreateMapCommand),
}

pub fn recieve_orders(
  mut ev_orders: EventReader<OrderEvent>,
  mut ev_move: EventWriter<MoveCommand>,
  mut ev_create_token: EventWriter<CreateTokenCommand>,
  mut ev_create_map: EventWriter<CreateMapCommand>,
) {
  for ord_ev in ev_orders.iter() {
    match &ord_ev.command {
      Command::Move(cmd) => ev_move.send(*cmd),
      Command::CreateToken(cmd) => ev_create_token.send(cmd.clone()),
      Command::CreateMap(cmd) => ev_create_map.send(cmd.clone())
    }
  }
}

#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct MoveCommand {
  pub x: f32,
  pub y: f32,
  pub id: baseplate::ID,
}

fn recieve_move(
  mut ev_move: EventReader<MoveCommand>,
  mut tokens: Query<(&baseplate::ID, &mut Transform)>
) {
  for mov_ev in ev_move.iter() {
    for mut token in tokens.iter_mut() {
      if token.0.0 == mov_ev.id.0 {
        token.1.translation.x = mov_ev.x;
        token.1.translation.z = mov_ev.y;
      }
    }
  }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct CreateTokenCommand {
  pub x: f32,
  pub y: f32,
  pub id: baseplate::ID,
  pub url: String,
}

fn recieve_create_token(
  mut ev_create_token: EventReader<CreateTokenCommand>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  for ev in ev_create_token.iter() {
    commands.spawn(tokens::TokenBundle::new(ev.id, Vec3::new(ev.x, 0.1, ev.y), "https://api.open5e.com/static/img/monsters/ankheg.png".to_string(),&mut meshes, &mut materials, &asset_server));
  }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct CreateMapCommand {
  pub x: f32,
  pub y: f32,
  pub id: baseplate::ID,
  pub url: String,
}

fn recieve_create_map(
  mut ev_create_map: EventReader<CreateMapCommand>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  for ev in ev_create_map.iter() {
    commands.spawn(maps::MapBundle::new(ev.id, Vec3::new(ev.x, 0., ev.y), ev.url.clone(), &mut meshes, &mut materials, &asset_server));
  }
}
