use crate::baseplate;
use crate::input;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Bundle)]
pub struct TokenBundle {
    #[bundle()]
    pub base: baseplate::BaseplateBundle,
    #[bundle()]
    pub pickable: PickableBundle,
    #[bundle()]
    pub target: RaycastPickTarget,
    #[bundle()]
    pub drag_event: On<Pointer<Drag>>,
    pub token: TokenFlag,
}

#[derive(Component)]
pub struct TokenFlag;

impl TokenBundle {
    pub fn new(
        id: baseplate::ID,
        position: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> TokenBundle {
        TokenBundle {
            base: baseplate::BaseplateBundle::new(
                id,
                position,
                Vec2::new(5., 5.),
                meshes,
                materials,
                asset_server,
            ),
            pickable: PickableBundle::default(), // Makes the entity pickable
            target: RaycastPickTarget::default(), // Marker for the `bevy_picking_raycast` backend
            drag_event: On::<Pointer<Drag>>::send_event::<input::TokenDragEvent>(),
            token: TokenFlag,
        }
    }
}
