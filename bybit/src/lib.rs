use crate::structs::Credentials::Credentials;
use crate::structs::PositionInfo::PositionInfo;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Response;
use sha2::Sha256;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, vec};
use structs::TradeRecord::TradeRecord;

pub mod parsers;
mod structs;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub struct Client {
    url: &'static str,
    pub auth: Credentials,
}

impl Client {
    pub fn new(auth: Credentials) -> Self {
        let url = "https://api.bybit.com";
        Self { url, auth }
    }

    pub async fn fetch_transaction_log(&self, symbol: &str) -> Vec<TradeRecord> {
        // https://bybit-exchange.github.io/docs/derivatives/contract/execution-list
        let endpoint = "/contract/v3/private/execution/list";
        let query_string = "?symbol=".to_string() + symbol;
        let res = self.make_request(query_string, endpoint).await;
        let text = res.text().await.unwrap();

        let raw_json: serde_json::Value = serde_json::from_str(&text).unwrap();

        let arr: &Vec<serde_json::Value> = raw_json
            .get("result")
            .unwrap()
            .get("list")
            .unwrap()
            .as_array()
            .unwrap();

        parsers::trade_records::build_trade_records(arr)
    }

    pub async fn fetch_closed_pnl(self) {
        //https://bybit-exchange.github.io/docs/derivatives/contract/closepnl
        let endpoint = "/contract/v3/private/position/closed-pnl";
        todo!();
    }

    async fn make_request(&self, query_string: String, endpoint: &str) -> Response {
        let url = self.url.to_string() + endpoint + &query_string;

        let client = reqwest::Client::new();
        let headers = self.build_headers(&query_string);

        client.get(url).headers(headers).send().await.unwrap()
    }

    pub async fn fetch_current_positions(&self) -> Vec<PositionInfo> {
        let assets_to_check: Vec<&str> = vec!["BTC", "EOS", "USDT"];
        let mut v: Vec<PositionInfo> = Vec::new();

        for asset in assets_to_check {
            let temp = self.fetch_current_position(asset).await;
            v.extend(temp);
        }

        v
    }

    pub async fn fetch_current_position(&self, settle_coin: &str) -> Vec<PositionInfo> {
        let endpoint = "/contract/v3/private/position/list";
        let query_string = "?settleCoin=".to_string() + settle_coin;

        let res = self.make_request(query_string, endpoint).await;

        parsers::pos_infos::parse_response(res).await
    }

    fn get_current_time(&self) -> u64 {
        let mut now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        now = now * 1000_f64;

        now as u64
    }

    fn get_signature(
        &self,
        query_string: &str,
        recv_window: &String,
        time_stamp: String,
    ) -> String {
        let auth = &self.auth;
        let cleaned_params: String = query_string.chars().filter(|c| *c != '?').collect();
        let param_str = time_stamp + &auth.api_key + recv_window + cleaned_params.as_str();

        let mut mac = HmacSha256::new_from_slice(auth.api_secret.as_bytes())
            .expect("Failed to initialize mac");
        mac.update(param_str.as_bytes());
        let result = mac.finalize();

        hex::encode(result.into_bytes())
    }

    fn build_headers(&self, query_string: &str) -> HeaderMap {
        let auth = &self.auth;
        let mut headers = HeaderMap::new();
        let time_stamp = self.get_current_time();
        let recv_window = "5000".to_string();
        let sign = &self.get_signature(query_string, &recv_window, time_stamp.to_string());

        headers.insert("X-BAPI-API-KEY", auth.api_key.parse().unwrap());
        headers.insert("X-BAPI-SIGN", sign.parse().unwrap());
        headers.insert("X-BAPI-SIGN-TYPE", "2".parse().unwrap());
        headers.insert("X-BAPI-TIMESTAMP", time_stamp.to_string().parse().unwrap());
        headers.insert("X-BAPI-RECV-WINDOW", recv_window.parse().unwrap());
        headers.insert(CONTENT_TYPE, "applicaition/json".parse().unwrap());

        headers
    }
}

pub fn fetch_credentials() -> Credentials {
    dotenv().ok();
    let api_key = env::var("bybit_api_key").unwrap();
    let api_secret = env::var("bybit_api_secret").unwrap();

    Credentials {
        api_key,
        api_secret,
    }
}

#[cfg(test)]
mod tests {}
