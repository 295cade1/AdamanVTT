use crate::fileload;

use bevy::prelude::*;
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;
use serde::Serialize;

#[cfg(target_env = "wasm")]
use std::collections::HashMap;
#[cfg(target_env = "wasm")]
use std::collections::VecDeque;

use bevy_pkv::PkvStore;

pub struct BankPlugin;

impl Plugin for BankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_bank);
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Component, Eq, Hash, PartialEq)]
pub struct DataId(pub uuid::Uuid);

pub fn get_new_id() -> DataId {
    DataId(Uuid::new_v4())
}

#[cfg(not(target_env = "wasm"))]
#[derive(Resource)]
pub struct Bank {
    data: PkvStore,
}

#[cfg(target_env = "wasm")]
#[derive(Resource)]
pub struct Bank {
    data: HashMap<DataId, Arc<Vec<u8>>>,
    last: VecDeque<DataId>,
}

#[cfg(not(target_env = "wasm"))]
pub fn setup_bank(
    mut commands: Commands,
) {
    commands.insert_resource(Bank{
        data: PkvStore::new("Cade", "AdamanVTT")
    })
}

#[cfg(target_env = "wasm")]
pub fn setup_bank(
    mut commands: Commands,
) {
    commands.insert_resource(Bank{
        data: HashMap::new(),
        last: VecDeque::new(),
    })
}

impl Bank {
    pub fn store(&mut self, data: Arc<Vec<u8>>) -> fileload::LoadIdentifier {
        let id = get_new_id();
        self.store_at_id(&id, data)
    }
    
    pub fn store_at_id(&mut self, id: &DataId, data: Arc<Vec<u8>>) -> fileload::LoadIdentifier {
        let size = (*data).len();
        let hash = (*data).reflect_hash().expect("Unable to hash vec<u8>");

        self.insert_data(id, data);

        fileload::LoadIdentifier{
            data_id: *id,
            size,
            hash,
        }
    }

    #[cfg(not(target_env = "wasm"))]
    pub fn request_data(&self, id: &DataId) -> Option<Arc<Vec<u8>>> {
        let data = self.data.get(id.0.to_string()).ok();
        let Some(data) = data else {
            return None;
        };
        Some(Arc::new(data))
    }

    #[cfg(not(target_env = "wasm"))]
    pub fn contains_data(&self, id: &DataId) -> bool {
        self.data.get::<Vec<u8>>(id.0.to_string()).is_ok()
    }

    #[cfg(not(target_env = "wasm"))]
    fn insert_data(&mut self, id: &DataId, data: Arc<Vec<u8>>) {
        let _ = self.data.set(id.0.to_string(), &data);
    }

    #[cfg(target_env = "wasm")]
    pub fn request_data(&self, id: &DataId) -> Option<Arc<Vec<u8>>> {
        let data = self.data.get(id);
        let Some(data) = data else {
            return None;
        };
        Some(data.clone())
    }

    #[cfg(target_env = "wasm")]
    pub fn contains_data(&self, id: &DataId) -> bool {
        self.data.get(id).is_some()
    }

    #[cfg(target_env = "wasm")]
    fn insert_data(&mut self, id: &DataId, data: Arc<Vec<u8>>) {
        let _ = self.data.insert(id.clone(), data);
    }
}
