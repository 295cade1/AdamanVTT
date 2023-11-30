use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::input;
use crate::networking;
use crate::files;
use crate::bank;
use crate::encounters;
use crate::open5e;

use std::collections::VecDeque;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, init_ui)
            .add_systems(Update, update_ui_state.before(ui))
            .add_systems(Update, ui)
            .add_systems(Update, update_log.before(display_log))
            .add_systems(Update, text_messages.after(ui))
            .add_systems(Update, display_log.after(text_messages))
            .insert_resource(Log{messages: Vec::new().into()})
            .add_event::<InsertLog>()
            .add_systems(Update, log_network)
            .insert_resource(TextMessages{messages: Vec::new().into(), input: "".to_string()})
            .add_event::<RecieveMessage>()
            .add_systems(Update, update_messages.before(text_messages))
        ;
    }
}

fn init_ui(mut commands: Commands) {
    let ui_state = UIState {
        popup_panel_state: PopupState::Closed,
        right_sidepanel_state: SidePanelState::Maps,
        map_name: "".to_string(),
        map_list: None,
        encounter_name: "".to_string(),
        encounter_list: None,
        token_name: "".to_string(),
        token_list: None,
    };
    commands.insert_resource(ui_state);
}

#[derive(Resource)]
struct UIState {
    pub right_sidepanel_state: SidePanelState,
    pub popup_panel_state: PopupState,
    pub map_name: String,
    pub map_list: Option<files::MapList>,
    pub encounter_name: String,
    pub encounter_list: Option<files::EncounterList>,
    pub token_name: String,
    pub token_list: Option<files::TokenList>,
}

#[derive(PartialEq, Eq)]
enum SidePanelState {
    Maps,
    Tokens,
    Encounters,
}

#[derive(PartialEq, Eq)]
enum PopupState {
    Closed,
    TokenCreation,
}

fn update_ui_state(
    mut ui_state: ResMut<UIState>,
    mut map_event: EventReader<files::MapListUpdated>,
    mut encounter_event: EventReader<files::EncounterListUpdated>,
    mut token_event: EventReader<files::TokenListUpdated>,
    bank: ResMut<bank::Bank>,
) {
    for _ev in map_event.read() {
        ui_state.map_list = None;
    }
    if ui_state.map_list.is_none() {
        ui_state.map_list = Some(bank.get_map_list());
    }
    for _ev in encounter_event.read() {
        ui_state.encounter_list = None;
    }
    if ui_state.encounter_list.is_none() {
        ui_state.encounter_list = Some(bank.get_encounter_list());
    }
    for _ev in token_event.read() {
        ui_state.token_list = None;
    }
    if ui_state.token_list.is_none() {
        ui_state.token_list = Some(bank.get_token_list());
    }
}

fn ui(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
    ev_save_encounter: EventWriter<encounters::EncounterSave>,
    mut ev_create_map: EventWriter<input::CreateMapFromFile>,
    mut ev_create_token: EventWriter<input::CreateTokenFromData>,
    mut connection: ResMut<open5e::Open5eMonsterSelection>,
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
                        ev_create_map.send(
                            input::CreateMapFromFile {
                                name: ui_state.map_name.clone().into(),
                            }
                        )
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
                    let create_token_btn = ui.button("Create From Open5e");
                    if create_token_btn.clicked() {
                        ui_state.popup_panel_state = PopupState::TokenCreation;
                    }
                    if let Some(ref token_list) = &ui_state.token_list {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for token in token_list.tokens.iter() {
                                ui.separator();
                                ui.label(token.name.clone());
                                let insert_btn = ui.button("Load");
                                if insert_btn.clicked() {
                                    input::create_token(token.load_identifier.clone(), &mut ev_client);
                                }
                            }
                        });
                    }
                }
                SidePanelState::Encounters => {
                    let save_encounter_btn = ui.button("Save Current");
                    ui.text_edit_singleline(&mut ui_state.encounter_name);
                    if save_encounter_btn.clicked() {
                        input::save_encounter(ev_save_encounter, ui_state.encounter_name.clone());
                    }
                    if let Some(ref encounter_list) = &ui_state.encounter_list {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for encounter in encounter_list.encounters.iter() {
                                ui.separator();
                                ui.label(encounter.name.clone());
                                let insert_btn = ui.button("Load");
                                if insert_btn.clicked() {
                                    input::load_encounter(encounter.load_identifier.clone(), &mut ev_client);
                                }
                            }
                        });
                    }
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut ui_state.right_sidepanel_state, SidePanelState::Maps, "Maps");
                    ui.selectable_value(&mut ui_state.right_sidepanel_state, SidePanelState::Tokens, "Tokens");
                    ui.selectable_value(&mut ui_state.right_sidepanel_state, SidePanelState::Encounters, "Encounters");
                })
            })
        });
    match ui_state.popup_panel_state {
        PopupState::Closed => {},
        PopupState::TokenCreation => {
            let mut open = true;
            egui::Window::new("Import From Open5e")
                .open(&mut open)
                .collapsible(false)
                .fixed_size(egui::vec2(500., 600.))
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(contexts.ctx_mut(), |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if let Some(list) = connection.get_list() {
                            for item in list.iter() {
                                ui.label(&item.name);
                                ui.label(item.hit_points.to_string());
                                let insert_btn = ui.button("Insert");
                                if insert_btn.clicked() {
                                    ev_create_token.send(
                                        input::CreateTokenFromData{
                                            data: item.clone(),
                                        }
                                    )
                                }
                                ui.separator();
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        let prev = ui.button("Prev");
                        let next = ui.button("Next");
                    });
                })
            ;
            if !open {
                ui_state.popup_panel_state = PopupState::Closed;
            }
        },
    }
}

