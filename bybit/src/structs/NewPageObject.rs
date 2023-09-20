use notion::data_types::{Date::Date, Number::Number, Select::Select, Title::Title};
use serde::{self, Deserialize, Serialize};
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct NewPageObject {
    #[serde(rename(serialize = "Name"))]
    pub title: Title, // String
    #[serde(rename(serialize = "Asset"))]
    pub asset: Select, // String
    #[serde(rename(serialize = "Entry Price"))]
    pub entry_price: Number, // f64
    #[serde(rename(serialize = "Entry Date"))]
    pub entry_date: Date,
    #[serde(rename(serialize = "Side"))]
    pub side: Select, //String
    #[serde(rename(serialize = "Market"))]
    pub market: Select, //String
    #[serde(rename(serialize = "Exchange"))]
    pub exchange: Select, //String
}
