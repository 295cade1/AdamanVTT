use bevy::{
    prelude::*,
    window::WindowPlugin,
    //winit::WinitSettings,
};
use bevy_mod_picking::prelude::*;

//All modules
mod camera;
mod filetransfer;
mod input;
mod maps;
mod networking;
mod orders;
mod startup;
mod tokens;
mod ui;
mod bank;
mod fileload;
mod dd2vtt;
mod files;
mod encounters;

fn main() {
    App::new()
        // Power-saving reactive rendering for applications.
        //.insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bank::BankPlugin)
        .add_plugins(filetransfer::FileTransfer)
        .add_plugins(files::FilesPlugin)
        .add_plugins(fileload::FileLoad)
        .add_plugins(input::InputPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(ui::UIPlugin)
        .add_plugins(startup::GameStartPlugin)
        .add_plugins(networking::NetworkingPlugin)
        .add_plugins(orders::OrdersPlugin)
        .add_plugins(maps::MapPlugin)
        .add_plugins(encounters::EncounterPlugin)
        .run();
}
