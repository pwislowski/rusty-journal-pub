use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Number {
    number: f64,
}

impl Number {
    pub fn new(number: f64) -> Self {
        Self { number }
    }
}
