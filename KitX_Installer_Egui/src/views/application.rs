use std::{sync::mpsc, thread::JoinHandle};

use eframe::{
    egui::{self, RichText, Ui},
    epaint::{Color32, Vec2},
    IconData,
};

use crate::{
    data::{
        data_fetcher, data_validator, download_config::DownloadConfig,
        install_config::InstallConfig,
    },
    platforms::windows::win_installer,
};

use super::translations::{self, get_lang, Languages};

// Load icon from include bytes in rgba8 mode.
// Return `IconData`.
pub fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../../assets/icon.ico");
        let image = image::load_from_memory(icon)
            .expect("! Failed to load icon to memery")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

// Get native options for the application.
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
        icon_data: Some(load_icon()),
        ..Default::default()
    };

    options
}

pub struct AppData {
    init: bool,
    frame_index: i32,
    frame_per_second: i32,
    lang: Languages,
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
    license_url_backup_tried: bool,
    license_content: Option<String>,
    license_content_fetched: bool,
    can_goto_install_config_step: bool,
    can_goto_install_step: bool,
    install_config: InstallConfig,
    install_details_visibility: bool,
    install_thread_handle: Option<JoinHandle<()>>,
    download_config: DownloadConfig,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            init: false,
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
            license_url_backup_tried: false,
            license_content: None,
            license_content_fetched: false,
            can_goto_install_config_step: false,
            can_goto_install_step: false,
            install_config: InstallConfig::default(),
            install_details_visibility: false,
            install_thread_handle: None,
            download_config: DownloadConfig::default(),
        }
    }
}

impl AppData {
    fn init(&mut self) {
        if self.init {
            return;
        }

        if cfg!(target_os = "windows") {
            self.install_config.windows_config.init();
        }

        if self.license_content == None {
            if self.license_url_tried {
                if self.license_url_backup_tried {
                    let tip =
                        "# Fetching license content failed, please check your network connection.";

                    println!("{}", tip);

                    self.license_content = Some(tip.to_string());
                    self.license_content_fetched = false;
                } else {
                    println!(
                        "# Fetching license content from {}",
                        self.license_url_backup
                    );

                    self.license_url_backup_tried = true;
                    self.license_content =
                        data_fetcher::fetch_string(self.license_url_backup.to_string(), 3 * 1000);

                    self.license_content_fetched = true;
                }
            } else {
                println!("# Fetching license content from {}", self.license_url);

                self.license_url_tried = true;
                self.license_content =
                    data_fetcher::fetch_string(self.license_url.to_string(), 3 * 1000);

                self.license_content_fetched = true;
            }
        }

        self.init = true && self.license_content != None;

        if self.init {
            println!("# Application init, launching GUI ...");
            println!();
        }
    }

    fn validater(&mut self) {
        self.can_goto_install_config_step = self.license_agreed;

        // TODO: Add path validation for other platforms.
        if data_validator::is_path_legal_in_windows(&self.install_config.installation_path) {
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

    fn draw_lang_selection(&mut self, ui: &mut Ui) {
        ui.label("");
        ui.heading(self.build_heading_text("    Select language: "));
        ui.label("");

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label(self.build_content_text("     "));

            egui::ComboBox::from_label("")
                .selected_text(self.build_content_text(&format!("{:?}", self.lang)))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut self.lang, Languages::English, "English");
                    ui.selectable_value(&mut self.lang, Languages::Chinese, "简体中文");
                });

