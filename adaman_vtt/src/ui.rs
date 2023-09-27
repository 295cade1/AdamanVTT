use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct UIPlugin;

impl Plugin for UIPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(EguiPlugin)
      .add_systems(Update, ui_test);
  }
}

fn ui_test(mut contexts: EguiContexts) {
  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    ui.label("world");
  });
}
