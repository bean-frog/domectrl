use egui::Color32;

pub const BG: Color32 = Color32::from_rgb(0x0a, 0x0a, 0x0a);
pub const PANEL_FILL: Color32 = Color32::from_rgb(0x11, 0x11, 0x11);
pub const WIDGET_BG: Color32 = Color32::from_rgb(0x1a, 0x00, 0x00);
pub const BUTTON_FILL: Color32 = Color32::from_rgb(0x1a, 0x00, 0x00);
pub const BUTTON_STROKE_COLOR: Color32 = Color32::from_rgb(0xcc, 0x00, 0x00);
pub const TEXT_NORMAL: Color32 = Color32::from_rgb(0xcc, 0x00, 0x00);
pub const TEXT_HEADING: Color32 = Color32::from_rgb(0xff, 0x22, 0x22);
pub const TEXT_STRONG: Color32 = Color32::from_rgb(0xff, 0x44, 0x44);
pub const SELECTION: Color32 = Color32::from_rgb(0x44, 0x00, 0x00);
pub const HYPERLINK: Color32 = Color32::from_rgb(0xff, 0x66, 0x66);
pub const SEPARATOR: Color32 = Color32::from_rgb(0x33, 0x00, 0x00);

pub const RAISE_FILL: Color32 = Color32::from_rgb(0x00, 0x1a, 0x00);
pub const RAISE_STROKE: Color32 = Color32::from_rgb(0x00, 0xcc, 0x00);
pub const RAISE_FILL_HELD: Color32 = Color32::from_rgb(0x00, 0x0d, 0x00);
pub const LOWER_FILL_HELD: Color32 = Color32::from_rgb(0x0d, 0x00, 0x00);

pub const ABORT_FILL: Color32 = Color32::from_rgb(0xcc, 0x00, 0x00);
pub const ABORT_TEXT: Color32 = Color32::WHITE;
pub const ABORT_STROKE: Color32 = Color32::from_rgb(0xff, 0x00, 0x00);

pub const STATUS_CONNECTED: Color32 = Color32::from_rgb(0x00, 0xcc, 0x44);
pub const STATUS_DISCONNECTED: Color32 = Color32::from_rgb(0xcc, 0x00, 0x00);

pub const RESPONSE_OPEN: Color32 = Color32::from_rgb(0x00, 0xcc, 0x44);
pub const RESPONSE_CLOSED: Color32 = Color32::from_rgb(0xcc, 0x00, 0x00);
pub const RESPONSE_UNKNOWN: Color32 = Color32::from_rgb(0x55, 0x55, 0x55);

pub fn apply(ctx: &egui::Context) {
    use egui::{FontFamily, FontId, Stroke, TextStyle, Visuals};

    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(TEXT_NORMAL);
    visuals.window_fill = BG;
    visuals.panel_fill = PANEL_FILL;
    visuals.faint_bg_color = WIDGET_BG;
    visuals.extreme_bg_color = BG;
    visuals.widgets.noninteractive.bg_fill = WIDGET_BG;
    visuals.widgets.inactive.bg_fill = BUTTON_FILL;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.5, BUTTON_STROKE_COLOR);
    visuals.widgets.hovered.bg_fill = BUTTON_FILL;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, TEXT_STRONG);
    visuals.widgets.active.bg_fill = LOWER_FILL_HELD;
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, BUTTON_STROKE_COLOR);
    visuals.selection.bg_fill = SELECTION;
    visuals.hyperlink_color = HYPERLINK;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, SEPARATOR);
    ctx.set_visuals(visuals);

    let mut style = (*ctx.global_style()).clone();
    style.text_styles = [
        (TextStyle::Body, FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Heading, FontId::new(18.0, FontFamily::Monospace)),
        (TextStyle::Small, FontId::new(11.0, FontFamily::Monospace)),
    ]
    .into();
    ctx.set_global_style(style);
}
