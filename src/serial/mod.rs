pub mod reader;

use std::sync::mpsc::{self, Receiver};
use std::time::Instant;
use serialport::SerialPort;
use crate::config::Config;
use reader::{RxEvent, spawn_reader};

pub enum SerialConn {
    Disconnected,
    Connected {
        port: Box<dyn SerialPort>,
        rx: Receiver<RxEvent>,
    },
    Error(String),
    Reconnecting {
        reason: String,
        reconnect_at: Instant,
    },
}

impl SerialConn {
    pub fn is_connected(&self) -> bool {
        matches!(self, SerialConn::Connected { .. })
    }
}

pub fn connect(cfg: &Config) -> SerialConn {
    let builder = serialport::new(&cfg.port, cfg.baud_rate)
        .timeout(std::time::Duration::from_millis(100));

    // Open once for writing, clone for the reader thread
    let port = match builder.open() {
        Ok(p) => p,
        Err(e) => return SerialConn::Error(format!("Failed to open {}: {}", cfg.port, e)),
    };

    let reader_port = match port.try_clone() {
        Ok(p) => p,
        Err(e) => return SerialConn::Error(format!("Failed to clone port: {e}")),
    };

    let (tx, rx) = mpsc::channel();
    spawn_reader(reader_port, tx);
    SerialConn::Connected { port, rx }
}

pub fn write_byte(conn: &mut SerialConn, byte: u8) -> Result<(), String> {
    if let SerialConn::Connected { port, .. } = conn {
        use std::io::Write;
        port.write_all(&[byte]).map_err(|e| e.to_string())
    } else {
        Err("Not connected".to_string())
    }
}
