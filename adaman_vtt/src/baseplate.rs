use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Copy, Component, Eq, Hash, PartialEq)]
pub struct ID(pub uuid::Uuid);

pub fn get_new_id() -> Uuid{
  Uuid::new_v4()
}

#[derive(Bundle)]
pub struct BaseplateBundle {
  pub id: ID,
  #[bundle()]
  pub pbr: PbrBundle,
} 

impl BaseplateBundle {
  pub fn new(
    id: ID,
    position: Vec3,
    size: Vec2,
    url: String,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) -> BaseplateBundle
  {
    let tex = Some(asset_server.load(url));

    let bg_quad = shape::Quad {
        size: Vec2{x: size.x, y: size.y},
        flip: false,
    };

    BaseplateBundle{
      id,
      pbr: PbrBundle {
          mesh: meshes.add(bg_quad.into()),
          material: materials.add(StandardMaterial{
              base_color_texture: tex,
            ..default()
          }),
          transform: Transform::from_xyz(position.x, position.y, position.z).looking_at(Vec3::new(position.x, -1., position.z), Vec3::Y),
          ..default()
      }

    }
  }
}