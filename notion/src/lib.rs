use crate::structs::Credentials::Credentials;
use crate::structs::RequestJSON::{DatabaseJSON, RequestJSON};
use crate::structs::TradeEntity::TradeEntity;
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Response,
};
use std::env;

pub mod data_types;
mod parser;
mod structs;

pub fn fetch_credentials() -> Credentials {
    dotenv().ok();
    let api_key = env::var("notion_api_key").unwrap();
    let db_id = env::var("notion_db_id").unwrap();

    Credentials { api_key, db_id }
}

pub struct Client {
    url: &'static str,
    pub auth: Credentials,
}

impl Client {
    pub fn new(auth: Credentials) -> Self {
        let url = "https://api.notion.com/v1";

        Client { url, auth }
    }

    pub async fn get_db_data(&self) -> Vec<TradeEntity> {
        let endpoint: String = "/databases/".to_string() + &self.auth.db_id + "/query";
        let res = self.make_request(endpoint.as_str()).await;
        let to_process = res.text().await.unwrap();

        parser::parse_trade_entities(to_process)
    }

    pub async fn create_new_page<T>(&self, new_page: T) -> Result<String, Response>
    where
        T: serde::Serialize,
    {
        let endpoint = "/pages";
        let res = self.make_request_json(endpoint, new_page).await;

        match res.status() {
            reqwest::StatusCode::OK => Ok(res.text().await.unwrap()),
            _ => Err(res),
        }
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth = "Bearer ".to_string() + &self.auth.api_key;

        headers.insert(AUTHORIZATION, auth.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("Notion-Version", "2022-06-28".parse().unwrap());

        headers
    }

    fn append_db_id_to_json<T>(&self, initial_json: T) -> RequestJSON<T>
    where
        T: serde::Serialize,
    {
        RequestJSON {
            parent: DatabaseJSON {
                database_id: self.auth.db_id.clone(),
            },
            properties: initial_json,
        }
    }

    async fn make_request(&self, endpoint: &str) -> Response {
        let url = self.url.to_string() + endpoint;
        let client = reqwest::Client::new();
        let headers = self.build_headers();

        client.post(url).headers(headers).send().await.unwrap()
    }

    async fn make_request_json<T>(&self, endpoint: &str, object: T) -> Response
    where
        T: serde::Serialize + std::marker::Sized,
    {
        // Sens a post request.
        let url = self.url.to_string() + endpoint;
        let client = reqwest::Client::new();
        let headers = self.build_headers();
        let json = self.append_db_id_to_json(object);

        client
            .post(url)
            .headers(headers)
            .json(&json)
            .send()
            .await
            .unwrap()
    }
}

#[cfg(test)]
mod tests {}
