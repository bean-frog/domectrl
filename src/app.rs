use std::collections::VecDeque;
use std::time::Instant;
use crate::config::Config;
use crate::protocol::DomeResponse;
use crate::serial::SerialConn;
use crate::ui::settings_page::SettingsState;

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Main,
    Settings,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmdKey {
    LowerA,
    LowerB,
    RaiseA,
    RaiseB,
}

impl CmdKey {
    pub fn to_char(&self, cfg: &Config) -> char {
        let s = match self {
            CmdKey::LowerA => &cfg.cmd_lower_a,
            CmdKey::LowerB => &cfg.cmd_lower_b,
            CmdKey::RaiseA => &cfg.cmd_raise_a,
            CmdKey::RaiseB => &cfg.cmd_raise_b,
        };
        s.chars().next().unwrap_or('?')
    }
}

pub enum HoldState {
    Idle,
    Sending {
        cmd_key: CmdKey,
        next_send: Instant,
    },
}

pub struct AppState {
    pub page: Page,
    pub conn: SerialConn,
    pub cfg: Config,
    pub hold: HoldState,
    pub response: DomeResponse,
    pub log: VecDeque<String>,
    pub banner: Option<String>,
    pub settings_state: Option<SettingsState>,
}

impl AppState {
    pub fn new() -> Self {
        let (cfg, err) = crate::config::load_config();
        Self {
            page: Page::Main,
            conn: SerialConn::Disconnected,
            cfg,
            hold: HoldState::Idle,
            response: DomeResponse::Unknown,
            log: VecDeque::new(),
            banner: err,
            settings_state: None,
        }
    }
}

impl eframe::App for AppState {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::ui::theme::apply(ctx);

        // If auto_reconnect toggled off while reconnecting, transition to Error
        let error_reason = if let SerialConn::Reconnecting { reason, .. } = &self.conn {
            if !self.cfg.auto_reconnect { Some(reason.clone()) } else { None }
        } else {
            None
        };
        if let Some(reason) = error_reason {
            self.conn = SerialConn::Error(reason);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        match self.page {
            Page::Main => crate::ui::main_page::show(self, ui),
            Page::Settings => {
                if self.settings_state.is_none() {
                    self.settings_state = Some(SettingsState::from_config(&self.cfg));
                }
                crate::ui::settings_page::show(self, ui);
            }
            Page::Info => crate::ui::info_page::show(self, ui),
        }
    }
}
