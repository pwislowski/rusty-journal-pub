use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct RequestJSON<T: serde::Serialize> {
    pub parent: DatabaseJSON,
    pub properties: T,
}

#[derive(Debug, Serialize)]
pub struct DatabaseJSON {
    pub database_id: String,
}
