#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuantityOffer {
    pub cron_expression: String,
    pub item: String,
    pub unit: u32,
    pub offer_price: f32,
    pub percent: u32,
    pub x_amount: u32,
    pub y_amount: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PercentageOffer {
    pub cron_expression: String,
    pub item: String,
    pub unit: u32,
    pub offer_price: f32,
    pub percent: u32,
    pub x_amount: u32,
    pub y_amount: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalItemsOffer {
    pub cron_expression: String,
    pub item: String,
    pub unit: u32,
    pub offer_price: f32,
    pub percent: u32,
    pub x_amount: u32,
    pub y_amount: u32,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Offer {
    Quantity { details: QuantityOffer },
    Percentage { details: PercentageOffer },
    AdditionalItems { details: AdditionalItemsOffer },
}

pub trait Sales {
    fn get_price(&self, order_qty: u32) -> (u32, f32);
}

impl Sales for QuantityOffer {
    fn get_price(&self, order_qty: u32) -> (u32, f32) {
        //Implement
        (0, 0.0)
    }
}

impl Sales for PercentageOffer {
    fn get_price(&self, order_qty: u32) -> (u32, f32) {
        //Implement
        (0, 0.0)
    }
}

impl Sales for AdditionalItemsOffer {
    fn get_price(&self, order_qty: u32) -> (u32, f32) {
        //Implement
        (0, 0.0)
    }
}
