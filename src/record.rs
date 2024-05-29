use eframe::egui;

pub fn record_ui(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
        // ui.label("123");
    });
}
