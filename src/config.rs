use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: String,
    pub baud_rate: u32,
    pub config_dir: String,
    pub send_interval_ms: u32,
    pub auto_reconnect: bool,
    pub reconnect_interval_ms: u32,
    pub cmd_lower_a: String,
    pub cmd_lower_b: String,
    pub cmd_raise_a: String,
    pub cmd_raise_b: String,
}

impl Default for Config {
    fn default() -> Self {
        let config_dir = default_config_dir();
        Self {
            port: "COM4".to_string(),
            baud_rate: 9600,
            config_dir: config_dir.to_string_lossy().to_string(),
            send_interval_ms: 100,
            auto_reconnect: true,
            reconnect_interval_ms: 3000,
            cmd_lower_a: "a".to_string(),
            cmd_lower_b: "b".to_string(),
            cmd_raise_a: "A".to_string(),
            cmd_raise_b: "B".to_string(),
        }
    }
}

fn default_config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(base).join("domectrl")
    }
    #[cfg(not(target_os = "windows"))]
    {
        let base = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(base).join(".config").join("domectrl")
    }
}

pub fn config_path(cfg: &Config) -> PathBuf {
    PathBuf::from(&cfg.config_dir).join("config.json")
}

pub fn load_config() -> (Config, Option<String>) {
    let default = Config::default();
    let path = config_path(&default);
    if !path.exists() {
        return (default, None);
    }
    match std::fs::read_to_string(&path) {
        Err(e) => {
            eprintln!("Config read error: {e}");
            (default, Some(format!("Could not read config: {e}")))
        }
        Ok(s) => match serde_json::from_str::<Config>(&s) {
            Ok(cfg) => (cfg, None),
            Err(e) => {
                eprintln!("Config parse error: {e}");
                (default, Some(format!("Config parse error (using defaults): {e}")))
            }
        },
    }
}

pub fn save_config(cfg: &Config, path: &PathBuf) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Cannot create config dir: {e}"))?;
    }
    let json = serde_json::to_string_pretty(cfg).map_err(|e| format!("Serialize error: {e}"))?;
    std::fs::write(path, json).map_err(|e| format!("Write error: {e}"))
}

pub fn validate_config(cfg: &Config) -> Vec<String> {
    let mut errors = Vec::new();
    if cfg.baud_rate == 0 {
        errors.push("Baud rate must be a positive integer".to_string());
    }
    for (name, val) in [
        ("cmd_lower_a", &cfg.cmd_lower_a),
        ("cmd_lower_b", &cfg.cmd_lower_b),
        ("cmd_raise_a", &cfg.cmd_raise_a),
        ("cmd_raise_b", &cfg.cmd_raise_b),
    ] {
        if val.len() != 1 || !val.chars().next().map(|c| c.is_ascii() && !c.is_ascii_control()).unwrap_or(false) {
            errors.push(format!("{name}: must be exactly 1 printable ASCII character"));
        }
    }
    errors
}

pub fn clamp_config(cfg: &mut Config) {
    cfg.send_interval_ms = cfg.send_interval_ms.clamp(10, 2000);
    cfg.reconnect_interval_ms = cfg.reconnect_interval_ms.clamp(500, 30000);
}
