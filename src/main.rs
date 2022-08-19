extern crate argh;
extern crate tokio_serial;

use std::error::Error;
use std::process;
use tokio::io::{AsyncReadExt, stdin};
use crate::args::PicoBoot;
use crate::bootloader::send_to_bootloader;
use crate::devices::list_rp2040;
use crate::serial::connect;

mod args;
mod bootloader;
mod devices;
mod serial;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args: PicoBoot = argh::from_env();
    let devices = list_rp2040();

    if args.list {
        for dev in devices {
            println!("{}", dev);
        }
        return Ok(());
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
            process::exit(3);
        }
    }

    if args.bootloader {
        send_to_bootloader(args.port.as_ref().unwrap());
        return Ok(());
    }

    loop {
        connect(args.port.as_ref().unwrap()).await.unwrap();
        eprintln!("Device disconnected. Press any key to reconnect...");

        let cont = wait_for_any_key();
        let sigterm = tokio::signal::ctrl_c();

        tokio::select! {
            _ = sigterm => {
                println!("Ctrl-C received, exiting...");
                process::exit(0);
            }
            _ = cont => {
                println!("Reconnecting...");
            }
        }
    }
}

async fn wait_for_any_key() -> Result<(), Box<dyn Error>> {
    stdin().read_u8().await?;

    return Ok(());
}