use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Title {
    title: Vec<TitleText>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TitleText {
    text: TitleContent,
}

#[derive(Debug, Deserialize, Serialize)]
struct TitleContent {
    content: String,
}

impl Title {
    pub fn new(_title: String) -> Self {
        Self {
            title: vec![TitleText {
                text: TitleContent { content: _title },
            }],
        }
    }
}
