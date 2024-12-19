use crate::daily_open_close::{self, DailyOpenClose};
use dotenv::dotenv;
use std::env;

#[derive(PartialEq, Debug)]
pub enum Ticker {
    UNDEFINED,
    APPL,
    NVDA,
    GOOG,
    TSLA,
}

pub enum IncreaseOrDecrease {
    UNCHANGED,
    INCREASE,
    DECREASE,
}

pub struct StockStats {
    inc_or_dec: IncreaseOrDecrease,
    percent_change: f32,
}

impl Default for StockStats {
    fn default() -> Self {
        StockStats {
            inc_or_dec: IncreaseOrDecrease::UNCHANGED,
            percent_change: 0.0,
        }
    }
}

pub struct JTrader {
    api_key: String,
    selected_stock: Ticker,
    doc_stock_data: Vec<DailyOpenClose>,

    // stats
    stats_yesterday: StockStats,
    stats_week: StockStats,
    stats_month: StockStats,
    stats_year: StockStats,
}

impl Default for JTrader {
    fn default() -> Self {
        JTrader {
            api_key: get_api_key(),
            selected_stock: Ticker::UNDEFINED,
            doc_stock_data: Vec::new(),

            stats_yesterday: StockStats::default(),
            stats_week: StockStats::default(),
            stats_month: StockStats::default(),
            stats_year: StockStats::default(),
        }
    }
}

impl JTrader {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        JTrader::default()
    }

    async fn get_selected_stock_data(&mut self) {
        let url = daily_open_close::format_url(&self.selected_stock, "2024-12-10", &self.api_key);

        match get_request(&url).await {
            Ok(value) => {
                println!("{:?}", &value);
                self.doc_stock_data.push(value);
            }
            Err(e) => println!("Error with sending get request for stock, {}", e),
        }
    }
}

impl eframe::App for JTrader {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World");

            // stock selection
            egui::ComboBox::from_label("Select Stock")
                .selected_text(format!("{:?}", self.selected_stock))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_stock, Ticker::APPL, "APPL");
                    ui.selectable_value(&mut self.selected_stock, Ticker::NVDA, "NVDA");
                    ui.selectable_value(&mut self.selected_stock, Ticker::GOOG, "GOOG");
                    ui.selectable_value(&mut self.selected_stock, Ticker::TSLA, "TSLA");
                });

            if ui.add(egui::Button::new("Run")).clicked()
                && self.selected_stock != Ticker::UNDEFINED
            {
                println!("{:?}", self.selected_stock);
            }

            if ui.add(egui::Button::new("Test")).clicked() {
                for stock in self.doc_stock_data.iter() {
                    println!("{:?}", stock);
                }
            }
        });
    }
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

async fn get_request(url: &String) -> Result<DailyOpenClose, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<DailyOpenClose>().await?;
    Ok(response)
}
