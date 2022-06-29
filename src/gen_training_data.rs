mod shared;

use shared::models::{Card, CardFace};
use rand::prelude::SliceRandom;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

/// Training data model for cards.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainingCard {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub card_faces: Vec<TrainingCardFace>,
    pub name: String,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub flavor_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana_cost: Option<String>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub color_indicator: HashSet<String>,
    #[serde()]
    pub rarity: String,
}

impl From<Card> for TrainingCard {
    fn from(card: Card) -> Self {
        TrainingCard {
            card_faces: card.card_faces.into_iter().map(Into::into).collect(),
            name: card.name,
            type_line: card.type_line,
            oracle_text: card.oracle_text,
            flavor_text: card.flavor_text,
            loyalty: card.loyalty,
            power: card.power,
            toughness: card.toughness,
            mana_cost: card.mana_cost,
            color_indicator: card.color_indicator,
            rarity: card.rarity,
        }
    }
}

/// Training data model for card faces.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainingCardFace {
    pub name: String,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub flavor_text: Option<String>,
    pub loyalty: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub mana_cost: Option<String>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub color_indicator: HashSet<String>,
}

impl From<CardFace> for TrainingCardFace {
    fn from(face: CardFace) -> Self {
        TrainingCardFace {
            name: face.name,
            type_line: face.type_line,
            oracle_text: face.oracle_text,
            flavor_text: face.flavor_text,
            loyalty: face.loyalty,
            power: face.power,
            toughness: face.toughness,
            mana_cost: face.mana_cost,
            color_indicator: face.color_indicator,
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Read cards
    let input = File::open("data/pioneer-cards.json")?;
    let cards: Vec<Card> = serde_json::from_reader(input)?;

    // Generate completions
    let mut completions: Vec<_> = cards
        .into_iter()
        .filter_map(|card| {
            serde_json::to_string_pretty(&card)
                .ok()
                .map(|completion| PromptCompletionPair {
                    prompt: format!("// Generated: {}\n", card.name),
                    completion,
                })
        })
        .collect();

    // Shuffle completions
    let mut rng = rand::thread_rng();
    completions.shuffle(&mut rng);

    // Generate output
    let output = File::create("data/training-set.jsonl")?;
    let output = Arc::new(Mutex::new(output));
    completions
        .into_par_iter()
        .map(|completion| serde_json::to_string(&completion).unwrap())
        .for_each_with(output, |output, completion| {
            let mut output = output.lock().unwrap();
            writeln!(output, "{completion}").unwrap();
        });

    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PromptCompletionPair {
    pub prompt: String,
    pub completion: String,
}
