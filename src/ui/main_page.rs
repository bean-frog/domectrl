use egui::{Button, Context, FontId, FontFamily, RichText, Sense, Stroke, Vec2};
use std::time::{Duration, Instant};
use crate::app::{AppState, CmdKey, HoldState, Page};
use crate::protocol::DomeResponse;
use crate::serial::SerialConn;
use super::theme;

pub fn show(state: &mut AppState, ui: &mut egui::Ui) {
    // Clone context so we can use ui mutably without a dangling borrow
    let ctx = ui.ctx().clone();

    // Top bar
    ui.horizontal(|ui| {
        if ui.button("Load Config").clicked() {
            let (cfg, err) = crate::config::load_config();
            state.cfg = cfg;
            state.banner = err;
        }
        if ui.button("Save Config").clicked() {
            let path = crate::config::config_path(&state.cfg);
            if let Err(e) = crate::config::save_config(&state.cfg, &path) {
                state.banner = Some(e);
            } else {
                state.banner = None;
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("?").clicked() {
                state.page = Page::Info;
            }
            if ui.button("Settings").clicked() {
                state.page = Page::Settings;
            }
        });
    });

    // Status row
    ui.horizontal(|ui| {
        let (dot_color, status_text) = match &state.conn {
            SerialConn::Connected { .. } => (theme::STATUS_CONNECTED, "CONNECTED".to_string()),
            SerialConn::Reconnecting { reconnect_at, .. } => {
                let secs = reconnect_at.saturating_duration_since(Instant::now()).as_secs();
                (theme::STATUS_DISCONNECTED, format!("Reconnecting…  ({}s)", secs))
            }
            SerialConn::Error(e) => (theme::STATUS_DISCONNECTED, format!("ERROR: {}", e)),
            SerialConn::Disconnected => (theme::STATUS_DISCONNECTED, "DISCONNECTED".to_string()),
        };

        ui.label(RichText::new("●").color(dot_color).size(16.0));
        ui.label(RichText::new(&status_text).color(dot_color));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let btn_label = match &state.conn {
                SerialConn::Connected { .. } => "Disconnect",
                SerialConn::Reconnecting { .. } => "Cancel",
                _ => "Connect",
            };
            if ui.button(btn_label).clicked() {
                state.hold = HoldState::Idle;
                let disconnect = matches!(&state.conn,
                    SerialConn::Connected { .. } | SerialConn::Reconnecting { .. });
                if disconnect {
                    state.conn = SerialConn::Disconnected;
                } else {
                    state.conn = crate::serial::connect(&state.cfg);
                }
            }
        });
    });

    // Error banner
    if let Some(msg) = state.banner.clone() {
        ui.colored_label(theme::ABORT_FILL, &msg);
    }

    ui.separator();

    // Motor buttons
    let connected = state.conn.is_connected();
    ui.columns(2, |cols| {
        cols[0].group(|ui| {
            ui.label(RichText::new("LOWER").color(theme::TEXT_HEADING));
            motor_button(ui, state, CmdKey::LowerA, "Lower A", false, connected);
            motor_button(ui, state, CmdKey::LowerB, "Lower B", false, connected);
        });
        cols[1].group(|ui| {
            ui.label(RichText::new("RAISE").color(theme::TEXT_HEADING));
            motor_button(ui, state, CmdKey::RaiseA, "Raise A", true, connected);
            motor_button(ui, state, CmdKey::RaiseB, "Raise B", true, connected);
        });
    });

    ui.separator();

    // Response display
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Controller Response").color(theme::TEXT_STRONG));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Clear").clicked() {
                    state.response = DomeResponse::Unknown;
                }
            });
        });
        let (resp_text, resp_color) = match &state.response {
            DomeResponse::Open => ("OPEN detected  (Y)", theme::RESPONSE_OPEN),
            DomeResponse::Closed => ("CLOSED detected  (X)", theme::RESPONSE_CLOSED),
            DomeResponse::Unknown => ("—", theme::RESPONSE_UNKNOWN),
        };
        ui.label(
            RichText::new(resp_text)
                .color(resp_color)
                .font(FontId::new(22.0, FontFamily::Monospace)),
        );
    });

    ui.separator();

    // ABORT button — full width
    let abort_size = Vec2::new(ui.available_width(), 40.0);
    let abort_btn = Button::new(
        RichText::new("  ABORT  ").color(theme::ABORT_TEXT).size(16.0),
    )
    .fill(theme::ABORT_FILL)
    .stroke(Stroke::new(2.0, theme::ABORT_STROKE))
    .min_size(abort_size);
    if ui.add(abort_btn).clicked() {
        state.hold = HoldState::Idle;
    }

    // Drive hold-send loop
    process_hold(state, &ctx);

    // Drive auto-reconnect
    process_reconnect(state, &ctx);

    // Drain serial rx
    drain_rx(state);
}

