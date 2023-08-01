use eframe::{egui, CreationContext};

pub fn set_default_font(cc: &CreationContext) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "SrcHei".to_string(),
        egui::FontData::from_owned(include_bytes!("../../assets/fonts/SrcHei.ttf").to_vec()),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "SrcHei".to_string());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("SrcHei".to_string());
    cc.egui_ctx.set_fonts(fonts);
}
