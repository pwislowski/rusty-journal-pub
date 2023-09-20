use crate::parsers;
use notion::data_types::{Date::Date, Number::Number, Select::Select, Title::Title};
use serde::{self, Deserialize, Serialize};
use std::fmt::Debug;

use super::NewPageObject::NewPageObject;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PositionInfo {
    pub symbol: String,
    pub avg_price: f64,
    pub entry_price: f64,
    pub created_time: u64,
    pub leverage: f64,
    pub liq_price: f64,
    pub position_balance: f64,
    pub position_value: f64,
    pub side: String,
    pub size_in_qoute: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub unrealised_pnl: f64,
}

impl PositionInfo {
    pub fn build_new_page_object(&self, date_tran: Option<u64>) -> NewPageObject {
        //! param: date_tran: [Optional] The latest transaction date of the
        //! current position. The `PositonInfo` Object lacks the information as
        //! `createdTime` refers to the date of the first-ever trade on the pair
        //! in question, whereas `updatedTime` merely provides the request
        //! timestamp. If not provided, uses `updatedTime`.

        let ddate = match date_tran {
            Some(v) => parsers::utils::convert_ts_milis_to_datetime(v),
            None => parsers::utils::convert_ts_milis_to_datetime(self.created_time),
        };
        let piped_side = sellside_to_position(&self.side);
        NewPageObject {
            title: Title::new("trade".to_string()),
            asset: Select::new(self.symbol.clone()),
            entry_price: Number::new(self.avg_price),
            entry_date: Date::new(ddate.to_rfc3339()),
            side: Select::new(piped_side),
            market: Select::new("Cryptocurrency".to_string()),
            exchange: Select::new("Bybit".to_string()),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_string(&self, string: &str) -> Self {
        serde_json::from_str::<PositionInfo>(string).unwrap()
    }
}

fn sellside_to_position(side: &str) -> String {
    if side == "Sell" {
        "Short".into()
    } else {
        "Long".into()
    }
}
