use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// * Date
#[derive(Debug, Deserialize, Serialize)]
pub struct Date {
    date: DateStart,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DateStart {
    start: String,
}

impl Date {
    pub fn new(_date: String) -> Self {
        Self {
            date: DateStart { start: _date },
        }
    }
}