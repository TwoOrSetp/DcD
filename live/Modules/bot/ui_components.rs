use egui::{Color32, DragValue, RichText, vec2};
use egui::epaint::Shadow;
use egui_notify::{Toast, Toasts};
use parking_lot::Mutex;
use std::sync::Arc;

pub fn help_text<R>(ui: &mut egui::Ui, help: &str, add_contents: impl FnOnce(&mut egui::Ui) -> R) {
    if help.is_empty() {
        add_contents(ui); // don't show help icon if there's no help text
        return;
    }
    ui.horizontal(|ui| {
        add_contents(ui);
        ui.add_enabled_ui(false, |ui| ui.label("(?)").on_disabled_hover_text(help));
    });
}

pub fn create_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(Color32::from_rgba_premultiplied(40, 40, 40, 240))
        .stroke(egui::Stroke::new(1.0, Color32::from_gray(60)))
        .shadow(Shadow::NONE)
        .inner_margin(vec2(8.0, 8.0))
        .outer_margin(vec2(4.0, 4.0))
}

pub fn create_panel_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(Color32::from_rgba_premultiplied(30, 30, 30, 220))
        .stroke(egui::Stroke::new(1.0, Color32::from_gray(50)))
        .inner_margin(vec2(6.0, 6.0))
        .outer_margin(vec2(2.0, 2.0))
}

pub fn styled_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(text).color(Color32::WHITE))
            .fill(Color32::from_rgb(60, 60, 60))
            .stroke(egui::Stroke::new(1.0, Color32::from_gray(80)))
    )
}

pub fn styled_checkbox(ui: &mut egui::Ui, checked: &mut bool, text: &str) -> egui::Response {
    ui.add(
        egui::Checkbox::new(checked, RichText::new(text).color(Color32::WHITE))
    )
}

pub fn styled_slider<Num: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    value: &mut Num,
    range: std::ops::RangeInclusive<Num>,
    text: &str,
) -> egui::Response {
    ui.add(
        egui::Slider::new(value, range)
            .text(RichText::new(text).color(Color32::WHITE))
    )
}

pub fn styled_drag_value<Num: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    value: &mut Num,
    text: &str,
) -> egui::Response {
    ui.add(
        DragValue::new(value)
            .prefix(text)
    )
}

pub fn section_header(ui: &mut egui::Ui, text: &str) {
    ui.separator();
    ui.label(RichText::new(text).color(Color32::WHITE).strong().size(14.0));
    ui.separator();
}

pub fn compact_section_header(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).color(Color32::LIGHT_GRAY).strong().size(12.0));
}

pub struct ToastManager {
    pub toasts: Arc<Mutex<Toasts>>,
}

impl Default for ToastManager {
    fn default() -> Self {
        Self {
            toasts: Arc::new(Mutex::new(Toasts::new())),
        }
    }
}

impl ToastManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_success(&self, message: &str) {
        let mut toasts = self.toasts.lock();
        toasts.success(message);
    }

    pub fn show_error(&self, message: &str) {
        let mut toasts = self.toasts.lock();
        toasts.error(message);
    }

    pub fn show_warning(&self, message: &str) {
        let mut toasts = self.toasts.lock();
        toasts.warning(message);
    }

    pub fn show_info(&self, message: &str) {
        let mut toasts = self.toasts.lock();
        toasts.info(message);
    }

    pub fn draw(&self, ctx: &egui::Context) {
        let mut toasts = self.toasts.lock();
        toasts.show(ctx);
    }

    pub fn clear_all(&self) {
        let mut toasts = self.toasts.lock();
        // Clear method may not exist, so we'll just create a new Toasts instance
        *toasts = Toasts::new();
    }

    pub fn get_toasts_ref(&self) -> Arc<Mutex<Toasts>> {
        self.toasts.clone()
    }
}

pub fn create_compact_ui_style() -> egui::Style {
    let mut style = egui::Style::default();
    
    // Reduce spacing
    style.spacing.item_spacing = vec2(4.0, 2.0);
    style.spacing.button_padding = vec2(6.0, 2.0);
    style.spacing.menu_margin = vec2(4.0, 2.0).into();
    style.spacing.indent = 12.0;
    
    // Smaller text
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(11.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(10.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(9.0, egui::FontFamily::Proportional),
    );
    
    style
}

pub fn apply_ui_scale(ctx: &egui::Context, scale: f32) {
    let mut style = (*ctx.style()).clone();
    
    // Scale text sizes
    for (_, font_id) in style.text_styles.iter_mut() {
        font_id.size *= scale;
    }
    
    // Scale spacing
    style.spacing.item_spacing *= scale;
    style.spacing.button_padding *= scale;
    style.spacing.menu_margin *= scale;
    style.spacing.indent *= scale;
    
    ctx.set_style(style);
}
