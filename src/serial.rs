use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, stdin};

use tokio_serial::{SerialPort, SerialPortBuilderExt};

pub async fn connect(port: &str) -> Result<(), Box<dyn Error>> {
    let mut input = BufReader::new(stdin());
    let mut port = tokio_serial::new(port, 115200)
        .open_native_async()?;
    port.write_data_terminal_ready(true)?;

    let mut serial_buf = [0; 4096];
    let mut stdin_buf = String::new();
    loop {
        let serial_read = port.read(&mut serial_buf);
        let stdin_read = input.read_line(&mut stdin_buf);

        tokio::select! {
            serial_read = serial_read => {
                let n = serial_read?;
                print!("{}", String::from_utf8_lossy(&serial_buf[..n]));
            }
            stdin_read = stdin_read => {
                let _ = stdin_read?;
                port.write_all(stdin_buf.as_bytes()).await?;
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Sending device into bootloader mode...");
                let _ = port.set_baud_rate(1200);
                return Ok(());
            }
        }
    }
}
