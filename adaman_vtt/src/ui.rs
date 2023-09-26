use bevy::prelude::*;
use bevy::egui::{egui, EguiContexts, EguiPlugin};

pub struct UIPlugin;

impl Plugin for UIPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(EguiPlugin)
  }
}
