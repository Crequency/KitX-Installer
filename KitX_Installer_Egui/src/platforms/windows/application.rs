use eframe::{
    egui::{self, RichText, Ui},
    epaint::{Color32, Vec2},
};

use crate::data::{data_fetcher, data_validator};

use super::{
    reg_helper,
    translations::{self, get_lang},
};

pub fn get_native_options(size: Option<Vec2>) -> eframe::NativeOptions {
    let size = size.unwrap_or(egui::vec2(800.0, 500.0));
    let mut min_size = size.clone();
    min_size.x = 780.0;
    min_size.y = 480.0;

    let options = eframe::NativeOptions {
        initial_window_size: Some(size),
        centered: true,
        // max_window_size: Some(size),
        min_window_size: Some(min_size),
        // resizable: false,
        ..Default::default()
    };

    options
}

pub struct AppData {
    frame_index: i32,
    frame_per_second: i32,
    lang: translations::Languages,
    lang_selected: bool,
    steps: i32,
    max_steps_count: i32,
    heading_text_font_size: f32,
    catalog_item_font_size: f32,
    basic_button_font_size: f32,
    tip_text_font_size: f32,
    content_text_font_size: f32,
    license_agreed: bool,
    license_url: String,
    license_url_tried: bool,
    license_url_backup: String,
    license_content: Option<String>,
    can_goto_install_config_step: bool,
    can_goto_install_step: bool,
    installation_path: String,
    create_desktop_shortcut: bool,
    create_start_menu_shortcut: bool,
    install_as_portable: bool,
    launch_after_install: bool,
    desktop_path: Option<String>,
    start_menu_path: Option<String>,
    install_progress: f32,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            frame_index: 0,
            frame_per_second: 60,
            lang: translations::Languages::English,
            lang_selected: false,
            steps: 0,
            max_steps_count: 5,
            heading_text_font_size: 28.0,
            catalog_item_font_size: 16.0,
            basic_button_font_size: 16.0,
            tip_text_font_size: 18.0,
            content_text_font_size: 14.0,
            license_agreed: false,
            license_url: "https://raw.githubusercontent.com/Crequency/KitX/main/LICENSE"
                .to_string(),
            license_url_tried: false,
            license_url_backup:
                "https://ghproxy.com/raw.githubusercontent.com/Crequency/KitX/main/LICENSE"
                    .to_string(),
            license_content: None,
            can_goto_install_config_step: false,
            can_goto_install_step: false,
            installation_path: "C:\\Program Files\\Crequency\\KitX\\".to_string(),
            create_desktop_shortcut: false,
            create_start_menu_shortcut: true,
            install_as_portable: false,
            launch_after_install: true,
            desktop_path: None,
            start_menu_path: None,
            install_progress: 0.0,
        }
    }
}

impl AppData {
    fn init(&mut self) {
        self.frame_index = self.frame_index + 1;
        if self.frame_index >= self.frame_per_second {
            self.frame_index = 0;
        }

        if self.lang_selected == false {}

        if self.desktop_path == None {
            self.desktop_path = Some(reg_helper::get_desktop_path().unwrap());
            println!("Desktop directory: {:?}", self.desktop_path);
        }

        if self.start_menu_path == None {
            self.start_menu_path = Some(reg_helper::get_start_menu_path().unwrap());
            println!("Start menu directory: {:?}", self.start_menu_path);
        }

        if self.license_content == None {
            if self.license_url_tried {
                self.license_content = Some(data_fetcher::fetch_string(
                    self.license_url_backup.to_string(),
                ));
            } else {
                self.license_url_tried = true;
                self.license_content =
                    Some(data_fetcher::fetch_string(self.license_url.to_string()));
            }
        }
    }

    fn change_lang(&mut self, lang: translations::Languages) {
        self.lang = lang;
    }

    fn validater(&mut self) {
        self.can_goto_install_config_step = self.license_agreed;

        if data_validator::is_path_legal_in_windows(&self.installation_path) {
            self.can_goto_install_step = true;
        } else {
            self.can_goto_install_step = false;
        }
    }

    fn build_heading_text(&mut self, text: &str) -> RichText {
        RichText::new(text).size(self.heading_text_font_size)
    }

    fn build_catalog_text(&mut self, text: &str) -> RichText {
        RichText::new(text).size(self.catalog_item_font_size)
    }

    fn build_content_text(&mut self, text: &str) -> RichText {
        RichText::new(text).size(self.content_text_font_size)
    }