            if ui.button(self.build_content_text("OK")).clicked() {
                self.lang_selected = true;

                println!("^ User selected language: {:?}", self.lang);
            }
        });
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
                                if _frame.info().system_theme.unwrap_or(eframe::Theme::Dark)
                                    == eframe::Theme::Light
                                {
                                    Color32::DARK_GREEN
                                } else {
                                    Color32::LIGHT_GREEN
                                },
                            ));
                        } else if me.steps == step {
                            let executing = me.build_catalog_text(&tip);
                            ui.label(executing.color(
                                if _frame.info().system_theme.unwrap_or(eframe::Theme::Dark)
                                    == eframe::Theme::Light
                                {
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

    fn draw_bottom_panel(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(40.0)
            .default_height(40.0)
            .show_inside(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let previous = self.build_button_text(&get_lang("previous", &self.lang));
                    let next = self.build_button_text(&get_lang("next", &self.lang));
                    let install = self.build_button_text(&get_lang("install", &self.lang));
                    let cancel = self.build_button_text(&get_lang("cancel", &self.lang));
                    let finish = self.build_button_text(&get_lang("finish", &self.lang));

                    if self.steps < 2 {
                        // In [Hello], [License] pages

                        if self.steps == 0 || self.can_goto_install_config_step {
                            if ui.button(next).clicked() {
                                if self.steps == 0 {
                                    println!("^ User clicked [Next] button in [Hello] page.");
                                } else {
                                    println!("^ User clicked [Next] button in [License] page.");
                                }

                                if self.steps < self.max_steps_count - 1 {
                                    self.steps = self.steps + 1;
                                }
                            }
                        }

                        if self.steps != 0 {
                            if ui.button(previous).clicked() {
                                if self.steps == 1 {
                                    println!("^ User clicked [Previous] button in [License] page.");
                                }

                                if self.steps > 0 {
                                    self.steps = self.steps - 1;
                                }
                            }
                        }
                    } else if self.steps == 2 {
                        // In [Install Config] page

                        if self.can_goto_install_step {
                            if ui.button(install).clicked() {
                                println!(
                                    "^ User clicked [Install] button in [Install Config] page."
                                );

                                self.steps = self.steps + 1;

                                if cfg!(target_os = "windows") {
                                    // Progress Report Channel
                                    let (prcs, prcr) = mpsc::channel();

                                    // Details Report Channel
                                    let (drcs, drcr) = mpsc::channel();

                                    // Cancel Command Send Channel
                                    let (ccscs, ccscr) = mpsc::channel();

                                    self.install_config.progress_channel_receiver = Some(prcr);
                                    self.install_config.install_details_channel_receiver = Some(drcr);
                                    self.install_config.cancle_channel_sender = Some(ccscs);

                                    self.install_thread_handle = Some(win_installer::install(
                                        self.lang.clone(),
                                        &self.install_config,
                                        &self.download_config,
                                        prcs,
                                        drcs,
                                        ccscr,
                                    ));
                                }
                            }
                        }
                        if ui.button(previous).clicked() {
                            println!("^ User clicked [Previous] button in [Install Config] page.");

                            if self.steps > 0 {
                                self.steps = self.steps - 1;
                            }
                        }
                    } else if self.steps == 3 {
                        // In [Install] page

                        // If installation not finished and not canceled, check thread status
                        if self.install_thread_handle.is_some()
                            && self.install_thread_handle.as_ref().unwrap().is_finished()
                        {
                            // Because installation thread finished, we can reset cancel status
                            self.install_config.installation_canceled = false;
                            self.install_config.installation_cancel_requested = false;

                            if self.install_config.install_progress == 1.0 {
                                // If installation finished and succeeded, show the next button
                                if ui.button(next).clicked() {
                                    println!();
                                    println!("^ User clicked [Next] button in [Install] page.");

                                    self.steps = self.steps + 1;
                                }
                            } else {
                                // Force installation progress sync to installation thread.
                                for _ in 0..100 {
                                    self.install_config.update_progress();
                                }

                                // If installation progress not reset to 0.0 but thread finished,
                                //   it means installation thread exit unexpectlly.
                                if self.install_config.install_progress != 0.0 {
                                    if !self.install_config.installation_failed_tip_pushed {
                                        let tip = "! Failed to receive details. This means installation thread exit unexpectlly.";
                                        println!("{}", tip);

                                        self.install_config.install_details.push(tip.to_string());
                                        self.install_config.installation_failed_tip_pushed = true;
                                    }
                                }

                                if ui.button(previous).clicked() {
                                    println!("^ User clicked [Previous] button in [Install] page.");

                                    // Reset thread handle to none.
                                    self.install_thread_handle = None;

                                    self.steps = self.steps - 1;
                                }
                            }
                        } else {
                            // If haven't requested cancellation, we draw the cancel button
                            // Otherwise, we draw the disabled cancel button
                            ui.add_enabled_ui(
                                !self.install_config.installation_cancel_requested,
                                |ui| {
                                    if ui.button(cancel).clicked() {
                                        println!();
                                        println!(
                                            "^ User clicked [Cancel] button in [Install] page."
                                        );
                                        println!();

                                        if self.install_config.cancle_channel_sender.is_some() {
                                            // When sending error, it means the receiver has been dropped
                                            // So we can assume the cancellation has been finished
                                            self.install_config.installation_canceled = self
                                                .install_config
                                                .cancle_channel_sender
                                                .as_ref()
                                                .unwrap()
                                                .send(1)
                                                .is_err();

                                            self.install_config.installation_cancel_requested =
                                                true;
                                        }
                                    }
                                },
                            );
                        }
                    } else {
                        // In [Finish] page

                        if ui.button(finish).clicked() {
                            println!("^ User clicked [Finish] button in [Finish] page.");

                            frame.close();
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
        if self.license_content_fetched {
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
        } else {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label("    ");
                ui.label(self.build_content_text(self.license_content.clone().unwrap().as_str()));
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label("    ");
                ui.label(self.build_content_text("Visit "));
                ui.hyperlink_to(self.build_content_text("LICENSE"), self.license_url.clone());
                ui.label(self.build_content_text(" to read the full license file."));
            });
        }

        ui.add_space(10.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let agreement = self.build_content_text(&get_lang("1_agree", &self.lang));

            ui.label("    ");
            if ui.checkbox(&mut self.license_agreed, agreement).changed() {
                println!(
                    "^ User {} with the license.",
                    if self.license_agreed {
                        "agreed"
                    } else {
                        "disagreed"
                    }
                );
            }
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
                        egui::TextEdit::singleline(&mut self.install_config.installation_path)
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
                if ui
                    .checkbox(
                        &mut self.install_config.windows_config.create_desktop_shortcut,
                        desktop_shortcut,
                    )
                    .changed()
                {
                    println!(
                        "^ User {} to create desktop shortcut.",
                        if self.install_config.windows_config.create_desktop_shortcut {
                            "chose"
                        } else {
                            "canceled"
                        }
                    );
                };
                ui.end_row();

                ui.label("");
                if self.install_config.windows_config.desktop_path.is_none() {
                    ui.label(get_lang("2_fetch_desktop_path_failed", &self.lang));
                } else {
                    ui.label(format!(
                        "({})",
                        self.install_config
                            .windows_config
                            .desktop_path
                            .as_ref()
                            .unwrap()
                    ));
                }
                ui.end_row();
                ui.end_row();

                if cfg!(target_os = "windows") {
                    ui.label("");
                    if ui
                        .checkbox(
                            &mut self
                                .install_config
                                .windows_config
                                .create_start_menu_shortcut,
                            start_menu_shortcut,
                        )
                        .changed()
                    {
                        println!(
                            "^ User {} to create start menu shortcut.",
                            if self
                                .install_config
                                .windows_config
                                .create_start_menu_shortcut
                            {
                                "chose"
                            } else {
                                "canceled"
                            }
                        );
                    };
                    ui.end_row();

                    ui.label("");
                    if self.install_config.windows_config.start_menu_path.is_none() {
                        ui.label(get_lang("2_fetch_start_menu_path_failed", &self.lang));
                    } else {
                        ui.label(format!(
                            "({})",
                            self.install_config
                                .windows_config
                                .start_menu_path
                                .as_ref()
                                .unwrap()
                        ));
                    }
                    ui.end_row();
                    ui.end_row();
                }

                ui.label("");
                if ui
                    .checkbox(
                        &mut self.install_config.install_as_portable,
                        portable_install,
                    )
                    .changed()
                {
                    println!(
                        "^ User {} to install as portable mode.",
                        if self.install_config.install_as_portable {
                            "chose"
                        } else {
                            "canceled"
                        }
                    );
                };
                ui.end_row();
                ui.end_row();

                ui.label("");
                if ui
                    .checkbox(
                        &mut self.install_config.launch_after_install,
                        launch_after_install,
                    )
                    .changed()
                {
                    println!(
                        "^ User {} to launch after install.",
                        if self.install_config.launch_after_install {
                            "chose"
                        } else {
                            "canceled"
                        }
                    );
                };
                ui.end_row();
                ui.end_row();

                ui.label("");
                ui.label(format!(
                    "Installing version: {}",
                    if self.download_config.version_patched() {
                        self.download_config.version.clone()
                    } else {
                        "latest (cause version not patched yet)".to_string()
                    }
                ));
                ui.end_row();
                ui.end_row();

                ui.label("");
                ui.label(format!(
                    "Installing profile: {}",
                    self.download_config.profile.clone()
                        + if self.download_config.is_profile_auto_detect {
                            " (auto detect)"
                        } else {
                            ""
                        }
                ));
                ui.end_row();
                ui.end_row();
            });
    }

    fn draw_body_installation(&mut self, ui: &mut Ui) {
        self.install_config.update_progress();
        self.install_config.receive_details();

        ui.vertical(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label("    ");
                ui.label(
                    egui::RichText::new(get_lang("3_installing", &self.lang))
                        .size(self.tip_text_font_size),
                );
                ui.add(
                    egui::ProgressBar::new(self.install_config.install_progress)
                        .animate(false)
                        .show_percentage()
                        .desired_width(ui.available_width() - 30.0),
                );
            });
            ui.end_row();
            ui.add_space(10.0);

            if self.install_details_visibility {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("    ");
                    if ui
                        .button(self.build_button_text(&get_lang("3_hide_details", &self.lang)))
                        .clicked()
                    {
                        self.install_details_visibility = false;
                    }
                });
                ui.end_row();
                ui.add_space(10.0);

                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                let lines = self.install_config.install_details.clone();
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show_rows(ui, row_height, lines.len(), |ui, row_range| {
                        for row in row_range {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                let line = &lines[row];
                                ui.label("    ");
                                if ui
                                    .label(self.build_content_text(line.clone().as_str()))
                                    .on_hover_text(line.clone())
                                    .hovered()
                                {}
                            });
                            // ui.add_space(5.0);
                        }
                    });
            } else {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("    ");
                    if ui
                        .button(self.build_button_text(&get_lang("3_view_details", &self.lang)))
                        .clicked()
                    {
                        self.install_details_visibility = true;
                    }
                });
                ui.end_row();
            }
        });
    }

    fn draw_body_finished(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(self.build_content_text("    "));
            ui.label(self.build_content_text(&get_lang("4_installed", &self.lang)));
            if self.install_config.launch_after_install {
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

        self.frame_index = self.frame_index + 1;
        if self.frame_index >= self.frame_per_second {
            self.frame_index = 0;
        }

        if self.init {
            egui::CentralPanel::default().show(ctx, |ui| {
                if self.lang_selected {
                    self.draw_steps(ui, _frame);
                    self.draw_bottom_panel(ui, _frame);
                    self.draw_body(ui);

                    self.validater();
                } else {
                    self.draw_lang_selection(ui);
                }
            });
        }
    }
}
