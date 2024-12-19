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

pub struct JTrader {
    api_key: String,
    selected_stock: Ticker,
    doc_stock_data: Vec<DailyOpenClose>,
}

impl Default for JTrader {
    fn default() -> Self {
        JTrader {
            api_key: get_api_key(),
            selected_stock: Ticker::UNDEFINED,
            doc_stock_data: Vec::new(),
        }
    }
}

impl JTrader {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        JTrader::default()
    }

    fn get_data_for_stock(self) {
        let url = daily_open_close::format_url(&self.selected_stock, "2024-12-10", &self.api_key);
        let response = self.get_request(&url);
    }

    async fn get_request(self, url: &String) {
        let response = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("{response}");
    }
}

impl eframe::App for JTrader {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World");

            // stock selection
            egui::ComboBox::from_label("Select Stock")
                .selected_text(format!("{:?}", self.selected_stock))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_stock, Ticker::APPL, "APPL")
                });

            if ui.add(egui::Button::new("Run")).clicked()
                && self.selected_stock != Ticker::UNDEFINED
            {
                println!("{:?}", self.selected_stock);
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
