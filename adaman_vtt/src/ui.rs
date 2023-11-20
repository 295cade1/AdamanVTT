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
    let ui_state = UIState {
        right_sidepanel_state: SidePanelState::Maps,
    };
    commands.insert_resource(ui_state);
}

#[derive(Resource)]
struct UIState {
    pub right_sidepanel_state: SidePanelState,
}

#[derive(PartialEq, Eq)]
enum SidePanelState {
    Maps,
    Tokens,
    Encounters,
}

fn ui(
    commands: Commands,
    mut contexts: EguiContexts,
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
    mut ui_state: ResMut<UIState>,
) {
    egui::SidePanel::right("Token Creation")
    .min_width(200.0)
    .max_width(300.0)
    .show(contexts.ctx_mut(), |ui| {

        match ui_state.right_sidepanel_state {
            SidePanelState::Maps => {
                let create_map_file_btn = ui.button("Import Map");
                if create_map_file_btn.clicked() {
                    input::create_map_from_file(commands);
                }
            }
            SidePanelState::Tokens => {
                let create_token_btn = ui.button("Import Token");
                if create_token_btn.clicked() {
                    ev_client.send(input::create_token(0., 0., None))
                }
            }
            SidePanelState::Encounters => {}
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut ui_state.right_sidepanel_state, SidePanelState::Maps, "Maps");
                ui.selectable_value(&mut ui_state.right_sidepanel_state, SidePanelState::Tokens, "Tokens");
                ui.selectable_value(&mut ui_state.right_sidepanel_state, SidePanelState::Encounters, "Encounters");
            })
        })
    });
}
