use chrono::Utc;
use serde::{Deserialize, Serialize};
use strum::Display;

type DateTime = chrono::DateTime<Utc>;

#[derive(Display, Serialize, Deserialize)]
pub enum ClientProvider {
    Aviasim,
    Bodo,
    Vnebo,
}

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub phone: String,
    pub email: String,
    pub first_name: String,
    pub second_name: String,
    pub provider: ClientProvider,
}

#[derive(Display, Serialize, Deserialize)]
pub enum Simulator {
    Boeing737,
    F18,
    BeechCraft,
}

#[derive(Display, Serialize, Deserialize)]
pub enum Discount {
    #[strum(to_string = "Percent")]
    Percent(f32),
}

#[derive(Serialize, Deserialize)]
pub struct Promocode {
    pub code: String,
    pub expire_date: DateTime,
    pub discount: Discount,
}

#[derive(Serialize, Deserialize)]
pub struct OrderInfo {
    pub client: Client,
    pub duration: u32,
    pub simulator: Simulator,
    pub promocode: Option<Promocode>,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DirectOrder {
    pub order_info: OrderInfo,
    pub time: DateTime,
}

#[derive(Serialize, Deserialize)]
pub struct GiftCode {
    pub code: String,
    pub expire_date: DateTime,
    pub is_physical: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GiftOrder {
    pub order_info: OrderInfo,
    pub gift_code: GiftCode,
    pub recipient: Client,
    pub delivery_address: Option<String>,
    pub activated_time: Option<DateTime>,
}
