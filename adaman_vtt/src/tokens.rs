use crate::input;
use crate::fileload;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
