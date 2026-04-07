/* Libs */
use egui::{
    Color32, CornerRadius, FontFamily, FontId, Frame, Margin, Stroke, Style, TextStyle, Visuals,
    vec2,
};

pub struct Palette;

impl Palette {
    pub const BACKGROUND: Color32 = Color32::from_rgb(6, 10, 18);
    pub const PANEL: Color32 = Color32::from_rgba_premultiplied(8, 12, 21, 242);
    pub const PANEL_STROKE: Color32 = Color32::from_rgb(42, 56, 83);
    pub const HEADER: Color32 = Color32::from_rgba_premultiplied(13, 19, 31, 236);
    pub const INPUT: Color32 = Color32::from_rgb(17, 24, 38);
    pub const BOT_BUBBLE: Color32 = Color32::from_rgb(22, 31, 48);
    pub const USER_BUBBLE: Color32 = Color32::from_rgb(34, 27, 48);
    pub const TEXT: Color32 = Color32::from_rgb(235, 241, 255);
    pub const MUTED: Color32 = Color32::from_rgb(145, 158, 188);
    pub const CYAN: Color32 = Color32::from_rgb(91, 218, 240);
    pub const VIOLET: Color32 = Color32::from_rgb(178, 137, 236);
}

pub fn apply(ctx: &egui::Context) {
    let mut style = (*ctx.global_style()).clone();
    apply_visuals(&mut style);
    apply_spacing(&mut style);
    apply_text(&mut style);
    ctx.set_global_style(style);
}

pub fn window_frame() -> Frame {
    Frame::new()
        .fill(Palette::PANEL)
        .stroke(Stroke::new(1.0, Palette::PANEL_STROKE))
        .corner_radius(CornerRadius::same(24))
        .inner_margin(Margin::same(12))
}

pub fn header_frame() -> Frame {
    Frame::new()
        .fill(Palette::HEADER)
        .stroke(Stroke::new(1.0, Color32::from_rgb(35, 48, 76)))
        .corner_radius(CornerRadius::same(18))
        .inner_margin(Margin::symmetric(12, 9))
}

pub fn composer_frame() -> Frame {
    Frame::new()
        .fill(Color32::from_rgba_premultiplied(13, 20, 33, 246))
        .stroke(Stroke::new(1.0, Color32::from_rgb(38, 54, 83)))
        .corner_radius(CornerRadius::same(18))
        .inner_margin(Margin::symmetric(10, 8))
}

pub fn message_frame(is_user: bool) -> Frame {
    let (fill, stroke) = if is_user {
        (Palette::USER_BUBBLE, Color32::from_rgb(91, 64, 126))
    } else {
        (Palette::BOT_BUBBLE, Color32::from_rgb(39, 54, 80))
    };

    Frame::new()
        .fill(fill)
        .stroke(Stroke::new(1.0, stroke))
        .corner_radius(CornerRadius::same(18))
        .inner_margin(Margin::symmetric(12, 9))
}

fn apply_visuals(style: &mut Style) {
    let mut visuals = Visuals::dark();

    visuals.override_text_color = Some(Palette::TEXT);
    visuals.weak_text_color = Some(Palette::MUTED);
    visuals.window_fill = Palette::PANEL;
    visuals.panel_fill = Palette::BACKGROUND;
    visuals.faint_bg_color = Color32::from_rgb(13, 19, 31);
    visuals.extreme_bg_color = Color32::from_rgb(5, 8, 14);
    visuals.code_bg_color = Palette::INPUT;
    visuals.warn_fg_color = Palette::VIOLET;
    visuals.error_fg_color = Color32::from_rgb(255, 116, 148);
    visuals.window_corner_radius = CornerRadius::same(24);
    visuals.menu_corner_radius = CornerRadius::same(14);
    visuals.window_stroke = Stroke::new(1.0, Palette::PANEL_STROKE);
    visuals.button_frame = true;
    visuals.slider_trailing_fill = true;

    visuals.widgets.noninteractive.bg_fill = Palette::PANEL;
    visuals.widgets.noninteractive.weak_bg_fill = Palette::INPUT;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(34, 48, 75));
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Palette::TEXT);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(14);

    visuals.widgets.inactive.bg_fill = Palette::INPUT;
    visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(21, 30, 48);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(45, 63, 98));
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Palette::TEXT);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(14);

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(33, 49, 76);
    visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(38, 57, 88);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Palette::CYAN);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(14);

    visuals.widgets.active.bg_fill = Color32::from_rgb(47, 73, 108);
    visuals.widgets.active.weak_bg_fill = Color32::from_rgb(53, 79, 116);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, Palette::CYAN);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.corner_radius = CornerRadius::same(14);

    visuals.selection.bg_fill = Color32::from_rgba_premultiplied(66, 224, 255, 64);
    visuals.selection.stroke = Stroke::new(1.0, Palette::CYAN);

    style.visuals = visuals;
}

fn apply_spacing(style: &mut Style) {
    style.spacing.item_spacing = vec2(8.0, 8.0);
    style.spacing.window_margin = Margin::same(12);
    style.spacing.button_padding = vec2(12.0, 7.0);
    style.spacing.interact_size = vec2(22.0, 28.0);
}

fn apply_text(style: &mut Style) {
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(20.0, FontFamily::Proportional),
    );
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(14.0, FontFamily::Proportional));
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(13.5, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(13.0, FontFamily::Monospace),
    );
}
