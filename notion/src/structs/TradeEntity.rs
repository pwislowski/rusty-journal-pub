use chrono::{DateTime, FixedOffset};
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TradeEntity {
    //properties
    pub trade_type: Option<String>,
    pub market_structure: Option<Vec<String>>,
    pub exit_price: Option<f64>,
    pub entry_price: Option<f64>,
    pub win: Option<bool>,
    pub side: Option<String>,
    pub exchange: Option<String>,
    pub asset: Option<String>,
    pub entry_model: Option<String>,
    pub confusion_matrix: Option<String>,
    pub used_orderflow: Option<bool>,
    pub confluences: Option<Vec<String>>,
    pub improvements: Option<Vec<String>>,
    pub market: Option<String>,
    pub entry_date: Option<DateTime<FixedOffset>>,
    pub exit_date: Option<DateTime<FixedOffset>>,
    pub is_open: Option<bool>,
    pub stop_loss: Option<f64>,
}
