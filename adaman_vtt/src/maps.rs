use bevy::prelude::*;
use crate::baseplate;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, modify_map_sizes);
  }
}

#[derive(Bundle)]
pub struct MapBundle {
  #[bundle()]
  pub base: baseplate::BaseplateBundle,
  pub map: MapFlag,
} 

#[derive(Component)]
pub struct MapFlag;

impl MapBundle {
  pub fn new(
    id: baseplate::ID,
    position: Vec3,
    url: String,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) -> MapBundle
  {
    MapBundle {
      base: baseplate::BaseplateBundle::new(id, position, Vec2::new(1., 1.), url, meshes, materials, asset_server),
      map: MapFlag,
    }
  }
}

pub fn modify_map_sizes(
  mut commands: Commands,
  mut ev_asset: EventReader<AssetEvent<Image>>,
  assets: ResMut<Assets<Image>>,
  mut maps: Query<(&Handle<Mesh>, &Handle<StandardMaterial>, Entity, With<MapFlag>)>,
  mut meshes: ResMut<Assets<Mesh>>,
  materials: Res<Assets<StandardMaterial>>,
)
{
  for ev in ev_asset.iter() {
    if let AssetEvent::Created{handle} = ev {
      for map in maps.iter_mut() {
        if let Some(mat) = materials.get(map.1) {
          if let Some(tex) = &mat.base_color_texture {
            if tex == handle {
              if let Some(img) = assets.get(handle) {
                let bg_quad = shape::Quad {
                    size: Vec2{x: img.size().x / 25., y: img.size().y / 25.},
                    flip: false,
                };
                let _ = meshes.set(map.0, bg_quad.into());
                //A workaround to manually recalculate the AABB's for the mesh
                commands.entity(map.2).remove::<bevy::render::primitives::Aabb>();
              }
            }
          }
        }
      }
    }
  }
}

