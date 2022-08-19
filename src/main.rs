extern crate argh;
extern crate tokio_serial;

use std::process;
use crate::args::PicoBoot;
use crate::bootloader::send_to_bootloader;
use crate::devices::list_rp2040;

mod args;
mod bootloader;
mod devices;

#[tokio::main]
async fn main() {
    let mut args: PicoBoot = argh::from_env();

    if args.list {
        for dev in list_rp2040() {
            println!("2e8a:{} {} {}", dev.pid, dev.port, dev.desc);
        }
        return;
    }

    if args.port.is_none() {
        let mut devices = list_rp2040();
        match devices.len() {
            0 => eprintln!("Error: No RP2040 devices detected."),
            1 => {
                eprintln!("Using {} ({})", devices[0].port, devices[0].desc);
                args.port = Some(devices.pop().unwrap().port);
            }
            _ => eprintln!("Error: Multiple RP2040 devices detected.")
        }

        if args.port.is_none() {
            process::exit(3)
        }
    }

    let mut ports = args.port.into_iter().collect::<Vec<_>>();
    ports.extend(list_rp2040().into_iter().map(|d| d.port));

    for port in ports {
        send_to_bootloader(&port);
    }
}
