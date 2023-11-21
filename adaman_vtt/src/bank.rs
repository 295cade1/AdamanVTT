use bevy::prelude::*;
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;
use serde::Serialize;

use general_storage::Storage;

#[cfg(not(target_family = "wasm"))]
use general_storage_file::{
    FileStorage,
    IfDirectoryMissing
};
#[cfg(not(target_family = "wasm"))]
use directories::ProjectDirs;

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

#[cfg(not(target_family = "wasm"))]
#[derive(Resource)]
pub struct Bank {
    data: FileStorage,
}

#[cfg(not(target_family = "wasm"))]
fn setup_bank(
    mut commands: Commands,
) {
    if let Some(proj_dirs) = ProjectDirs::from("vtt", "Cade", "AdamanVTT") {
        let path = proj_dirs.data_dir();
    let bank = FileStorage::new(path, IfDirectoryMissing::Create).expect("Failed to create storage location");
    commands.insert_resource(Bank{
        data: bank
    })
    }
}

#[cfg(not(target_family = "wasm"))]
impl Bank {
    pub fn request_data(&self, id: &DataId) -> Option<Arc<Vec<u8>>> {
        let data = self.data.load_raw(id.0.to_string()).ok();
        let Some(data) = data else {
            return None;
        };
        return Some(Arc::new(data));
    }

    pub fn contains_data(&self, id: &DataId) -> bool {
        self.data.exists(id.0.to_string())
    }

    pub fn insert_data(&mut self, id: &DataId, data: Arc<Vec<u8>>) {
        let _ = self.data.store_raw(id.0.to_string(), data.as_slice());
    }
}
