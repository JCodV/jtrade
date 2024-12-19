use eframe::egui;
use std::fmt;
pub struct JTrader {
    selected_stock: Tickers,
    checked: bool,
}

impl Default for JTrader {
    fn default() -> Self {
        Self {
            selected_stock: self::Tickers::default(),
            checked: false,
        }
    }
}

impl JTrader {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

#[derive(Debug, PartialEq)]
enum Tickers {
    UNDEFINED,
    AAPL,
    NVDA,
    TSLA,
}

impl Default for Tickers {
    fn default() -> Self {
        Tickers::UNDEFINED
    }
}

impl eframe::App for JTrader {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            egui::ComboBox::from_label("Select stock")
                .selected_text(format!("{:?}", self.selected_stock))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_stock, Tickers::AAPL, "AAPL");
                    ui.selectable_value(&mut self.selected_stock, Tickers::NVDA, "NVDA");
                    ui.selectable_value(&mut self.selected_stock, Tickers::TSLA, "TSLA");
                });

            println!("current selection: {:?}", &self.selected_stock);
        });
    }
}
