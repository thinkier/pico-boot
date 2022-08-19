use std::fmt::Display;
use tokio_serial::SerialPortType;

#[derive(Debug)]
pub struct Rp2040Device {
    pub vid: String,
    pub pid: String,
    pub desc: &'static str,
    pub port: String,
    pub serial: Option<String>,
}

impl Display for Rp2040Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} {} {}{}",
               self.vid,
               self.pid,
               self.port,
               self.desc,
               self.serial.as_ref().map(|s| format!(" (Serial: {})", s)).unwrap_or_default()
        )
    }
}

pub fn list_rp2040() -> Vec<Rp2040Device> {
    tokio_serial::available_ports()
        .unwrap()
        .into_iter()
        .filter_map(|p| {
            if let SerialPortType::UsbPort(info) = p.port_type {
                if info.vid == 0x2E8A {
                    return Some(Rp2040Device {
                        vid: format!("{:04X}", info.vid),
                        pid: format!("{:04x}", info.pid),
                        desc: get_rp2040_name(info.pid),
                        port: p.port_name,
                        serial: info.serial_number,
                    });
                }
            }

            return None;
        })
        .collect::<Vec<_>>()
}

include!(concat!(env!("OUT_DIR"), "/rp2040_pids.rs"));
