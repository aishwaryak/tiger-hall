use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_offers(path: &str) -> Result<Vec<Offer>, Box<dyn Error>> {
    let current_dir_path = env::current_dir()?;
    let file_path = format!("{}/files/{}", current_dir_path.display(), path);
    read_offers_from_file(file_path)
}

pub fn read_offers_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Offer>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;
    Ok(u)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    pub cron_expression: String,
    pub item: String,
    pub unit: u32,
    pub offer_price: f32,
    pub percent: f32,
    pub offer_type: String,
    pub x_amount: u32,
    pub y_amount: u32,
}

pub fn get_price(offer: &Offer, order_qty: u32, single_unit_price: f32) -> (u32, f32) {
    match offer.offer_type.as_str() {
        "quantity" => get_quantity_price(offer.unit, offer.offer_price, order_qty),
        "percentage" => get_percentage_price(offer.percent, order_qty, single_unit_price),
        "xForY" => get_xfory_price(offer.x_amount, offer.y_amount, order_qty, single_unit_price),
        _ => (order_qty, 0.0),
    }
}

fn get_quantity_price(offer_unit: u32, offer_price: f32, order_qty: u32) -> (u32, f32) {
    let mut remaining_qty: u32 = order_qty;
    let mut total_offer_price: f32 = 0.0;
    if offer_unit <= order_qty {
        total_offer_price = (order_qty / offer_unit) as f32 * offer_price;
        remaining_qty = order_qty % offer_unit;
    }
    (remaining_qty, total_offer_price)
}

fn get_percentage_price(offer_percent: f32, order_qty: u32, single_unit_price: f32) -> (u32, f32) {
    let actual_price: f32 = order_qty as f32 * single_unit_price;
    let total_offer_price: f32 = actual_price * (offer_percent / 100.00);
    (0, actual_price - total_offer_price)
}

fn get_xfory_price(
    offer_x_amount: u32,
    offer_y_amount: u32,
    order_qty: u32,
    single_unit_price: f32,
) -> (u32, f32) {
    let mut remaining_qty: u32 = order_qty;
    let mut total_offer_price: f32 = 0.0;

    if offer_y_amount <= order_qty {
        total_offer_price =
            (order_qty / offer_x_amount) as f32 * offer_y_amount as f32 * single_unit_price;
        remaining_qty = order_qty % offer_x_amount;
    }
    (remaining_qty, total_offer_price)
}
