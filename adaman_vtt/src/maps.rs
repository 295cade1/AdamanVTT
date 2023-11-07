use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use std::sync::Arc;
use image::io::Reader as ImageReader;
use std::io::Cursor;

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapData {
    pub format: f64,
    pub image: Vec<u8>,
    pub grid: MapGrid,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapGrid {
    pub pixels_per: i64,
    pub width: i64,
    pub height: i64,
}

#[allow(clippy::type_complexity)]
pub fn load_map(
    mut commands: Commands,
    mut ev_map_load: EventReader<MapLoad>,
    mut maps: Query<(&Handle<Mesh>, &Handle<StandardMaterial>, Entity, &MapId)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_map_load.iter() {
        //Deserialize the map data
        let Some(data) = serde_json::from_slice::<MapData>(ev.data.as_slice()).ok() else {
            println!("Bad Map Data");
            continue;
        };
        //Deserialize the image data
        let image_data = ImageReader::new(
            Cursor::new(data.image.clone()))
            .with_guessed_format()
            .expect("Unable to guess format")
            .decode()
            .expect("Malformed Image");
        //Get the image in bevy's format
        let bevy_image = Image::from_dynamic(image_data, true);
        //Insert it into the images pool
        let image_handle = images.add(bevy_image);

        for map in maps.iter_mut() {
            //Check if the id matches
            if *map.3 == ev.map_id {
                let Some(mat) = materials.get_mut(map.1) else {
                    println!("Failed to get mat");
                    continue;
                };
                //Replace the material's image with the new one
                mat.base_color_texture = Some(image_handle.clone());

                let width = data.grid.width as f32 * 1.;
                let height = data.grid.height as f32 * 1.;
                println!("{width}, {height}");
                //Create a new mesh of the correct size
                let new_quad = shape::Quad {
                    size: Vec2 {
                        x: width,
                        y: height,
                    },
                    flip: false,
                };
                let _ = meshes.set(map.0, new_quad.into());

                //Workaround to recalculate AABBs
                commands.entity(map.2).remove::<bevy::render::primitives::Aabb>();
            }
        }
    }
}
