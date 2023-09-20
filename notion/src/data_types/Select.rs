use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Select {
    select: SelectContent,
}

#[derive(Debug, Deserialize, Serialize)]
struct SelectContent {
    name: String,
}


impl Select {
    pub fn new(_name: String) -> Self {
        Self {
            select: SelectContent { name: _name },
        }
    }
}
