use eframe::{egui, epaint::Vec2};

pub fn get_native_options(size: Option<Vec2>) -> eframe::NativeOptions {
    let size = size.unwrap_or(egui::vec2(800.0, 500.0));

    let options = eframe::NativeOptions {
        initial_window_size: Some(size),
        centered: true,
        max_window_size: Some(size),
        min_window_size: Some(size),
        resizable: false,
        ..Default::default()
    };

    options
}

pub struct AppData {}

impl Default for AppData {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });

            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }

            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            //     ui.heading("Right-to-left layout");
            // });

            ui.heading("KitX Installer");
        });
    }
}
