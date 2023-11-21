use bevy::prelude::*;
use uuid::uuid;
use crate::bank;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

pub struct FilesPlugin;

impl Plugin for FilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, check_for_main);
    }
}

pub const MAIN_MENU_ID: bank::DataId = bank::DataId(uuid!("00000000-0000-0000-0000-ffffff000000"));

fn check_for_main(
    mut bank: ResMut<bank::Bank>,
) {
    if !bank.contains_data(&MAIN_MENU_ID) {
        let menu = Arc::new(serde_json::to_vec(&MainMenu::new()).ok().unwrap());
        bank.insert_data(&MAIN_MENU_ID, menu);
    }
}

#[derive(Serialize, Deserialize)]
pub struct MainMenu {
    pub campaigns: Vec<bank::DataId>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            campaigns: Vec::<bank::DataId>::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenList {
    pub tokens: Vec<bank::DataId>,
}
