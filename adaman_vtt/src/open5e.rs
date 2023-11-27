use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use bevy_async_task::*;

use std::sync::Arc;

use crate::tokens;

pub struct Open5ePlugin;

impl Plugin for Open5ePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_connection)
            .insert_resource(Open5eMonsterSelection{
                ..Default::default()
            })
        ;
    }
}

#[derive(Resource)]
pub struct Open5eMonsterSelection {
    client: Arc<reqwest::Client>,
    current: Option<Root>,
    current_action: RequestAction,
}

#[derive(PartialEq)]
enum RequestAction {
    Idle,
    Init,
    Next,
    Back,
}

impl Open5eMonsterSelection {
    pub fn get_list(&mut self) -> Option<&Vec<Result>> {
        let Some(ref current) = self.current else {
            self.current_action = RequestAction::Init;
            return None
        };
        Some(&current.results)
    }
}

impl Default for Open5eMonsterSelection {
    fn default() -> Self {
        Open5eMonsterSelection {
            client: reqwest::Client::new().into(),
            current: None,
            current_action: RequestAction::Idle,
        }
    }
}

const BASE_URL: &str = "https://api.open5e.com/v1";
const MONSTER_URL: &str = "monsters";

fn update_connection(
    mut connection: ResMut<Open5eMonsterSelection>,
    mut poll_connection: AsyncTaskRunner<Option<Root>>,
) {
    match poll_connection.poll() {
        AsyncTaskStatus::Idle => {
            if connection.current_action == RequestAction::Init {
                let connection_str = BASE_URL.to_owned() + "/" + &MONSTER_URL;
                let client = connection.client.clone();
                let task = async move {
                    let result = client.get(connection_str).send().await.ok().unwrap();
                    let text = result.text().await.unwrap();
                    println!("{}", &text);

                    Some(Root {
                        count: 0,
                        next: "".to_string(),
                        previous: "".to_string(),
                        results: Vec::<Result>::new(),
                    })
                };
                println!("Started Open5e Connection");
                poll_connection.start(task);
            }
        },
        AsyncTaskStatus::Pending => {},
        AsyncTaskStatus::Finished(x) => {
            connection.current_action = RequestAction::Idle;
            connection.current = x;
        },
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub count: i64,
    pub next: String,
    pub previous: String,
    pub results: Vec<Result>,
}

impl From<Result> for tokens::TokenData {
    fn from(x: Result) -> Self {
        tokens::TokenData{
            format: 0.1,
            name: x.name,
            size: x.size,
            armor_class: x.armor_class,
            hit_points: x.hit_points,
            type_field: x.type_field,
            img: x.img_main,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub slug: String,
    pub desc: String,
    pub name: String,
    pub size: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub subtype: String,
    pub group: Option<String>,
    pub alignment: String,
    #[serde(rename = "armor_class")]
    pub armor_class: i64,
    #[serde(rename = "armor_desc")]
    pub armor_desc: Option<String>,
    #[serde(rename = "hit_points")]
    pub hit_points: i64,
    #[serde(rename = "hit_dice")]
    pub hit_dice: String,
    pub speed: Speed,
    pub strength: i64,
    pub dexterity: i64,
    pub constitution: i64,
    pub intelligence: i64,
    pub wisdom: i64,
    pub charisma: i64,
    #[serde(rename = "strength_save")]
    pub strength_save: Option<i64>,
    #[serde(rename = "dexterity_save")]
    pub dexterity_save: Option<i64>,
    #[serde(rename = "constitution_save")]
    pub constitution_save: Option<i64>,
    #[serde(rename = "intelligence_save")]
    pub intelligence_save: Option<i64>,
    #[serde(rename = "wisdom_save")]
    pub wisdom_save: Option<i64>,
    #[serde(rename = "charisma_save")]
    pub charisma_save: Option<i64>,
    pub perception: Option<i64>,
    pub skills: Skills,
    #[serde(rename = "damage_vulnerabilities")]
    pub damage_vulnerabilities: String,
    #[serde(rename = "damage_resistances")]
    pub damage_resistances: String,
    #[serde(rename = "damage_immunities")]
    pub damage_immunities: String,
    #[serde(rename = "condition_immunities")]
    pub condition_immunities: String,
    pub senses: String,
    pub languages: String,
    #[serde(rename = "challenge_rating")]
    pub challenge_rating: String,
    pub cr: f64,
    pub actions: Vec<Action>,
    #[serde(rename = "bonus_actions")]
    pub bonus_actions: Value,
    #[serde(default)]
    pub reactions: Vec<Reaction>,
    #[serde(rename = "legendary_desc")]
    pub legendary_desc: String,
    #[serde(rename = "legendary_actions")]
    pub legendary_actions: Option<Vec<LegendaryAction>>,
    #[serde(rename = "special_abilities")]
    #[serde(default)]
    pub special_abilities: Vec<SpecialAbility>,
    #[serde(rename = "spell_list")]
    pub spell_list: Vec<String>,
    #[serde(rename = "page_no")]
    pub page_no: i64,
    pub environments: Vec<String>,
    #[serde(rename = "img_main")]
    pub img_main: Option<String>,
    #[serde(rename = "document__slug")]
    pub document_slug: String,
    #[serde(rename = "document__title")]
    pub document_title: String,
    #[serde(rename = "document__license_url")]
    pub document_license_url: String,
    #[serde(rename = "document__url")]
    pub document_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Speed {
    pub walk: i64,
    pub swim: Option<i64>,
    pub fly: Option<i64>,
    pub burrow: Option<i64>,
    pub climb: Option<i64>,
    pub hover: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skills {
    pub history: Option<i64>,
    pub perception: Option<i64>,
    pub medicine: Option<i64>,
    pub religion: Option<i64>,
    pub stealth: Option<i64>,
    pub persuasion: Option<i64>,
    pub insight: Option<i64>,
    pub deception: Option<i64>,
    pub arcana: Option<i64>,
    pub athletics: Option<i64>,
    pub acrobatics: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub name: String,
    pub desc: String,
    #[serde(rename = "attack_bonus")]
    pub attack_bonus: Option<i64>,
    #[serde(rename = "damage_dice")]
    pub damage_dice: Option<String>,
    #[serde(rename = "damage_bonus")]
    pub damage_bonus: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub name: String,
    pub desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegendaryAction {
    pub name: String,
    pub desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialAbility {
    pub name: String,
    pub desc: String,
    #[serde(rename = "attack_bonus")]
    pub attack_bonus: Option<i64>,
    #[serde(rename = "damage_dice")]
    pub damage_dice: Option<String>,
}

