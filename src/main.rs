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
    let devices = list_rp2040();

    if args.list {
        for dev in devices {
            println!("{}", dev);
        }
        return;
    }

    if args.port.is_none() {
        match devices.len() {
            0 => eprintln!("Error: No RP2040 devices detected."),
            1 => {
                eprintln!("Using {}", devices[0]);
                args.port = Some(devices[0].port.clone());
            }
            _ => eprintln!("Error: Multiple RP2040 devices detected.")
        }

        if args.port.is_none() {
            process::exit(3)
        }
    }

    if let Some(port) = args.port {
        send_to_bootloader(&port);
    }
}