fn motor_button(
    ui: &mut egui::Ui,
    state: &mut AppState,
    key: CmdKey,
    label: &str,
    is_raise: bool,
    connected: bool,
) {
    let is_held = matches!(&state.hold, HoldState::Sending { cmd_key, .. } if *cmd_key == key);

    let (fill, stroke_color) = if is_raise {
        if is_held {
            (theme::RAISE_FILL_HELD, theme::RAISE_STROKE)
        } else {
            (theme::RAISE_FILL, theme::RAISE_STROKE)
        }
    } else if is_held {
        (theme::LOWER_FILL_HELD, theme::BUTTON_STROKE_COLOR)
    } else {
        (theme::BUTTON_FILL, theme::BUTTON_STROKE_COLOR)
    };

    let text_color = if is_raise { theme::RAISE_STROKE } else { theme::TEXT_NORMAL };
    let btn = Button::new(RichText::new(label).color(text_color))
        .fill(fill)
        .stroke(Stroke::new(1.5, stroke_color))
        .sense(Sense::click_and_drag())
        .min_size(Vec2::new(120.0, 30.0));

    let resp = ui.add_enabled(connected, btn);

    if resp.is_pointer_button_down_on() {
        if !is_held {
            state.hold = HoldState::Sending {
                cmd_key: key,
                next_send: Instant::now(),
            };
        }
    } else if is_held {
        state.hold = HoldState::Idle;
    }
}

fn process_hold(state: &mut AppState, ctx: &Context) {
    let now = Instant::now();

    let (cmd_byte, next_send) = match &state.hold {
        HoldState::Sending { cmd_key, next_send } => (cmd_key.to_char(&state.cfg) as u8, *next_send),
        HoldState::Idle => return,
    };

    if now < next_send {
        ctx.request_repaint_after(next_send.duration_since(now));
        return;
    }

    if let Err(e) = crate::serial::write_byte(&mut state.conn, cmd_byte) {
        let reason = e.clone();
        state.hold = HoldState::Idle;
        if state.cfg.auto_reconnect {
            let interval = Duration::from_millis(state.cfg.reconnect_interval_ms as u64);
            state.conn = SerialConn::Reconnecting {
                reason,
                reconnect_at: Instant::now() + interval,
            };
        } else {
            state.conn = SerialConn::Error(e);
        }
        return;
    }

    let interval = Duration::from_millis(state.cfg.send_interval_ms as u64);
    if let HoldState::Sending { next_send, .. } = &mut state.hold {
        *next_send = now + interval;
    }
    ctx.request_repaint_after(interval);
}

fn process_reconnect(state: &mut AppState, ctx: &Context) {
    let reconnect_at = match &state.conn {
        SerialConn::Reconnecting { reconnect_at, .. } => *reconnect_at,
        _ => return,
    };

    ctx.request_repaint_after(Duration::from_millis(500));

    if Instant::now() < reconnect_at {
        return;
    }

    let new_conn = crate::serial::connect(&state.cfg);
    match new_conn {
        SerialConn::Connected { .. } => {
            state.conn = new_conn;
        }
        _ => {
            let interval = Duration::from_millis(state.cfg.reconnect_interval_ms as u64);
            // Extract reason before mutating conn
            let reason = match &state.conn {
                SerialConn::Reconnecting { reason, .. } => reason.clone(),
                _ => String::new(),
            };
            state.conn = SerialConn::Reconnecting {
                reason,
                reconnect_at: Instant::now() + interval,
            };
        }
    }
}

fn drain_rx(state: &mut AppState) {
    use crate::serial::reader::RxEvent;

    // Collect first to release borrow on state.conn
    let events: Vec<RxEvent> = if let SerialConn::Connected { rx, .. } = &state.conn {
        std::iter::from_fn(|| rx.try_recv().ok()).collect()
    } else {
        return;
    };

    for event in events {
        match event {
            RxEvent::Data(bytes) => {
                if let Some(r) = crate::protocol::parse_response(&bytes) {
                    state.response = r;
                }
                let line = String::from_utf8_lossy(&bytes).to_string();
                state.log.push_back(line);
                while state.log.len() > 200 {
                    state.log.pop_front();
                }
            }
            RxEvent::Err(e) => {
                state.hold = HoldState::Idle;
                if state.cfg.auto_reconnect {
                    let interval = Duration::from_millis(state.cfg.reconnect_interval_ms as u64);
                    state.conn = SerialConn::Reconnecting {
                        reason: e,
                        reconnect_at: Instant::now() + interval,
                    };
                } else {
                    state.conn = SerialConn::Error(e);
                }
                return;
            }
        }
    }
}
