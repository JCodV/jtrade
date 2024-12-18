use dotenv::dotenv;
use reqwest::{self, Error, Url};
use serde::Deserialize;
use serde_json;
use std::env;

#[derive(Deserialize, Debug)]
pub struct DailyOpenClose {
    #[serde(rename = "afterHours")]
    after_hours: f32,
    close: f32,
    from: String,
    high: f32,
    low: f32,
    #[serde(rename = "preMarket")]
    pre_market: f32,
    status: String,
    volume: f32,
    symbol: String,
}

pub fn format_url(stock_symbol: &str, date: &str, polygon_api_key: &String) -> String {
    let s = format!(
        "https://api.polygon.io/v1/open-close/{}/{}?adjusted=true&apiKey={}",
        stock_symbol, date, polygon_api_key
    );

    println!("{s}");
    s
}
