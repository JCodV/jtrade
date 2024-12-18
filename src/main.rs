use dotenv::dotenv;
use reqwest::{self, Error};
use serde::Deserialize;
use serde_json;
use std::env;

#[derive(Deserialize, Debug)]
struct DailyOpenClose {
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    // api key access
    dotenv().ok();
    let polygon_api = env::var("POLYGON_KEY");
    let polygon_api_key = match polygon_api {
        Ok(key) => {
            println!("The Polygon API key is : {:}", key);
            key
        }
        Err(e) => {
            println!("Failed to get Polygon API key!, {}", e);
            e.to_string()
        }
    };

    // add symbols here
    let symbols: Vec<&str> = vec!["AAPL", "TSLA", "NVDA"];

    for symbol in symbols.iter() {
        let url = daily_open_close(symbol, "2024-12-10", &polygon_api_key);
        let response = reqwest::get(url).await?.text().await?;
        let test: DailyOpenClose = serde_json::from_str(&response).unwrap();

        println!("{:?}", test);
    }

    Ok(())
}

fn daily_open_close(stock_symbol: &str, date: &str, polygon_api_key: &String) -> String {
    let s = format!(
        "https://api.polygon.io/v1/open-close/{}/{}?adjusted=true&apiKey={}",
        stock_symbol, date, polygon_api_key
    );

    println!("{s}");
    s
}
