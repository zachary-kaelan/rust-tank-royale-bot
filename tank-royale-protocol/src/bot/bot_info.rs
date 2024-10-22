use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotInfo {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: String,
    pub homepage: String,
    pub country_codes: Option<Vec<String>>,

    pub game_types: Vec<String>,
    pub platform: String,
    pub programming_lang: String,
    pub initial_position: Option<crate::connection::message_types::InitialPosition>,
}