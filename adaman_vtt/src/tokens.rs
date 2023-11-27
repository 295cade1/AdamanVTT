use crate::input;
use crate::fileload;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::sync::Arc;

pub struct TokenPlugin;

impl Plugin for TokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TokenLoad>()
            .add_systems(Update, load_token);
    }
}
#[derive(Serialize, Deserialize, Clone, Copy, Component, Eq, Hash, PartialEq)]
pub struct TokenId(pub uuid::Uuid);

pub fn get_new_id() -> TokenId {
    TokenId(Uuid::new_v4())
}

#[derive(Bundle)]
pub struct TokenBundle {
    pub id: TokenId,
    pub load_identifier: fileload::LoadIdentifier,
    #[bundle()]
    pub pbr: PbrBundle,
    #[bundle()]
    pub pickable: PickableBundle,
    #[bundle()]
    pub drag_event: On<Pointer<Drag>>,
    pub token: TokenFlag,
}

#[derive(Component)]
pub struct TokenFlag;

impl TokenBundle {
    pub fn new(
        id: TokenId,
        load_identifier: fileload::LoadIdentifier,
        position: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> TokenBundle {
        let bg_quad = shape::Quad {
            size: Vec2 {
                x: 5.,
                y: 5.,
            },
            flip: false,
        };

        TokenBundle {
            id,
            pbr: PbrBundle {
                mesh: meshes.add(bg_quad.into()),
                material: materials.add(StandardMaterial {
                    ..default()
                }),
                transform: Transform::from_xyz(position.x, position.y, position.z)
                    .looking_at(Vec3::new(position.x, -1., position.z), Vec3::Y),
                ..default()
            },
            pickable: PickableBundle::default(), // Makes the entity pickable
            drag_event: On::<Pointer<Drag>>::send_event::<input::TokenDragEvent>(),
            token: TokenFlag,
            load_identifier,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    pub format: f32,
    pub name: String,
    pub size: String,
    pub type_field: String,
    pub hit_points: i64,
    pub armor_class: i64,
    pub img: Option<String>,
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct TokenLoad {
    pub token_id: TokenId,
    pub data: Arc<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Component, Eq, Hash, PartialEq)]
pub struct TokenLoaded;

pub fn load_token(
    mut commands: Commands,
    mut ev_token_load: EventReader<TokenLoad>,
    mut tokens: Query<(&Handle<Mesh>, &Handle<StandardMaterial>, Entity, &TokenId, Without<TokenLoaded>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_token_load.read() {
        //Deserialize the token data
        let Some(data) = serde_json::from_slice::<TokenData>(ev.data.as_slice()).ok() else {
            println!("Bad Token Data");
            continue;
        };
        //Deserialize the image data
        //let image_data = ImageReader::new(
            //Cursor::new(data.get_image().clone()))
            //.with_guessed_format()
            //.expect("Unable to guess format")
            //.decode()
            //.expect("Malformed Image");
        //Get the image in bevy's format
        //let bevy_image = Image::from_dynamic(image_data, true);
        //Insert it into the images pool
        //let image_handle = images.add(bevy_image);

        for token in tokens.iter_mut() {
            //Check if the id matches
            if *token.3 == ev.token_id {

                commands.entity(token.2).insert(TokenLoaded);

                let Some(mat) = materials.get_mut(token.1) else {
                    println!("Failed to get mat");
                    continue;
                };
                //Replace the material's image with the new one
                //mat.base_color_texture = Some(image_handle.clone());

                let radius = 1.;
                println!("{radius}");
                //Create a new mesh of the correct size
                let new_quad = shape::Circle {
                    radius,
                    vertices: 64,
                };
                meshes.insert(token.0, new_quad.into());

                //Workaround to recalculate AABBs
                commands.entity(token.2).remove::<bevy::render::primitives::Aabb>();
            }
        }
    }
}
