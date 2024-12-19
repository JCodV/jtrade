pub mod daily_open_close;
pub mod jtrader;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "JTrader",
        native_options,
        Box::new(|cc| Ok(Box::new(jtrader::JTrader::new(cc)))),
    );
}
