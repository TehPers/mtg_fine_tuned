use image::{imageops::FilterType, ImageOutputFormat};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{fs::File, path::PathBuf};

const OUTPUT_WIDTH: u32 = 64;
const OUTPUT_HEIGHT: u32 = 64;

fn main() -> anyhow::Result<()> {
    let input_path: PathBuf = "data/images/art_crop".into();
    let output_path: PathBuf = format!("data/resized_images/art_crop/{OUTPUT_WIDTH}_{OUTPUT_HEIGHT}").into();
    std::fs::create_dir_all(&output_path)?;

    // Resize all images
    std::fs::read_dir(&input_path)?
        .par_bridge()
        .filter_map(|source_entry| {
            let source_entry = source_entry.ok()?;
            source_entry
                .file_type()
                .ok()?
                .is_file()
                .then(move || source_entry)
        })
        .try_for_each(|source_entry| -> anyhow::Result<()> {
            // Read source image
            let source = image::open(source_entry.path())?;

            // Check if output path already exists
            let mut dest_path = output_path.clone();
            dest_path.push(source_entry.file_name());
            if dest_path.exists() {
                return Ok(());
            }

            // Create destination file
            let mut dest_file = File::create(dest_path)?;

            // Resize image
            let dest = source.resize_exact(OUTPUT_WIDTH, OUTPUT_HEIGHT, FilterType::Lanczos3);
            dest.write_to(&mut dest_file, ImageOutputFormat::Png)?;

            Ok(())
        })?;

    Ok(())
}
