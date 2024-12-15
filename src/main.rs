use std::env;
use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;
use serde_json;

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

enum Url {
    DailyOpenClose(String),
    PreviousClose(String),
    Trades(String)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let symbols: Vec<&str> = vec!["AAPL", "TSLA", "NVDA"];

    dotenv().ok();
    let polygon_api = env::var("POLYGON_KEY");
    let url = "https://api.polygon.io/v1/open-close/AAPL/2024-12-13?adjusted=true&apiKey=gsM8ccIfernPglYyzOUF0xewqbp19c8C";

    let polygon_api_key = match polygon_api {
        Ok(value) => println!("The Polygon API key is : {:}", value),
        Err(e) => println!("Failed to get Polygon API key!, {}", e),
    };

    let response = reqwest::get(&url).await?;

    let post: DailyOpenClose = serde_json::from_str(&body).expect("Failed to deserialize json");
    println!("Received post:\n{:#?}", post);
    Ok(())
}
