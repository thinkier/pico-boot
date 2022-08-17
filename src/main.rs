extern crate argh;
extern crate serialport;

use argh::FromArgs;
use serialport::SerialPortType;

include!(concat!(env!("OUT_DIR"), "/rp2040_pids.rs"));

#[derive(FromArgs)]
/// Send all (or some of) your connected pico into bootloader mode!
struct PicoBoot {
    /// list all connected rp2040 devices
    #[argh(switch, short = 'l')]
    pub list: bool,

    /// send all connected rp2040 devices into bootloader mode
    #[argh(switch, short = 'a')]
    pub all: bool,

    /// send a specific rp2040 device into bootloader mode, identified by its port
    #[argh(positional)]
    pub port: Option<String>,
}

#[derive(Debug)]
struct Rp2040Device {
    pid: String,
    desc: &'static str,
    port: String,
}

fn list_rp2040() -> Vec<Rp2040Device> {
    serialport::available_ports()
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

fn main() {
    let args: PicoBoot = argh::from_env();

    if args.list {
        for dev in list_rp2040() {
            println!("2e8a:{} {} {}", dev.pid, dev.port, dev.desc);
        }
        return;
    }

    if args.all && args.port.is_some() {
        eprintln!("Please only specify -a OR a port.")
    }

    let mut ports = args.port.into_iter().collect::<Vec<_>>();
    ports.extend(list_rp2040().into_iter().map(|d|d.port));

    for port in ports {
        let _ = serialport::new(port, 1200).open();
    }
}
