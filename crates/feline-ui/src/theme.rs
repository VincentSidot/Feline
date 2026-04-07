/* Libs */
use egui::{
    Button, Color32, CornerRadius, FontFamily, FontId, Frame, Margin, RichText, Stroke, Style,
    TextStyle, Visuals, vec2,
};

pub struct Palette;

impl Palette {
    pub const BG: Color32 = Color32::from_rgb(8, 11, 18);
    pub const CARD: Color32 = Color32::from_rgb(18, 24, 38);
    pub const CARD_SOFT: Color32 = Color32::from_rgb(24, 32, 50);
    pub const TEXT: Color32 = Color32::from_rgb(232, 239, 255);
    pub const MUTED: Color32 = Color32::from_rgb(136, 151, 180);
    pub const CYAN: Color32 = Color32::from_rgb(63, 221, 255);
    pub const VIOLET: Color32 = Color32::from_rgb(160, 112, 255);
    pub const GREEN: Color32 = Color32::from_rgb(97, 255, 172);
    pub const AMBER: Color32 = Color32::from_rgb(255, 190, 92);
    pub const WHITE: Color32 = Color32::WHITE;
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
        .fill(Color32::from_rgba_premultiplied(8, 11, 18, 232))
        .stroke(Stroke::new(1.0, Color32::from_rgb(72, 95, 135)))
        .corner_radius(CornerRadius::same(22))
        .inner_margin(Margin::same(18))
}

pub fn hero_frame() -> Frame {
    Frame::new()
        .fill(Color32::from_rgba_premultiplied(18, 28, 48, 230))
        .stroke(Stroke::new(1.0, Color32::from_rgb(64, 91, 132)))
        .corner_radius(CornerRadius::same(20))
        .inner_margin(Margin::symmetric(18, 16))
}

pub fn card_frame() -> Frame {
    Frame::new()
        .fill(Color32::from_rgba_premultiplied(18, 24, 38, 220))
        .stroke(Stroke::new(1.0, Color32::from_rgb(43, 57, 83)))
        .corner_radius(CornerRadius::same(18))
        .inner_margin(Margin::same(14))
}

pub fn tab_button(label: &str, selected: bool) -> Button<'_> {
    let fill = if selected {
        Palette::CYAN
    } else {
        Palette::CARD_SOFT
    };
    let text = if selected { Palette::BG } else { Palette::TEXT };

    Button::new(RichText::new(label).strong().color(text))
        .fill(fill)
        .stroke(Stroke::new(1.0, Color32::from_rgb(61, 78, 112)))
        .corner_radius(CornerRadius::same(255))
}

pub fn pill_button(text: &str, color: Color32) -> Button<'_> {
    Button::new(RichText::new(text).small().strong().color(Palette::BG))
        .fill(color)
        .stroke(Stroke::NONE)
        .corner_radius(CornerRadius::same(255))
}

pub fn accent_button(text: &str, color: Color32) -> Button<'_> {
    Button::new(RichText::new(text).strong().color(Palette::WHITE))
        .fill(color.gamma_multiply(0.34))
        .stroke(Stroke::new(1.0, color))
        .corner_radius(CornerRadius::same(14))
}

fn apply_visuals(style: &mut Style) {
    let mut visuals = Visuals::dark();

    visuals.override_text_color = Some(Palette::TEXT);
    visuals.weak_text_color = Some(Palette::MUTED);
    visuals.window_fill = Color32::from_rgba_premultiplied(10, 14, 24, 232);
    visuals.panel_fill = Color32::from_rgba_premultiplied(8, 11, 18, 196);
    visuals.faint_bg_color = Color32::from_rgb(16, 22, 34);
    visuals.extreme_bg_color = Color32::from_rgb(6, 8, 14);
    visuals.code_bg_color = Color32::from_rgb(10, 16, 28);
    visuals.hyperlink_color = Palette::CYAN;
    visuals.warn_fg_color = Palette::AMBER;
    visuals.error_fg_color = Color32::from_rgb(255, 94, 129);
    visuals.window_corner_radius = CornerRadius::same(22);
    visuals.menu_corner_radius = CornerRadius::same(14);
    visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(66, 89, 129));
    visuals.slider_trailing_fill = true;
    visuals.button_frame = true;
    visuals.collapsing_header_frame = true;
    visuals.indent_has_left_vline = true;

    visuals.widgets.noninteractive.bg_fill = Palette::CARD;
    visuals.widgets.noninteractive.weak_bg_fill = Color32::from_rgb(13, 18, 29);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(44, 58, 84));
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Palette::TEXT);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(14);

    visuals.widgets.inactive.bg_fill = Color32::from_rgb(22, 31, 48);
    visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(25, 35, 54);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(52, 69, 103));
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(219, 229, 255));
    visuals.widgets.inactive.corner_radius = CornerRadius::same(12);

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(38, 61, 90);
    visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(44, 70, 103);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Palette::CYAN);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, Palette::WHITE);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(14);
    visuals.widgets.hovered.expansion = 1.0;

    visuals.widgets.active.bg_fill = Color32::from_rgb(56, 82, 122);
    visuals.widgets.active.weak_bg_fill = Color32::from_rgb(57, 90, 133);
    visuals.widgets.active.bg_stroke = Stroke::new(1.5, Palette::GREEN);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, Palette::WHITE);
    visuals.widgets.active.corner_radius = CornerRadius::same(14);
    visuals.widgets.active.expansion = 1.0;

    visuals.selection.bg_fill = Color32::from_rgba_premultiplied(63, 221, 255, 72);
    visuals.selection.stroke = Stroke::new(1.5, Palette::CYAN);

    style.visuals = visuals;
}

fn apply_spacing(style: &mut Style) {
    style.spacing.item_spacing = vec2(10.0, 10.0);
    style.spacing.window_margin = Margin::same(18);
    style.spacing.button_padding = vec2(14.0, 9.0);
    style.spacing.menu_margin = Margin::same(10);
    style.spacing.indent = 18.0;
    style.spacing.interact_size = vec2(20.0, 26.0);
    style.spacing.slider_width = 170.0;
    style.spacing.slider_rail_height = 6.0;
}

fn apply_text(style: &mut Style) {
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(24.0, FontFamily::Proportional),
    );
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(14.5, FontFamily::Proportional));
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(14.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(13.5, FontFamily::Monospace),
    );
}
