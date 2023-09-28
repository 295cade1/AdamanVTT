use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use bevy_mod_picking::prelude::*;

use crate::tokens;
use crate::input;

pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<OrderEvent>()
        .add_systems(Update, recieve_orders)
        .add_event::<MoveCommand>()
        .add_systems(Update, recieve_move.after(recieve_orders))
        .add_event::<CreateTokenCommand>()
        .add_systems(Update, recieve_create_token.after(recieve_orders));
  }
}
#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct OrderEvent {
  pub command: Command,
} 

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Command {
  Move(MoveCommand),
  CreateToken(CreateTokenCommand),
}

pub fn recieve_orders(
  mut ev_orders: EventReader<OrderEvent>,
  mut ev_move: EventWriter<MoveCommand>,
  mut ev_create_token: EventWriter<CreateTokenCommand>,
) {
  for ord_ev in ev_orders.iter() {
    match ord_ev.command {
      Command::Move(cmd) => ev_move.send(cmd),
      Command::CreateToken(cmd) => ev_create_token.send(cmd),
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
  for mov_ev in ev_move.iter() {
    for mut token in tokens.iter_mut() {
      if token.0.0 == mov_ev.id.0 {
        token.1.translation.x = mov_ev.x;
        token.1.translation.z = mov_ev.y;
      }
    }
  }
}

#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct CreateTokenCommand {
  pub x: f32,
  pub y: f32,
  pub id: tokens::TokenID,
}

fn recieve_create_token(
  mut ev_create_token: EventReader<CreateTokenCommand>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for ev in ev_create_token.iter() {
    let token_quad = shape::Quad {
        size: Vec2{x: 5.,y: 5.},
        flip: false,
    };

    let new_token = (tokens::TokenBundle {
        pbr: PbrBundle {
            mesh: meshes.add(token_quad.into()),
            material: materials.add(StandardMaterial{
                base_color: Color::BLUE,
              ..default()
            }),
            transform: Transform::from_xyz(ev.x, 0.1, ev.y).looking_at(Vec3::new(ev.x, -1., ev.y), Vec3::Y),
            ..default()
        },
        token_id: ev.id,
    }, 
    PickableBundle::default(),      // Makes the entity pickable
    RaycastPickTarget::default(),    // Marker for the `bevy_picking_raycast` backend
    On::<Pointer<Drag>>::send_event::<input::TokenDragEvent>(),
    );
    commands.spawn(new_token);
  }
}
