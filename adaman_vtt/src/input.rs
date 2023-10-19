use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::networking;
use crate::orders;
use crate::baseplate;

pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<TokenDragEvent>()
        .add_systems(Update, recieve_dragging_tokens.before(orders::recieve_orders));
  }
}

#[derive(Event)]
pub struct TokenDragEvent{
  pub input: ListenerInput<Pointer<Drag>>
}

impl From<ListenerInput<Pointer<Drag>>> for TokenDragEvent {
  fn from(input: ListenerInput<Pointer<Drag>>) -> TokenDragEvent{
    TokenDragEvent{input}
  }
}

fn recieve_dragging_tokens(
  mut ev_drag : EventReader<TokenDragEvent>,
  mut ev_client : EventWriter<networking::ClientCommandEvent>,
  tokens : Query<(&baseplate::ID, &Transform)>,
  // query to get camera transform
  camera_q: Query<(&Camera, &GlobalTransform)>,
) {

  let (camera, camera_transform) = camera_q.single();

  let mut dict = std::collections::HashMap::<baseplate::ID, (f32, f32)>::new();
  for drag_ev in ev_drag.iter() {
    if let Ok(token) = tokens.get(drag_ev.input.listener()) {
      dict.insert(*token.0, (drag_ev.input.pointer_location.position.x, drag_ev.input.pointer_location.position.y));
    }
  }

  for token in dict.iter() {
    if let Some(new_pos) = get_plane_intersection(camera.viewport_to_world(camera_transform, Vec2::new(token.1.0, token.1.1)).unwrap(), 
      Vec3::new(0.,0.,0.), 
      Vec3::new(0.,1.,0.)) {
      ev_client.send(networking::ClientCommandEvent{
        order: orders::OrderEvent{
          command: orders::Command::Move(orders::MoveCommand{
            id: *token.0,
            x: new_pos.x,
            y: new_pos.z,
          }),
        },
        reliability: networking::NetworkReliability::Reliable,
      })
    }
  }
}

fn get_plane_intersection(ray: Ray, plane_origin: Vec3, plane_normal: Vec3) -> Option<Vec3> {
    let ray_dir = ray.direction;
    let dot = plane_normal.dot(ray_dir);
    if dot.abs() > 1e-6 {
        //# The factor of the point between p0 -> p1 (0 - 1)
        //# if 'fac' is between (0 - 1) the point intersects with the segment.
        //# Otherwise:
        //#  < 0.0: behind p0.
        //#  > 1.0: infront of p1.
        let w = ray.origin - plane_origin;
        let fac = -plane_normal.dot(w) / dot;
        let u = ray_dir * fac;
        Some(ray.origin + u)
    } else {
    Option::None
  }
}

//UI funcs
pub fn create_token(x: f32, y: f32, url: Option<&str>) -> networking::ClientCommandEvent {
  let url = match url {
    Some(x) => x,
    None => "https://api.open5e.com/static/img/monsters/hezrou.png"
  };
  networking::ClientCommandEvent{
      order: orders::OrderEvent{
        command: orders::Command::CreateToken(orders::CreateTokenCommand{
          x,
          y,
          id: baseplate::ID(baseplate::get_new_id()),
          url: url.to_string(),
        })
      },
      reliability: networking::NetworkReliability::Reliable,
    }
}

pub fn create_map(x: f32, y: f32, url: Option<&str>) -> networking::ClientCommandEvent {
  let url = match url {
    Some(x) => x,
    None => &"https://api.open5e.com/static/img/monsters/hezrou.png"
  };
  networking::ClientCommandEvent{
      order: orders::OrderEvent{
        command: orders::Command::CreateMap(orders::CreateMapCommand{
          x,
          y,
          id: baseplate::ID(baseplate::get_new_id()),
          url: url.to_string(),
        })
      },
      reliability: networking::NetworkReliability::Reliable,
    }
}