    fn build_button_text(&mut self, text: &str) -> RichText {
        RichText::new(text).size(self.basic_button_font_size)
    }

    fn draw_steps(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(200.0)
            // .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(self.build_heading_text(&get_lang("steps", &self.lang)));
                    ui.label("");
                    ui.separator();
                    ui.label("");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut catalog_painter = |me: &mut AppData, step: i32, tip: String| {
                        if me.steps > step {
                            let finished = me.build_catalog_text(&tip);
                            ui.label(finished.color(
                                if _frame.info().system_theme.unwrap() == eframe::Theme::Light {
                                    Color32::DARK_GREEN
                                } else {
                                    Color32::LIGHT_GREEN
                                },
                            ));
                        } else if me.steps == step {
                            let executing = me.build_catalog_text(&tip);
                            ui.label(executing.color(
                                if _frame.info().system_theme.unwrap() == eframe::Theme::Light {
                                    Color32::DARK_BLUE
                                } else {
                                    Color32::LIGHT_BLUE
                                },
                            ));
                        } else {
                            ui.label(me.build_catalog_text(&tip));
                        }
                        ui.end_row();
                        ui.label("");
                    };
                    catalog_painter(self, 0, get_lang("0_hello", &self.lang));
                    catalog_painter(self, 1, get_lang("1_license", &self.lang));
                    catalog_painter(self, 2, get_lang("2_install_config", &self.lang));
                    catalog_painter(self, 3, get_lang("3_install", &self.lang));
                    catalog_painter(self, 4, get_lang("4_finish", &self.lang));
                });
            });
    }

    fn draw_bottom_panel(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .default_height(40.0)
            .show_inside(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let previous = self.build_button_text(&get_lang("previous", &self.lang));
                    let next = self.build_button_text(&get_lang("next", &self.lang));
                    let install = self.build_button_text(&get_lang("install", &self.lang));
                    let cancle = self.build_button_text(&get_lang("cancel", &self.lang));
                    let finish = self.build_button_text(&get_lang("finish", &self.lang));

                    if self.steps < 2 {
                        if self.steps == 0 || self.can_goto_install_config_step {
                            if ui.button(next).clicked() {
                                if self.steps < self.max_steps_count - 1 {
                                    self.steps = self.steps + 1;
                                }
                            }
                        }

                        if self.steps != 0 {
                            if ui.button(previous).clicked() {
                                if self.steps > 0 {
                                    self.steps = self.steps - 1;
                                }
                            }
                        }
                    } else if self.steps == 2 {
                        if self.can_goto_install_step {
                            if ui.button(install).clicked() {
                                self.steps = self.steps + 1;
                            }
                        }
                        if ui.button(previous).clicked() {
                            if self.steps > 0 {
                                self.steps = self.steps - 1;
                            }
                        }
                    } else if self.steps == 3 {
                        if ui.button(cancle).clicked() {
                            self.steps = self.steps - 1;
                        }
                        if cfg!(debug_assertions) {
                            if ui.button("debug: next").clicked() {
                                self.steps = self.steps + 1;
                            }
                        }
                    } else {
                        if ui.button(finish).clicked() {
                            _frame.close();
                        }
                    }
                });
            });
    }

    fn draw_body(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(self.build_heading_text(&get_lang("kitx_installer", &self.lang)));
            ui.label("");

            if self.steps == 0 {
                self.draw_body_hello(ui);
            } else if self.steps == 1 {
                self.draw_body_license(ui);
            } else if self.steps == 2 {
                self.draw_body_installation_config(ui);
            } else if self.steps == 3 {
                self.draw_body_installation(ui);
            } else if self.steps == 4 {
                self.draw_body_finished(ui);
            }
        });
    }

    fn draw_body_hello(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.label(self.build_content_text(&get_lang("0_intro", &self.lang)));
            ui.label(self.build_content_text(&get_lang("0_install_on_your_device", &self.lang)));
            ui.label("    ");
            ui.label(self.build_content_text(&get_lang("0_connect_internet", &self.lang)));
            ui.label(self.build_content_text(&get_lang("0_traffic_charges", &self.lang)));
            ui.label("    ");
            ui.horizontal_wrapped(|ui| {
                ui.label(self.build_content_text(&get_lang("0_fetch_src_codes", &self.lang)));
                ui.hyperlink_to(
                    self.build_content_text(&get_lang("github", &self.lang)),
                    "https://github.com/Crequency/KitX",
                );
                ui.label(self.build_content_text("."));
            });
            ui.horizontal_wrapped(|ui| {
                ui.label(self.build_content_text(&get_lang("0_visit_home_page", &self.lang)));
                ui.hyperlink_to(
                    self.build_content_text(&get_lang("home_page", &self.lang)),
                    "https://kitx.apps.catrol.cn",
                );
                ui.label(self.build_content_text(&get_lang("0_for_more", &self.lang)));
            });
        });
    }

    fn draw_body_license(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            let license_content = self
                .license_content
                .clone()
                .unwrap_or(get_lang("fetching", &self.lang))
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
                            ui.label(self.build_content_text(
                                license_content_lines.clone().nth(row).unwrap(),
                            ));
                        }
                    },
                );
            ui.end_row();
        });

        ui.add_space(10.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let agreement = self.build_content_text(&get_lang("1_agree", &self.lang));

            ui.label("    ");
            ui.checkbox(&mut self.license_agreed, agreement);
            ui.end_row();
        });
    }

    fn draw_body_installation_config(&mut self, ui: &mut Ui) {
        egui::Grid::new("installation_config_path_grid")
            .num_columns(3)
            // .spacing([40.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("");
                ui.label(self.build_content_text(&get_lang("2_install_path", &self.lang)));
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.installation_path)
                            .hint_text("C:\\Program Files\\Crequency\\KitX"),
                    );
                    if ui.button("...").clicked() {}
                });
                ui.end_row();
                ui.end_row();
            });

        ui.label("");
        ui.label("");

        egui::Grid::new("installation_config_options_grid")
            .num_columns(3)
            // .spacing([40.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                let desktop_shortcut =
                    self.build_content_text(&get_lang("2_create_desktop_shortcut", &self.lang));
                let start_menu_shortcut =
                    self.build_content_text(&get_lang("2_create_start_menu_shortcut", &self.lang));
                let portable_install =
                    self.build_content_text(&get_lang("2_install_as_portable_mode", &self.lang));
                let launch_after_install =
                    self.build_content_text(&get_lang("2_launch_after_install", &self.lang));

                ui.label("");
                ui.checkbox(&mut self.create_desktop_shortcut, desktop_shortcut);
                ui.end_row();

                ui.label("");
                if self.desktop_path.is_none() {
                    ui.label(get_lang("2_fetch_desktop_path_failed", &self.lang));
                } else {
                    ui.label(format!("({})", self.desktop_path.as_ref().unwrap()));
                }
                ui.end_row();
                ui.end_row();

                ui.label("");
                ui.checkbox(&mut self.create_start_menu_shortcut, start_menu_shortcut);
                ui.end_row();

                ui.label("");
                if self.start_menu_path.is_none() {
                    ui.label(get_lang("2_fetch_start_menu_path_failed", &self.lang));
                } else {
                    ui.label(format!("({})", self.start_menu_path.as_ref().unwrap()));
                }
                ui.end_row();
                ui.end_row();

                ui.label("");
                ui.checkbox(&mut self.install_as_portable, portable_install);
                ui.end_row();
                ui.end_row();

                ui.label("");
                ui.checkbox(&mut self.launch_after_install, launch_after_install);
                ui.end_row();
                ui.end_row();
            });
    }

    fn draw_body_installation(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label(
                    egui::RichText::new(get_lang("3_installing", &self.lang))
                        .size(self.tip_text_font_size),
                );
                ui.add(
                    egui::ProgressBar::new(self.install_progress)
                        .animate(false)
                        .show_percentage()
                        .desired_width(460.0),
                );
            });
            ui.end_row();
        });
    }

    fn draw_body_finished(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(self.build_content_text("    "));
            ui.label(self.build_content_text(&get_lang("4_installed", &self.lang)));
            if self.launch_after_install {
                ui.label(self.build_content_text(&get_lang("4_auto_launch", &self.lang)));
            } else {
                ui.label(self.build_content_text(&get_lang("4_manually_launch", &self.lang)));
            }
            ui.label(self.build_content_text("    "));
        });
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.init();

        // if _frame.info().window_info.maximized {
        //     _frame.set_window_size(egui::vec2(800.0, 500.0));
        // }
        // _frame.set_window_size(get_native_options(None).initial_window_size.unwrap_or(egui::vec2(800.0, 500.0)));

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_steps(ui, _frame);
            self.draw_bottom_panel(ui, _frame);
            self.draw_body(ui);

            self.validater();
        });
    }
}
