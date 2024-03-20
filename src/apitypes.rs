use serde::Deserialize;
use std::collections::HashMap;

// Run
#[derive(Deserialize, Debug)]
pub struct RunData {
    pub id: String,
    pub category: String,
    pub game: String,
    pub level: Option<String>,
    pub status: RunStatus,
    pub values: HashMap<String, String>,
    pub weblink: String,
    pub times: Times,
    pub date: String,
}

#[derive(Deserialize, Debug)]
pub struct Times {
    pub primary_t: f64,
}

#[derive(Deserialize, Debug)]
pub struct Run {
    pub place: u16,
    pub run: RunData,
}

#[derive(Deserialize, Debug)]
pub struct RunStatus {
    #[serde(rename = "verify-date")]
    pub verify_date: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RunResponse {
    pub data: Option<Vec<Run>>,
}

// Game
#[derive(Deserialize, Debug)]
pub struct GameResponse {
    pub data: Game,
}

#[derive(Deserialize, Debug)]
pub struct Game {
    pub id: String,
    pub names: GameNames,
    pub assets: GameAssets,
}

#[derive(Deserialize, Debug)]
pub struct GameNames {
    pub international: String,
}

#[derive(Deserialize, Debug)]
pub struct GameAssets {
    #[serde(rename = "cover-medium")]
    pub cover_medium: Asset,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub uri: String,
}

// Category
#[derive(Deserialize, Debug)]
pub struct CategoryResponse {
    pub data: Category,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    pub name: String,
}

// Level
#[derive(Deserialize, Debug)]
pub struct LevelResponse {
    pub data: Level,
}

#[derive(Deserialize, Debug)]
pub struct Level {
    pub name: String,
}

// Variable
#[derive(Deserialize, Debug)]
pub struct VariableResponse {
    pub data: Variable,
}

#[derive(Deserialize, Debug)]
pub struct Variable {
    pub values: VariableValues,
}

#[derive(Deserialize, Debug)]
pub struct VariableValues {
    pub values: HashMap<String, VariableLabel>,
}

#[derive(Deserialize, Debug)]
pub struct VariableLabel {
    pub label: String,
}
