use bevy::prelude::*;


#[derive(Bundle)]
pub struct BaseplateBundle {
}

impl BaseplateBundle {
    pub fn new(
        id: ID,
        position: Vec3,
        size: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        _asset_server: &Res<AssetServer>,
    ) -> BaseplateBundle {

    }
}
