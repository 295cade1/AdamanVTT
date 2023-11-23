use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::Projection::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_movement);
    }
}

fn camera_movement(
    keys: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_q: Query<(&mut Transform, &mut Projection, With<Camera>)>,
) {
    let mut horizontal: f32 = 0.;
    let mut vertical: f32 = 0.;

    let mut zoom: f32 = 0.;

    //Keyboard Input
    if keys.pressed(KeyCode::Left) {
        horizontal -= 1.;
    }
    if keys.pressed(KeyCode::Right) {
        horizontal += 1.;
    }

    if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        if keys.pressed(KeyCode::Down) {
            zoom -= 1.;
        }
        if keys.pressed(KeyCode::Up) {
            zoom += 1.;
        }
    } else {
        if keys.pressed(KeyCode::Down) {
            vertical -= 1.;
        }
        if keys.pressed(KeyCode::Up) {
            vertical += 1.;
        }
    }

    //Scroll Input
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
                vertical += ev.y;
                horizontal -= ev.x;
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
                vertical += ev.y / 50.;
                horizontal -= ev.x / 50.;
            }
        }
    }

    let camera = camera_q.get_single_mut().ok().unwrap();
    let mut camera_transform = camera.0;
    camera_transform.translation.x -= vertical;
    camera_transform.translation.z -= horizontal;

    zoom = -zoom;

    match camera.1.into_inner() {
        Perspective(p) => {
            p.fov = (p.fov * (zoom / 10. + 1.)).clamp(10.0f32.to_radians(), 120.0f32.to_radians())
        }
        Orthographic(o) => o.scale = (o.scale * (zoom / 10. + 1.)).clamp(0.2, 5.),
    }
}
