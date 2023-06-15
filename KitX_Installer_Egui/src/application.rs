use eframe::{
    egui::{self, RichText},
    epaint::{Color32, Vec2},
};

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

pub struct AppData {
    steps: i32,
    max_steps_count: i32,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            steps: 0,
            max_steps_count: 5,
        }
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let heading_text_font_size = 28.0;
        let catalog_item_font_size = 16.0;
        let basic_button_font_size = 16.0;

        // if _frame.info().window_info.maximized {
        //     _frame.set_window_size(egui::vec2(800.0, 500.0));
        // }
        // _frame.set_window_size(get_native_options(None).initial_window_size.unwrap_or(egui::vec2(800.0, 500.0)));

        egui::CentralPanel::default().show(ctx, |ui| {

            egui::SidePanel::left("left_panel")
                .resizable(false)
                .default_width(200.0)
                // .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(RichText::new("Catalog").size(heading_text_font_size));
                        ui.label("");
                        ui.separator();
                        ui.label("");
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let mut catalog_painter = |step: i32, tip: String| {
                            if self.steps > step {
                                ui.label(
                                    RichText::new(tip)
                                        .color(Color32::LIGHT_GREEN)
                                        .size(catalog_item_font_size),
                                );
                            } else if self.steps == step {
                                ui.label(
                                    RichText::new(tip)
                                        .color(Color32::LIGHT_BLUE)
                                        .size(catalog_item_font_size),
                                );
                            } else {
                                ui.label(RichText::new(tip).size(catalog_item_font_size));
                            }
                            ui.end_row();
                            ui.label("");
                        };
                        catalog_painter(0, "0. Hello".to_string());
                        catalog_painter(1, "1. License".to_string());
                        catalog_painter(2, "2. Installation Config".to_string());
                        catalog_painter(3, "3. Installing".to_string());
                        catalog_painter(4, "4. Finished".to_string());
                    });
                });

            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(false)
                .min_height(0.0)
                .default_height(40.0)
                .show_inside(ui, |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.steps < 2 {
                            if ui
                                .button(RichText::new("Next >").size(basic_button_font_size))
                                .clicked()
                            {
                                if self.steps < self.max_steps_count - 1 {
                                    self.steps = self.steps + 1;
                                }
                            }
                            if ui
                                .button(RichText::new("< Previous").size(basic_button_font_size))
                                .clicked()
                            {
                                if self.steps > 0 {
                                    self.steps = self.steps - 1;
                                }
                            }
                        } else if self.steps == 2 {
                            if ui
                                .button(RichText::new("Install").size(basic_button_font_size))
                                .clicked()
                            {
                                self.steps = self.steps + 1;
                            }
                            if ui
                                .button(RichText::new("< Previous").size(basic_button_font_size))
                                .clicked()
                            {
                                if self.steps > 0 {
                                    self.steps = self.steps - 1;
                                }
                            }
                        } else if self.steps == 3 {
                            if ui.button(RichText::new("Cancel").size(basic_button_font_size)).clicked() {
                                self.steps = self.steps - 1;
                            }
                            // TODO: Remove in release.
                            if ui.button("debug: next").clicked() {
                                self.steps = self.steps + 1;
                            }
                        } else {
                            if ui.button(RichText::new("Finish").size(basic_button_font_size)).clicked() {
                                _frame.close();
                            }
                        }
                    });
                });

            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("KitX Installer").size(heading_text_font_size));
                ui.label("");
            });
        });
    }
}
