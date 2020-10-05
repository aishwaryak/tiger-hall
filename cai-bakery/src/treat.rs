use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_treats(path: &str) -> Result<Vec<Treat>, Box<dyn Error>> {
    let current_dir_path = env::current_dir()?;
    let file_path = format!("{}/files/{}", current_dir_path.display(), path);
    read_treats_from_file(file_path)
}

pub fn read_treats_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Treat>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BulkPricing {
    pub amount: u32,
    pub total_price: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Treat {
    id: u32,
    pub name: String,
    pub image_url: String,
    pub price: f32,
    pub bulk_pricing: Option<BulkPricing>,
}