#[derive(Resource)]
struct Log {
    messages: VecDeque<Message>,
}

struct Message {
    text: String,
    alive: f32,
}

#[derive(Event)]
pub struct InsertLog {
    text: String,
}

impl InsertLog {
    pub fn new(text: String) -> InsertLog {
        InsertLog{
            text,
        }
    }
}

const MESSAGE_SHOW_TIME: f32 = 10.;

fn log_network(
    mut events: EventWriter<InsertLog>,
    mut ev_connected: EventReader<networking::PeerConnected>,
    mut ev_disconnected: EventReader<networking::PeerDisconnected>,
) {
    for ev in ev_connected.read() {
        events.send(
            InsertLog{
                text: "Connected Peer ".to_string() + &ev.0.to_string(),
            }
        )
    }
    for ev in ev_disconnected.read() {
        events.send(
            InsertLog{
                text: "Disconnected Peer ".to_string() + &ev.0.to_string(),
            }
        )
    }
}

fn update_log(
    mut events: EventReader<InsertLog>,
    mut log: ResMut<Log>,
    time: Res<Time>,
) {
    for ev in events.read() {
        log.messages.push_back(
            Message{
                text: ev.text.clone(),
                alive: MESSAGE_SHOW_TIME,
            }
        )
    }

    for msg in log.messages.iter_mut() {
        msg.alive -= time.delta_seconds();
    }

    let mut to_remove = Vec::<usize>::new();
    for (i, _) in log.messages.iter().enumerate() {
        if log.messages.get(i).unwrap().alive < 0. {
            to_remove.push(i)
        }
    }
    
    for i in to_remove.iter() {
        log.messages.remove(*i);
    }
}

fn display_log(
    log: Res<Log>,
    mut contexts: EguiContexts,
) {
    egui::SidePanel::left("Log")
        .frame(egui::Frame::none().stroke(egui::Stroke::NONE))
        .max_width(300.)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                for message in log.messages.iter().rev() {
                    ui.label(&message.text);
                }
            });
        });
}

#[derive(Resource)]
struct TextMessages {
    messages: VecDeque<TextMessage>,
    input: String,
}

struct TextMessage {
    text: String,
    from: bevy_matchbox::prelude::PeerId,
    roll: bool,
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct RecieveMessage {
    pub text: String,
    pub from: bevy_matchbox::prelude::PeerId,
    pub roll: bool,
}

fn update_messages(
    mut events: EventReader<RecieveMessage>,
    mut log: ResMut<TextMessages>,
) {
    for ev in events.read() {
        log.messages.push_back(
            TextMessage{
                text: ev.text.clone(),
                from: ev.from.clone(),
                roll: ev.roll,
            }
        )
    }

    while log.messages.len() > 20 {
        log.messages.pop_front();
    }

}

fn text_messages(
    mut log: ResMut<TextMessages>,
    mut contexts: EguiContexts,
    mut ev_client: EventWriter<networking::ClientCommandEvent>,
    local_peer_id: Option<Res<networking::LocalPeerId>>,
) {
    let Some(local_peer_id) = local_peer_id else {
        return
    };
    egui::SidePanel::left("Messages")
        .max_width(500.)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut log.input);
                    let btn = ui.button("Send");
                    if btn.clicked() {
                        input::send_message(log.input.clone(), &mut ev_client, local_peer_id);
                    }
                });
                
                for message in log.messages.iter().rev() {
                    ui.horizontal(|ui| {
                        if message.roll {
                            ui.label("ROLL");
                        }
                        ui.label(format!("{} : {}", &message.from, &message.text));
                    });
                }
            });
        });
}
