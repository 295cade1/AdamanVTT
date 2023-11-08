use bevy::prelude::*;
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub struct BankPlugin;

impl Plugin for BankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bank);
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Component, Eq, Hash, PartialEq)]
pub struct DataId(pub uuid::Uuid);

pub fn get_new_id() -> DataId {
    DataId(Uuid::new_v4())
}

#[derive(Resource)]
pub struct Bank{
    data: HashMap<DataId, Arc<Vec<u8>>>,
}

fn setup_bank(
    mut commands: Commands,
) {
    commands.insert_resource(Bank{
        data: HashMap::new(),
    })
}

impl Bank {
    pub fn request_data(&self, id: &DataId) -> Option<&Arc<Vec<u8>>> {
        self.data.get(id) 
    }

    pub fn contains_data(&self, id: &DataId) -> bool {
        self.data.contains_key(id)
    }

    pub fn insert_data(&mut self, id: &DataId, data: Arc<Vec<u8>>) {
        self.data.insert(*id, data);
    }
}
