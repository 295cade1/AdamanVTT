use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::input;
use crate::networking;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, init_ui)
            .add_systems(Update, ui);
    }
}

fn init_ui(mut commands: Commands) {
    let ui_state = UIState {};
    commands.insert_resource(ui_state);
}

#[derive(Resource)]
struct UIState {

}

fn ui(
    commands: Commands,
    mut contexts: EguiContexts,
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
    mut _ui_state: ResMut<UIState>,
) {
    egui::SidePanel::right("Token Creation")
        .default_width(200.0)
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            let create_token_btn = ui.button("Create Token");
            if create_token_btn.clicked() {
                ev_client.send(input::create_token(0., 0., None))
            }

            let create_map_file_btn = ui.button("Create Map from file");
            if create_map_file_btn.clicked() {
                input::create_map_from_file(commands);
            }
        });
}
