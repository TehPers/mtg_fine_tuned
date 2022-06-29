use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Source: https://scryfall.com/docs/api/cards
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    #[serde(default)]
    pub card_faces: Vec<CardFace>,
    pub name: String,
    #[serde(default)]
    pub type_line: String,
    #[serde(default)]
    pub oracle_text: Option<String>,
    #[serde(default)]
    pub flavor_text: Option<String>,
    #[serde(default)]
    pub loyalty: Option<String>,
    #[serde(default)]
    pub power: Option<String>,
    #[serde(default)]
    pub toughness: Option<String>,
    #[serde(default)]
    pub mana_cost: Option<String>,
    #[serde(default)]
    pub color_indicator: HashSet<String>,
    #[serde(default)]
    pub rarity: String,
    #[serde(default)]
    pub legalities: HashMap<String, String>,
    #[serde(default)]
    pub image_uris: HashMap<String, String>,
}

/// Source: https://scryfall.com/docs/api/cards#card-face-objects
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardFace {
    pub name: String,
    #[serde(default)]
    pub type_line: String,
    #[serde(default)]
    pub oracle_text: Option<String>,
    #[serde(default)]
    pub flavor_text: Option<String>,
    #[serde(default)]
    pub loyalty: Option<String>,
    #[serde(default)]
    pub power: Option<String>,
    #[serde(default)]
    pub toughness: Option<String>,
    #[serde(default)]
    pub mana_cost: Option<String>,
    #[serde(default)]
    pub color_indicator: HashSet<String>,
    #[serde(default)]
    pub image_uris: HashMap<String, String>,
}
