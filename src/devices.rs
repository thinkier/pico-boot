use tokio_serial::SerialPortType;

#[derive(Debug)]
pub struct Rp2040Device {
    pub pid: String,
    pub desc: &'static str,
    pub port: String,
}

pub fn list_rp2040() -> Vec<Rp2040Device> {
    tokio_serial::available_ports()
        .unwrap()
        .into_iter()
        .filter_map(|p| {
            if let SerialPortType::UsbPort(info) = p.port_type {
                if info.vid == 0x2E8A {
                    return Some(Rp2040Device {
                        pid: format!("{:04x}", info.pid),
                        desc: get_rp2040_name(info.pid),
                        port: p.port_name,
                    });
                }
            }

            return None;
        })
        .collect::<Vec<_>>()
}

include!(concat!(env!("OUT_DIR"), "/rp2040_pids.rs"));
