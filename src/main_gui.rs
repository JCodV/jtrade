use egui;

pub fn test() {
    let mut ctx = egui::Context::default();
    egui::Window::new("My Window").show(&ctx, |ui| {
        ui.label("Hello World!");
    });
}
