use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::orders;
use crate::bank;
use crate::fileload;
use crate::maps::MapId;
use crate::tokens::TokenId;
use crate::files;


pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_default_encounter)
            .add_systems(Update, load_encounter)
            .add_event::<EncounterLoad>()
            .add_systems(Update, save_encounter)
            .add_event::<EncounterSave>()
        ;
    }
}

#[derive(Resource)]
pub struct CurrentEncounterID(bank::DataId);

fn setup_default_encounter(
    mut commands: Commands,
) {
    let current_encounter = CurrentEncounterID(bank::get_new_id());
    commands.insert_resource(current_encounter);
}

//Load Encounter 
#[derive(Event)]
pub struct EncounterLoad {
    pub data_id: bank::DataId,
    pub data: Arc<Vec<u8>>,
}

fn load_encounter(
    mut commands: Commands,
    mut ev_encounter_load: EventReader<EncounterLoad>,
    maps: Query<(Entity, With<MapId>)>,
    tokens: Query<(Entity, With<TokenId>)>,
    mut current_encounter: ResMut<CurrentEncounterID>,
    mut map_creation: EventWriter<orders::CreateMapCommand>,
    mut token_creation: EventWriter<orders::CreateTokenCommand>,
) {
    for ev in ev_encounter_load.read() {
        //Remove all the old maps
        for (map, _) in maps.iter() {
            commands.entity(map).despawn_recursive();
        }
        //Remove all the old tokens
        for (token, _) in tokens.iter() {
            commands.entity(token).despawn_recursive();
        }
        //
        //Update the current encounter resource
        current_encounter.0 = ev.data_id;

        let Some(data) = serde_json::from_slice::<Encounter>(ev.data.as_slice()).ok() else {
            println!("Bad Encounter Data");
            continue;
        };

        //Actually handle the loading of entities
        for map_load in data.map_instances.iter() {
            map_creation.send(map_load.command.clone())
        }
        //Actually handle the loading of entities
        for token_load in data.token_instances.iter() {
            token_creation.send(token_load.command.clone())
        }
    }
}

//Save Encounter
#[derive(Event)]
pub struct EncounterSave{
    pub name: String,
}

fn save_encounter(
    mut ev_encounter_save: EventReader<EncounterSave>,
    maps: Query<(&fileload::LoadIdentifier, &MapId, &Transform)>,
    tokens: Query<(&fileload::LoadIdentifier, &TokenId, &Transform)>,
    current_encounter: ResMut<CurrentEncounterID>,
    mut bank: ResMut<bank::Bank>,
    mut ev_register_encounter: EventWriter<files::RegisterEncounter>,
) {
    for ev in ev_encounter_save.read() {
        let mut map_instances = Vec::<MapInstance>::new();
        for (data_id, map_id, transform) in maps.iter() {
            map_instances.push(
                MapInstance{
                    command: orders::CreateMapCommand{
                        data_id: data_id.clone(),
                        map_id: *map_id,
                        x: transform.translation.x,
                        y: transform.translation.z,
                    }
                }
            )
        }
        let mut token_instances = Vec::<TokenInstance>::new();
        for (data_id, token_id, transform) in tokens.iter() {
            token_instances.push(
                TokenInstance{
                    command: orders::CreateTokenCommand{
                        load_identifier: data_id.clone(),
                        id: *token_id,
                        x: transform.translation.x,
                        y: transform.translation.z,
                    }
                }
            )
        }
        let enc = Encounter{
            map_instances,
            token_instances,
        };
        let enc_data = serde_json::to_vec(&enc).expect("Unable to serialize encounter data");
        let load_identifier = bank.store_at_id(&current_encounter.0, enc_data.into());
        
        ev_register_encounter.send(
            files::RegisterEncounter{
                name: ev.name.clone(),
                load_identifier,
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Encounter {
    pub map_instances: Vec<MapInstance>,
    pub token_instances: Vec<TokenInstance>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MapInstance {
    command: orders::CreateMapCommand,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenInstance {
    command: orders::CreateTokenCommand,
}
