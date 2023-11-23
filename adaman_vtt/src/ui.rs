use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::input;
use crate::networking;
use crate::files;
use crate::bank;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, init_ui)
            .add_systems(Update, update_ui_state)
            .add_systems(Update, ui);
    }
}

fn init_ui(mut commands: Commands) {
    let ui_state = UIState {
        right_sidepanel_state: SidePanelState::Maps,
        map_name: "".to_string(),
        map_list: None,
    };
    commands.insert_resource(ui_state);
}

#[derive(Resource)]
struct UIState {
    pub right_sidepanel_state: SidePanelState,
    pub map_name: String,
    pub map_list: Option<files::MapList>,
}

#[derive(PartialEq, Eq)]
enum SidePanelState {
    Maps,
    Tokens,
    Encounters,
}

fn update_ui_state(
    mut ui_state: ResMut<UIState>,
    mut map_event: EventReader<files::MapListUpdated>,
    bank: ResMut<bank::Bank>,
) {
    for _ev in map_event.read() {
        ui_state.map_list = None;
    }
    if ui_state.map_list.is_none() {
        ui_state.map_list = Some(bank.get_map_list());
    }
}

fn ui(
    commands: Commands,
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
) {
    egui::SidePanel::right("Token Creation")
        .min_width(200.0)
        .max_width(300.0)
        .show(contexts.ctx_mut(), |ui| {
            match ui_state.right_sidepanel_state {
                SidePanelState::Maps => {
                    let create_map_file_btn = ui.button("Import Map");
                    ui.text_edit_singleline(&mut ui_state.map_name);
                    if create_map_file_btn.clicked() {
                        input::create_map_from_file(commands, ui_state.map_name.clone());
                    }
                    
                    if let Some(ref map_list) = &ui_state.map_list {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for map in map_list.maps.iter() {
                                ui.separator();
                                ui.label(map.name.clone());
                                let insert_btn = ui.button("Insert");
                                if insert_btn.clicked() {
                                    input::create_map(map.load_identifier.clone(), &mut ev_client);
                                }
                            }
                        });
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
