use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use std::sync::Arc;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MapLoad>()
            .add_systems(Update, load_map);
    }
}

#[derive(Bundle)]
pub struct MapBundle {
    pub id: MapId,
    #[bundle()]
    pub pbr: PbrBundle,
}

#[derive(Serialize, Deserialize, Clone, Copy, Component, Eq, Hash, PartialEq)]
pub struct MapId(pub uuid::Uuid);

pub fn get_new_id() -> MapId {
    MapId(Uuid::new_v4())
}

impl MapBundle {
    pub fn new(
        id: MapId,
        position: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> MapBundle {
        let bg_quad = shape::Quad {
            size: Vec2 {
                x: 10.,
                y: 10.,
            },
            flip: false,
        };

        MapBundle {
            pbr: PbrBundle {
                mesh: meshes.add(bg_quad.into()),
                material: materials.add(StandardMaterial {
                    ..default()
                }),
                transform: Transform::from_xyz(position.x, position.y, position.z)
                    .looking_at(Vec3::new(position.x, -1., position.z), Vec3::Y),
                ..default()
            },
            id,
        }
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct MapLoad {
    pub map_id: MapId,
    pub data: Arc<Vec<u8>>,
}

#[allow(clippy::type_complexity)]
pub fn load_map(
    mut commands: Commands,
    mut ev_map_load: EventReader<MapLoad>,
    assets: ResMut<Assets<Image>>,
    mut maps: Query<( &Handle<Mesh>, &Handle<StandardMaterial>, Entity, &MapId)>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    for ev in ev_map_load.iter() {
        let Some(data) = serde_json::from_slice::<MapData>(ev.data.as_slice()).ok() else {
            panic!("Bad Map Data");
        };

        for map in maps.iter_mut() {
            //Check if the id matches
            if *map.3 == ev.map_id {
                //Deserialize the data
            }

           // if let Some(mat) = materials.get(map.1) {
           //     if let Some(tex) = &mat.base_color_texture {
           //         if tex == handle {
           //             if let Some(img) = assets.get(handle) {
           //                 let bg_quad = shape::Quad {
           //                     size: Vec2 {
           //                         x: img.size().x / 25.,
           //                         y: img.size().y / 25.,
           //                     },
           //                     flip: false,
           //                 };
           //                 let _ = meshes.set(map.0, bg_quad.into());
           //                 //A workaround to manually recalculate the AABB's for the mesh
           //                 commands
           //                     .entity(map.2)
           //                     .remove::<bevy::render::primitives::Aabb>();
           //             }
           //         }
           //     }
           // }
        }
    }
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapData {
    pub format: f64,
    pub image: Vec<u8>,
}


