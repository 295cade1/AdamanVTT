use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, modify_map_sizes);
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

//#[allow(clippy::type_complexity)]
//pub fn modify_map_sizes(
//    mut commands: Commands,
//    mut ev_asset: EventReader<AssetEvent<Image>>,
//    assets: ResMut<Assets<Image>>,
//    mut maps: Query<(
//        &Handle<Mesh>,
//        &Handle<StandardMaterial>,
//        Entity,
//        With<MapFlag>,
//    )>,
//    mut meshes: ResMut<Assets<Mesh>>,
//    materials: Res<Assets<StandardMaterial>>,
//) {
//    for ev in ev_asset.iter() {
//        if let AssetEvent::Created { handle } = ev {
//            for map in maps.iter_mut() {
//                if let Some(mat) = materials.get(map.1) {
//                    if let Some(tex) = &mat.base_color_texture {
//                        if tex == handle {
//                            if let Some(img) = assets.get(handle) {
//                                let bg_quad = shape::Quad {
//                                    size: Vec2 {
//                                        x: img.size().x / 25.,
//                                        y: img.size().y / 25.,
//                                    },
//                                    flip: false,
//                                };
//                                let _ = meshes.set(map.0, bg_quad.into());
//                                //A workaround to manually recalculate the AABB's for the mesh
//                                commands
//                                    .entity(map.2)
//                                    .remove::<bevy::render::primitives::Aabb>();
//                            }
//                        }
//                    }
//                }
//            }
//        }
//    }
//}

pub struct LoadMap;

pub fn load_map() {
    
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapData {
    pub format: f64,
    pub image: Vec<u8>,
}

