# pico-boot
Send all (or some of) your connected pico into bootloader mode!

*Except if it's running MicroPython, and it decided to crash. Then you might need to unplug and press boot while plugging it back in.

# Usage
```
Usage: pico-boot.exe [<port>] [-l] [-a]

Send all (or some of) your connected pico into bootloader mode!

Positional Arguments:
  port              send a specific rp2040 device into bootloader mode,
                    identified by its port

Options:
  -l, --list        list all connected rp2040 devices
  -a, --all         send all connected rp2040 devices into bootloader mode
  --help            display usage information
```
