use egui::RichText;
use crate::app::{AppState, Page};
use crate::config::{clamp_config, config_path, save_config, validate_config};
use super::theme;

pub struct SettingsState {
    pub port_input: String,
    pub baud_input: String,
    pub send_interval_input: String,
    pub reconnect_interval_input: String,
    pub cmd_lower_a: String,
    pub cmd_lower_b: String,
    pub cmd_raise_a: String,
    pub cmd_raise_b: String,
    pub config_dir_input: String,
    pub auto_reconnect: bool,
    pub errors: Vec<String>,
    pub available_ports: Vec<String>,
}

impl SettingsState {
    pub fn from_config(cfg: &crate::config::Config) -> Self {
        let available_ports = serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .map(|p| p.port_name)
            .collect();
        Self {
            port_input: cfg.port.clone(),
            baud_input: cfg.baud_rate.to_string(),
            send_interval_input: cfg.send_interval_ms.to_string(),
            reconnect_interval_input: cfg.reconnect_interval_ms.to_string(),
            cmd_lower_a: cfg.cmd_lower_a.clone(),
            cmd_lower_b: cfg.cmd_lower_b.clone(),
            cmd_raise_a: cfg.cmd_raise_a.clone(),
            cmd_raise_b: cfg.cmd_raise_b.clone(),
            config_dir_input: cfg.config_dir.clone(),
            auto_reconnect: cfg.auto_reconnect,
            errors: Vec::new(),
            available_ports,
        }
    }

    fn apply_to_config(&self, cfg: &mut crate::config::Config) -> Vec<String> {
        let mut errors = Vec::new();

        cfg.port = self.port_input.clone();
        cfg.auto_reconnect = self.auto_reconnect;
        cfg.config_dir = self.config_dir_input.clone();

        match self.baud_input.parse::<u32>() {
            Ok(v) if v > 0 => cfg.baud_rate = v,
            _ => errors.push("Baud rate: must be a positive integer".to_string()),
        }
        match self.send_interval_input.parse::<u32>() {
            Ok(v) => cfg.send_interval_ms = v,
            Err(_) => errors.push("Send interval: must be an integer".to_string()),
        }
        match self.reconnect_interval_input.parse::<u32>() {
            Ok(v) => cfg.reconnect_interval_ms = v,
            Err(_) => errors.push("Reconnect interval: must be an integer".to_string()),
        }

        cfg.cmd_lower_a = self.cmd_lower_a.clone();
        cfg.cmd_lower_b = self.cmd_lower_b.clone();
        cfg.cmd_raise_a = self.cmd_raise_a.clone();
        cfg.cmd_raise_b = self.cmd_raise_b.clone();

        let mut val_errors = validate_config(cfg);
        errors.append(&mut val_errors);

        if errors.is_empty() {
            clamp_config(cfg);
        }
        errors
    }
}

#[derive(Default)]
enum Action {
    #[default]
    None,
    Apply,
    SaveToFile,
    Back,
}

pub fn show(state: &mut AppState, ui: &mut egui::Ui) {
    ui.heading(RichText::new("Settings").color(theme::TEXT_HEADING));
    ui.separator();

    // Collect button clicks while ss is borrowed; collect action intent as enum
    let action = {
        let ss = state.settings_state.as_mut().unwrap();

        // Connection section
        ui.group(|ui| {
            ui.label(RichText::new("Connection").color(theme::TEXT_STRONG));

            ui.horizontal(|ui| {
                ui.label("Serial port:");
                ui.text_edit_singleline(&mut ss.port_input);
                let ports = ss.available_ports.clone();
                egui::ComboBox::from_id_salt("port_combo")
                    .selected_text(if ports.is_empty() { "(none found)" } else { "▼" })
                    .show_ui(ui, |ui| {
                        for p in &ports {
                            ui.selectable_value(&mut ss.port_input, p.clone(), p);
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Baud rate:");
                ui.text_edit_singleline(&mut ss.baud_input);
            });

            ui.horizontal(|ui| {
                ui.label("Send interval:");
                ui.text_edit_singleline(&mut ss.send_interval_input);
                ui.label("ms");
            });

            ui.checkbox(&mut ss.auto_reconnect, "Auto-reconnect on error");

            ui.add_enabled_ui(ss.auto_reconnect, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label("Reconnect interval:");
                    ui.text_edit_singleline(&mut ss.reconnect_interval_input);
                    ui.label("ms");
                });
            });
        });

        ui.add_space(4.0);

        // Commands section
        ui.group(|ui| {
            ui.label(RichText::new("Commands").color(theme::TEXT_STRONG));
            for (label, field) in [
                ("Lower A char:", &mut ss.cmd_lower_a),
                ("Lower B char:", &mut ss.cmd_lower_b),
                ("Raise A char:", &mut ss.cmd_raise_a),
                ("Raise B char:", &mut ss.cmd_raise_b),
            ] {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.add(egui::TextEdit::singleline(field).desired_width(30.0));
                    if let Some(c) = field.chars().next() {
                        ui.label(format!("'{}' (0x{:02x})", c, c as u32));
                    }
                });
            }
        });

        ui.add_space(4.0);

        // Config directory section
        ui.group(|ui| {
            ui.label(RichText::new("Config Directory").color(theme::TEXT_STRONG));
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut ss.config_dir_input);
                if ui.button("Browse…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        ss.config_dir_input = path.to_string_lossy().to_string();
                    }
                }
            });
        });

        ui.separator();

        // Error display
        for e in &ss.errors.clone() {
            ui.colored_label(theme::ABORT_FILL, e);
        }

        // Collect action intent — buttons don't need ss
        let mut action = Action::None;
        ui.horizontal(|ui| {
            if ui.button("Apply").clicked() { action = Action::Apply; }
            if ui.button("Save to File").clicked() { action = Action::SaveToFile; }
            if ui.button("Back").clicked() { action = Action::Back; }
        });
        action
        // ss is dropped here
    };

    // Perform action with full access to state
    match action {
        Action::None => {}
        Action::Apply => {
            let mut cfg = state.cfg.clone();
            let errors = state.settings_state.as_ref().unwrap().apply_to_config(&mut cfg);
            if errors.is_empty() {
                state.cfg = cfg;
                state.settings_state.as_mut().unwrap().errors.clear();
            } else {
                state.settings_state.as_mut().unwrap().errors = errors;
            }
        }
        Action::SaveToFile => {
            let mut cfg = state.cfg.clone();
            let errors = state.settings_state.as_ref().unwrap().apply_to_config(&mut cfg);
            if errors.is_empty() {
                state.cfg = cfg;
                let default_path = config_path(&state.cfg);
                let dialog = rfd::FileDialog::new()
                    .set_file_name("config.json")
                    .set_directory(default_path.parent().unwrap_or(std::path::Path::new(".")));
                if let Some(path) = dialog.save_file() {
                    if let Err(e) = save_config(&state.cfg, &path) {
                        state.settings_state.as_mut().unwrap().errors = vec![e];
                    }
                }
            } else {
                state.settings_state.as_mut().unwrap().errors = errors;
            }
        }
        Action::Back => {
            let mut cfg = state.cfg.clone();
            let errors = state.settings_state.as_ref().unwrap().apply_to_config(&mut cfg);
            if errors.is_empty() {
                state.cfg = cfg;
                state.page = Page::Main;
                state.settings_state = None;
            } else {
                state.settings_state.as_mut().unwrap().errors = errors;
            }
        }
    }
}
