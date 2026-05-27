use std::io::Read;
use std::sync::mpsc::Sender;
use serialport::SerialPort;

pub enum RxEvent {
    Data(Vec<u8>),
    Err(String),
}

pub fn spawn_reader(mut port: Box<dyn SerialPort>, tx: Sender<RxEvent>) {
    std::thread::spawn(move || {
        let mut buf = [0u8; 256];
        loop {
            match port.read(&mut buf) {
                Ok(0) => continue,
                Ok(n) => {
                    if tx.send(RxEvent::Data(buf[..n].to_vec())).is_err() {
                        break;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(e) => {
                    let _ = tx.send(RxEvent::Err(e.to_string()));
                    break;
                }
            }
        }
    });
}
