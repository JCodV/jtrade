use crate::jtrader;
use serde::Deserialize;
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

pub fn format_url(stock: &jtrader::Ticker, date: &str, api_key: &String) -> String {
    let s = format!(
        "https://api.polygon.io/v1/open-close/{:?}/{}?adjusted=true&apiKey={}",
        stock, date, api_key
    );

    println!("{s}");
    s
}
