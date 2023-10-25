use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

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

#[allow(clippy::type_complexity)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawMapData {
  pub format: f64,
  pub resolution: Resolution,
  #[serde(rename = "line_of_sight")]
  pub line_of_sight: Vec<Vec<LineOfSight>>,
  pub portals: Vec<Portal>,
  pub lights: Vec<Light>,
  pub environment: Environment,
  pub image: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
  #[serde(rename = "map_origin")]
  pub map_origin: MapOrigin,
  #[serde(rename = "map_size")]
  pub map_size: MapSize,
  #[serde(rename = "pixels_per_grid")]
  pub pixels_per_grid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapOrigin {
  pub x: i64,
  pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapSize {
  pub x: i64,
  pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineOfSight {
  pub x: f64,
  pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Portal {
  pub position: Position,
  pub bounds: Vec<Bound>,
  pub rotation: f64,
  pub closed: bool,
  pub freestanding: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
  pub x: f64,
  pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bound {
  pub x: f64,
  pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Light {
  pub position: Position2,
  pub range: f64,
  pub intensity: f64,
  pub color: String,
  pub shadows: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position2 {
  pub x: f64,
  pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
  #[serde(rename = "baked_lighting")]
  pub baked_lighting: bool,
  #[serde(rename = "ambient_light")]
  pub ambient_light: String,
}
