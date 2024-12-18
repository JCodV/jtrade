use dotenv::dotenv;
use reqwest::{self, Error, Url};
use serde_json;
use std::env;

pub mod daily_open_close;
use daily_open_close::DailyOpenClose;

pub mod main_gui;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = get_api_key();
    let symbols: Vec<&str> = vec!["AAPL", "TSLA", "NVDA"];

    for symbol in symbols.iter() {
        let url = daily_open_close::format_url(symbol, "2024-12-10", &api_key);
        let response = reqwest::get(Url::parse(&url).unwrap())
            .await?
            .text()
            .await?;
        let test: DailyOpenClose = serde_json::from_str(&response).unwrap();
        println!("{:?}", test);
    }

    main_gui::test();

    Ok(())
}

fn get_api_key() -> String {
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
    polygon_api_key
}
