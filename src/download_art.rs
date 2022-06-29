mod shared;

use shared::models::Card;
use reqwest::{Client, Method, Request, Response, Url};
use std::{fs::File, io::Write, path::PathBuf, time::Duration};
use tower::{Service, ServiceBuilder, ServiceExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start().await
}

async fn start() -> anyhow::Result<()> {
    // Read cards
    let input = File::open("data/pioneer-cards.json")?;
    let cards: Vec<Card> = serde_json::from_reader(input)?;
    println!("{} cards", cards.len());

    // Create output directory
    let output_dir = PathBuf::from("data/images/art_crop");
    std::fs::create_dir_all(&output_dir)?;

    // Create rate limited web client
    let client = Client::default();
    let mut svc = ServiceBuilder::new()
        .rate_limit(10, Duration::new(1, 0))
        .service(tower::service_fn(move |req| client.execute(req)));

    // Download images
    for card in cards.into_iter() {
        // Get image file path
        let mut output_path = output_dir.clone();
        output_path.push(format!("{}.jpg", card.id));

        // Download the image if possible
        match card.image_uris.get("art_crop") {
            // Skip if already exists
            Some(_) if output_path.exists() => {
                println!("Skipping {}", card.name);
            }
            Some(art_crop) => {
                println!("Downloading art crop for {}...", card.name);
                let mut output_file = File::create(output_path)?;
                download_to(&mut svc, &mut output_file, Url::parse(art_crop)?).await?;
            }
            None => {
                println!("No art crop for {}", card.name);
                let _ = std::fs::remove_file(&output_path);
            }
        };

        // Download card face arts if possible
        for (i, face) in card.card_faces.into_iter().enumerate() {
            // Get image file path
            let mut output_path = output_dir.clone();
            output_path.push(format!("{}.{}.jpg", card.id, i));

            // Download face art if possible
            match face.image_uris.get("art_crop") {
                // Skip if already exists
                Some(_) if output_path.exists() => {
                    println!("Skipping {}", face.name);
                }
                Some(art_crop) => {
                    println!("Downloading art crop for {}...", face.name);
                    let mut output_file = File::create(output_path)?;
                    download_to(&mut svc, &mut output_file, Url::parse(art_crop)?).await?;
                }
                None => {
                    println!("No art crop for {}", face.name);
                    let _ = std::fs::remove_file(&output_path);
                }
            };
        }
    }

    Ok(())
}

async fn download_to<S>(mut svc: S, output_file: &mut File, art_crop: Url) -> anyhow::Result<()>
where
    S: Service<Request, Response = Response>,
    S::Error: std::error::Error + Send + Sync + 'static,
{
    let data = svc
        .ready()
        .await?
        .call(Request::new(Method::GET, art_crop))
        .await?
        .bytes()
        .await?;
    output_file.write_all(&data)?;

    Ok(())
}
