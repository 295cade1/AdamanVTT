use bevy::prelude::*;
use uuid::uuid;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

use crate::bank;
use crate::fileload;

pub struct FilesPlugin;

impl Plugin for FilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, check_for_main.after(bank::setup_bank))
            .add_systems(Update, register_token)
            .add_event::<RegisterToken>()
            .add_event::<TokenListUpdated>()
            .add_systems(Update, register_map)
            .add_event::<RegisterMap>()
            .add_event::<MapListUpdated>()
            .add_systems(Update, register_encounter)
            .add_event::<RegisterEncounter>()
            .add_event::<EncounterListUpdated>()
        ;
    }
}

pub const CAMPAIGNS_ID: bank::DataId = bank::DataId(uuid!("00000000-0000-0000-0000-ffffff000000"));
pub const TOKENS_ID: bank::DataId = bank::DataId(uuid!("00000000-0000-0000-0000-ffffff000001"));
pub const ENCOUNTER_ID: bank::DataId = bank::DataId(uuid!("00000000-0000-0000-0000-ffffff000003"));
pub const MAPS_ID: bank::DataId = bank::DataId(uuid!("00000000-0000-0000-0000-ffffff000002"));

fn check_for_main(
    mut bank: ResMut<bank::Bank>,
) {
    if !bank.contains_data(&CAMPAIGNS_ID) {
        let menu = MainMenu::new();
        let menu_data = Arc::new(serde_json::to_vec(&menu).ok().unwrap());
        bank.store_at_id(&CAMPAIGNS_ID, menu_data);
    }
    if !bank.contains_data(&TOKENS_ID) {
        let tokens = Arc::new(serde_json::to_vec(&TokenList::new()).ok().unwrap());
        bank.store_at_id(&TOKENS_ID, tokens);
    }
    if !bank.contains_data(&ENCOUNTER_ID) {
        let tokens = Arc::new(serde_json::to_vec(&EncounterList::new()).ok().unwrap());
        bank.store_at_id(&ENCOUNTER_ID, tokens);
    }
    if !bank.contains_data(&MAPS_ID) {
        let maps = Arc::new(serde_json::to_vec(&MapList::new()).ok().unwrap());
        bank.store_at_id(&MAPS_ID, maps);
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
    pub tokens: Vec<TokenFileData>,
}

impl TokenList {
    pub fn new() -> TokenList {
        TokenList {
            tokens: Vec::<TokenFileData>::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenFileData {
    pub name: String,
    pub load_identifier: fileload::LoadIdentifier,
}

#[derive(Serialize, Deserialize)]
pub struct EncounterList {
    pub encounters: Vec<EncounterFileData>,
}

impl EncounterList {
    pub fn new() -> EncounterList {
        EncounterList {
            encounters: Vec::<EncounterFileData>::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EncounterFileData {
    pub name: String,
    pub load_identifier: fileload::LoadIdentifier,
}

#[derive(Serialize, Deserialize)]
pub struct MapList {
    pub maps: Vec<MapFileData>,
}

impl MapList {
    pub fn new() -> MapList {
        MapList {
            maps: Vec::<MapFileData>::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MapFileData {
    pub name: String,
    pub load_identifier: fileload::LoadIdentifier,
}


impl bank::Bank {
    pub fn get_map_list(&self) -> MapList {
        serde_json::from_slice(self.request_data(&MAPS_ID).expect("Should have found maps.").as_slice()).expect("Should have been valid map data.")
    }

    pub fn get_encounter_list(&self) -> EncounterList {
        serde_json::from_slice(self.request_data(&ENCOUNTER_ID).expect("Should have found encounters.").as_slice()).expect("Should have been valid encounter data.")
    }


    pub fn get_token_list(&self) -> TokenList {
        serde_json::from_slice(self.request_data(&TOKENS_ID).expect("Should have found tokens.").as_slice()).expect("Should have been valid token data.")
    }
}

#[derive(Event)]
pub struct RegisterMap {
    pub name: String,
    pub load_identifier: fileload::LoadIdentifier,
}

#[derive(Event)]
pub struct MapListUpdated;

pub fn register_map(
    mut bank: ResMut<bank::Bank>,
    mut events: EventReader<RegisterMap>,
    mut update_event: EventWriter<MapListUpdated>,
) {
    for ev in events.read() {
        let mut maps = bank.get_map_list();
        maps.maps.push(
            MapFileData {
                name: ev.name.clone(),
                load_identifier: ev.load_identifier.clone(),
            }
        );
        let maps = Arc::new(serde_json::to_vec(&maps).ok().unwrap());
        bank.store_at_id(&MAPS_ID, maps);

        update_event.send(MapListUpdated);
    }
}

#[derive(Event)]
pub struct RegisterToken {
    pub name: String,
    pub load_identifier: fileload::LoadIdentifier,
}

#[derive(Event)]
pub struct TokenListUpdated;

pub fn register_token(
    mut bank: ResMut<bank::Bank>,
    mut events: EventReader<RegisterToken>,
    mut update_event: EventWriter<TokenListUpdated>,
) {
    for ev in events.read() {
        let mut tokens = bank.get_token_list();
        tokens.tokens.push(
            TokenFileData {
                name: ev.name.clone(),
                load_identifier: ev.load_identifier.clone(),
            }
        );
        let tokens = Arc::new(serde_json::to_vec(&tokens).ok().unwrap());
        bank.store_at_id(&TOKENS_ID, tokens);

        update_event.send(TokenListUpdated);
    }
}

#[derive(Event)]
pub struct RegisterEncounter {
    pub name: String,
    pub load_identifier: fileload::LoadIdentifier,
}

#[derive(Event)]
pub struct EncounterListUpdated;

pub fn register_encounter(
    mut bank: ResMut<bank::Bank>,
    mut events: EventReader<RegisterEncounter>,
    mut update_event: EventWriter<EncounterListUpdated>,
) {
    for ev in events.read() {
        let mut encounters = bank.get_encounter_list();
        encounters.encounters.push(
            EncounterFileData {
                name: ev.name.clone(),
                load_identifier: ev.load_identifier.clone(),
            }
        );
        let encounters = Arc::new(serde_json::to_vec(&encounters).ok().unwrap());
        bank.store_at_id(&ENCOUNTER_ID, encounters);

        update_event.send(EncounterListUpdated);
    }
}

