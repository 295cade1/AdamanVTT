use serde::{Deserialize, Serialize};

use crate::maps;

impl From<DD2VTT> for maps::MapData {
    fn from(x: DD2VTT) -> Self {
        maps::MapData::new(
            x.format,
            x.image,
            maps::MapGrid{
                pixels_per: x.resolution.pixels_per_grid,
                width: x.resolution.map_size.x,
                height: x.resolution.map_size.y,
            }
        )
    }
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
