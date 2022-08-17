extern crate reqwest;

use std::env;
use std::error::Error;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

fn is_hex(c: char) -> bool {
    c.is_numeric() || c == 'a' || c == 'b' || c == 'c' || c == 'd' || c == 'e' || c == 'f'
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pi_usb_pid = reqwest::get("https://raw.githubusercontent.com/raspberrypi/usb-pid/main/Readme.md")
        .await?
        .text()
        .await?;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("rp2040_pids.rs");
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(&dest_path)
        .await?;

    f.write_all(b"fn get_rp2040_name(pid: u16) -> &'static str {\n").await?;
    f.write_all(b"  match pid {\n").await?;

    for line in pi_usb_pid.lines() {
        let comps = line.split('|').collect::<Vec<_>>();
        let pid = comps.get(1);
        let desc = comps.get(3);

        if let (Some(pid), Some(desc)) = (pid, desc) {
            let pid = pid.trim().to_ascii_lowercase();
            let desc = desc.trim().replace("\\", "\\\\").replace("\"", "\\\"");

            // Ensure neither are empty
            if pid.len() == 0 || desc.len() == 0 {
                continue;
            }

            // Ensure pid is a valid hex-notated string
            if pid.len() < 3 || !pid.starts_with("0x") || pid[2..].contains(|c| !is_hex(c)) {
                continue;
            }

            f.write_all(format!("    {} => \"{}\",\n", pid, desc).as_bytes()).await?;
        }
    }
    f.write_all(b"    _ => \"Generic RP2040\"\n").await?;
    f.write_all(b"  }\n").await?;
    f.write_all(b"}\n").await?;


    return Ok(());
}
