use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

use crate::maps;

impl Into<maps::MapData> for DD2VTT {
    fn into(self) -> maps::MapData {
        maps::MapData{
            format: self.format,
            image: decode_img(self.image),
        }
    }
}

fn decode_img(img: String) -> Vec<u8> {
    general_purpose::STANDARD_NO_PAD.decode(img).unwrap()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DD2VTT {
    pub format: f64,
    pub resolution: Resolution,
    #[serde(rename = "line_of_sight")]
    pub line_of_sight: Vec<Vec<LineOfSight>>,
    pub portals: Vec<Portal>,
    pub lights: Vec<Light>,
    pub environment: Environment,
    pub image: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    #[serde(rename = "map_origin")]
    pub map_origin: MapOrigin,
    #[serde(rename = "map_size")]
    pub map_size: MapSize,
    #[serde(rename = "pixels_per_grid")]
    pub pixels_per_grid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapOrigin {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapSize {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineOfSight {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Portal {
    pub position: Position,
    pub bounds: Vec<Bound>,
    pub rotation: f64,
    pub closed: bool,
    pub freestanding: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bound {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Light {
    pub position: Position2,
    pub range: f64,
    pub intensity: f64,
    pub color: String,
    pub shadows: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    #[serde(rename = "baked_lighting")]
    pub baked_lighting: bool,
    #[serde(rename = "ambient_light")]
    pub ambient_light: String,
}
