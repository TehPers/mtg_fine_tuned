mod shared;

use shared::models::Card;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::File;

fn main() -> anyhow::Result<()> {
    // Parse input cards
    let input = File::open("./data/oracle-cards.json")?;
    let cards: Vec<serde_json::Value> = serde_json::from_reader(input)?;
    println!("{} cards read.", cards.len());

    let cards = cards
        .into_par_iter()
        .map(|card| serde_json::from_value(card))
        .collect::<Result<Vec<Card>, _>>()?;
    println!("{} cards parsed.", cards.len());

    // Filter cards
    let filtered_cards: Vec<_> = cards
        .into_par_iter()
        .filter(|card| card.legalities.get("pioneer").map(String::as_str) == Some("legal"))
        .collect();
    println!("{} cards legal in pioneer.", filtered_cards.len());

    // Write output file
    let output = File::create("./data/pioneer-cards.json")?;
    serde_json::to_writer(output, &filtered_cards)?;

    Ok(())
}
