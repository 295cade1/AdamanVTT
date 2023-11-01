use bevy::prelude::*;
use std::collections::HashMap;

use crate::maps;

pub struct BankPlugin;

impl Plugin for BankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bank);
    }
}

#[derive(Resource)]
pub struct Bank{
    map: HashMap<maps::MapId, maps::MapData>,
}

fn setup_bank(
    mut commands: Commands,
) {
    commands.insert_resource(Bank{
        map: HashMap::new(),
    })
}

impl Bank {
    pub fn request_map(&self, id: &maps::MapId) -> Option<&maps::MapData> {
        self.map.get(id)
    }

    pub fn contains_map(&self, id: &maps::MapId) -> bool {
        self.map.contains_key(id)
    }

    pub fn insert_map(&mut self, id: &maps::MapId, data: maps::MapData) {
        self.map.insert(*id, data);
    }
}
