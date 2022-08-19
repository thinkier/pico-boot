use argh::*;

#[derive(FromArgs)]
/// Send all (or some of) your connected pico into bootloader mode!
pub struct PicoBoot {
    /// list all connected rp2040 devices
    #[argh(switch, short = 'l')]
    pub list: bool,

    /// send the rp2040 device into bootloader mode
    #[argh(switch, short = 'b')]
    pub bootloader: bool,

    /// the rp2040's port, if unspecified the application will automatically connect to the only rp2040 device detected
    #[argh(positional)]
    pub port: Option<String>,
}
