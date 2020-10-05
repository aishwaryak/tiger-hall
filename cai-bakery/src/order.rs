use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use cron_parser::parse;
use std::collections::HashMap;

use super::offer;
use super::treat;

//Order Tuple
pub struct Tuple {
    pub name: String,
    pub unit: u32,
}

pub struct Order {
    pub tuple: Vec<Tuple>,
    pub total_price: f32,
}

pub fn calculate_price_without_offer(
    treat_map: &HashMap<&String, &treat::Treat>,
    unit: u32,
    item_name: &String,
) -> f32 {
    let treat_details = treat_map.get(&item_name);
    let mut item_total: f32 = 0.0;
    match treat_details {
        None => panic!("Inventory lookup failed for {}!", item_name),
        Some(treat) => {
            match &treat.bulk_pricing {
                None => {
                    //Without bulk price
                    item_total = unit as f32 * treat.price
                }
                Some(bulk) => {
                    //With existing bulk price.
                    let mut temp_unit = unit; //Since we can't modify unit directly.
                    if bulk.amount <= temp_unit {
                        let bulk_items = temp_unit / bulk.amount;
                        item_total = bulk_items as f32 * bulk.total_price;
                        let dividor = temp_unit % bulk.amount;
                        temp_unit = dividor;
                    }
                    if temp_unit > 0 {
                        item_total += temp_unit as f32 * treat.price
                    }
                }
            };
        }
    };
    item_total
}

pub fn calculate_price_with_offer(
    treat_map: &HashMap<&String, &treat::Treat>,
    item: &Tuple,
    offer_map: &HashMap<&String, &offer::Offer>,
    order_date: &str,
) -> f32 {
    let mut total_offer_price: f32 = 0.0;

    if offer_map.contains_key(&item.name) {
        let offer = offer_map.get(&item.name);

        match offer {
            None => {
                total_offer_price = calculate_price_without_offer(treat_map, item.unit, &item.name);
            }
            Some(existing_offer) => {
                if let Ok(order_date_result) =
                    NaiveDateTime::parse_from_str(order_date, "%Y/%m/%d  %H:%M:%S")
                {
                    let chrono_order_date = DateTime::<Utc>::from_utc(order_date_result, Utc);

                    //Cron expression would return the next minute that matches with the chrono_order_date
                    //For example 2021/10/1 00:00:00 as input it would return the 2021/10/1 00:01:00
                    if let Ok(next_date) =
                        parse(&existing_offer.cron_expression, &chrono_order_date)
                    {
                        //Subtract a minute - since the chrono expression gives the next minute that satisfies the condition.
                        let next_min = next_date - Duration::minutes(1);

                        if chrono_order_date.date() == next_min.date() {
                            //The offer can be applied.
                            let treat_details = treat_map.get(&item.name);

                            match treat_details {
                                None => panic!("Inventory lookup failed for {}!", &item.name),
                                Some(treat) => {
                                    let (remaining, offer_price) =
                                        offer::get_price(*existing_offer, item.unit, treat.price);

                                    //Calculate price for the rest of the units that don't have offer
                                    total_offer_price += offer_price;
                                    if remaining > 0 {
                                        total_offer_price += calculate_price_without_offer(
                                            treat_map, remaining, &item.name,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            } //Closing some
        } //Closing match
    } //Closing if

    total_offer_price
}

pub fn execute_order(
    order_date: &str,
    items: &std::vec::Vec<Tuple>,
    treat_map: &HashMap<&String, &treat::Treat>,
    offer_map: &HashMap<&String, &offer::Offer>,
) -> f32 {
    let mut order_total: f32 = 0.0;
    if order_date.is_empty() {
        //Without offers
        for item in items.iter() {
            order_total += calculate_price_without_offer(treat_map, item.unit, &item.name);
        }
    } else {
        //With possible offers
        for item in items.iter() {
            order_total += calculate_price_with_offer(treat_map, item, offer_map, &order_date);
        }
    }
    order_total
}
