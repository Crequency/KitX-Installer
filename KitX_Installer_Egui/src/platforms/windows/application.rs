use eframe::{
    egui::{self, RichText},
    epaint::{Color32, Vec2},
};

use crate::data::data_fetcher;

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
    heading_text_font_size: f32,
    catalog_item_font_size: f32,
    basic_button_font_size: f32,
    license_agreed: bool,
    license_url: String,
    license_url_backup: String,
    license_content: Option<String>,
    can_goto_install_config_step: bool,
    can_goto_install_step: bool,
    installation_path: String,
    create_desktop_shortcut: bool,
    create_start_menu_shortcut: bool,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            steps: 0,
            max_steps_count: 5,
            heading_text_font_size: 28.0,
            catalog_item_font_size: 16.0,
            basic_button_font_size: 16.0,
            license_agreed: false,
            license_url: "https://raw.githubusercontent.com/Crequency/KitX/main/LICENSE"
                .to_string(),
            license_url_backup:
                "https://ghproxy.com/raw.githubusercontent.com/Crequency/KitX/main/LICENSE"
                    .to_string(),
            license_content: None,
            can_goto_install_config_step: false,
            can_goto_install_step: false,
            installation_path: "C:\\Program Files\\Crequency\\KitX\\".to_string(),
            create_desktop_shortcut: false,
            create_start_menu_shortcut: true,
        }
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                        ui.heading(RichText::new("Steps").size(self.heading_text_font_size));
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
                                        .size(self.catalog_item_font_size),
                                );
                            } else if self.steps == step {
                                ui.label(
                                    RichText::new(tip)
                                        .color(Color32::LIGHT_BLUE)
                                        .size(self.catalog_item_font_size),
                                );
                            } else {
                                ui.label(RichText::new(tip).size(self.catalog_item_font_size));
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
                            self.can_goto_install_config_step = self.license_agreed;

                            if self.steps == 0 || self.can_goto_install_config_step {
                                if ui
                                    .button(
                                        RichText::new("Next >").size(self.basic_button_font_size),
                                    )
                                    .clicked()
                                {
                                    if self.steps < self.max_steps_count - 1 {
                                        self.steps = self.steps + 1;
                                    }
                                }
                            }

                            if self.steps != 0 {
                                if ui
                                    .button(
                                        RichText::new("< Previous")
                                            .size(self.basic_button_font_size),
                                    )
                                    .clicked()
                                {
                                    if self.steps > 0 {
                                        self.steps = self.steps - 1;
                                    }
                                }
                            }
                        } else if self.steps == 2 {
                            if ui
                                .button(RichText::new("Install").size(self.basic_button_font_size))
                                .clicked()
                            {
                                self.steps = self.steps + 1;
                            }
                            if ui
                                .button(
                                    RichText::new("< Previous").size(self.basic_button_font_size),
                                )
                                .clicked()
                            {
                                if self.steps > 0 {
                                    self.steps = self.steps - 1;
                                }
                            }
                        } else if self.steps == 3 {
                            if ui
                                .button(RichText::new("Cancel").size(self.basic_button_font_size))
                                .clicked()
                            {
                                self.steps = self.steps - 1;
                            }
                            if cfg!(debug_assertions) {
                                if ui.button("debug: next").clicked() {
                                    self.steps = self.steps + 1;
                                }
                            }
                        } else {
                            if ui
                                .button(RichText::new("Finish").size(self.basic_button_font_size))
                                .clicked()
                            {
                                _frame.close();
                            }
                        }
                    });
                });

            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("KitX Installer").size(self.heading_text_font_size));
                ui.label("");

                if self.steps == 0 {
                    ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                        ui.label(
                            "    Welcome to KitX Project! You are running the KitX Installer.",
                        );
                        ui.label(
                            "    This installer will install KitX Dashboard into your device.",
                        );
                        ui.label("    ");
                        ui.label(
                            "    This is a online installer, you need to connect to the internet.",
                        );
                        ui.label(
                            "    We are not responsible for the traffic charges incurred during the installation process."
                        );
                        ui.label("    ");
                        ui.horizontal_wrapped(|ui| {
                            ui.label("    You can fetch all source code via");
                            ui.hyperlink_to("Github", "https://github.com/Crequency/KitX");
                            ui.label(".")
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.label("    Visit our");
                            ui.hyperlink_to("Home Page", "https://kitx.apps.catrol.cn");
                            ui.label("for more.")
                        });
                    });
                } else if self.steps == 1 {
                    if self.license_content == None {
                        self.license_content =
                            Some(data_fetcher::fetch_string(self.license_url.to_string()));
                    }

                    ui.vertical_centered(|ui| {
                        let license_content = self
                            .license_content
                            .clone()
                            .unwrap_or("Fetching ...".to_string())
                            .to_string();
                        let license_content_lines = license_content.split('\n');

                        let text_style = egui::TextStyle::Body;
                        let row_height = ui.text_style_height(&text_style);
                        egui::ScrollArea::vertical()
                            .auto_shrink([false; 2])
                            .show_rows(
                                ui,
                                row_height,
                                license_content_lines.clone().count(),
                                |ui, row_range| {
                                    for row in row_range {
                                        ui.label(license_content_lines.clone().nth(row).unwrap());
                                    }
                                },
                            );
                        ui.end_row();
                    });

                    ui.add_space(10.0);

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.label("    ");
                        ui.checkbox(
                            &mut self.license_agreed,
                            "  I agree to the terms of the license agreement.",
                        );
                        ui.end_row();
                    });
                } else if self.steps == 2 {
                    egui::Grid::new("my_grid")
                        .num_columns(3)
                        // .spacing([40.0, 4.0])
                        .striped(false)
                        .show(ui, |ui| {
                            ui.label("");
                            ui.label("Installation path: ");
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.installation_path)
                                );
                                if ui.button("...").clicked() {

                                }
                            });
                            ui.end_row();
                            ui.end_row();
                            ui.label("");
                            ui.checkbox(&mut self.create_desktop_shortcut, "Create desktop shortcut.");
                            ui.end_row();
                            ui.end_row();
                            ui.label("");
                            ui.checkbox(&mut self.create_start_menu_shortcut, "Create start menu shortcut.");
                            ui.end_row();
                            ui.end_row();
                        });
                } else if self.steps == 3 {
                } else if self.steps == 4 {
                }
            });
        });
    }
}
