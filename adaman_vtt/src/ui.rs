use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::networking;
use crate::input;

pub struct UIPlugin;

impl Plugin for UIPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(EguiPlugin)
      .add_systems(Startup, init_ui)
      .add_systems(Update, ui);
  }
}

fn init_ui(mut commands: Commands) {
  let ui_state = UIState{
    map_url: "".to_string(),
  };
  commands.insert_resource(ui_state);
}


#[derive(Resource)]
struct UIState{
  map_url: String,
}

fn ui(
  commands: Commands,
  mut contexts: EguiContexts,
  mut ev_client: EventWriter<networking::ClientCommandEvent>,
  mut ui_state: ResMut<UIState>,
) {
  egui::SidePanel::right("Token Creation")
    .default_width(200.0)
    .resizable(true)
    .show(contexts.ctx_mut(), |ui| {
      let create_token_btn = ui.button("Create Token");
      if create_token_btn.clicked() {
        ev_client.send(input::create_token(0., 0., None))
      }
      ui.text_edit_singleline(&mut ui_state.map_url);
      let create_map_btn = ui.button("Create Map");
      if create_map_btn.clicked() {
        ev_client.send(input::create_map(0., 0., Some(&ui_state.map_url)));
      }
      let create_map_file_btn = ui.button("Create Map from file");
      if create_map_file_btn.clicked() {
        input::create_map_from_file(commands, 0., 0.);
      }
  });
}
