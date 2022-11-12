# pico-serial
Connect to a pico over serial with some helper features.

Currently only supports Windows and MacOS.

# Usage
```
Usage: pico-serial [<port>] [-l] [-b]

Send all (or some of) your connected pico into bootloader mode!

Positional Arguments:
  port              the rp2040's port, if unspecified the application will
                    automatically connect to the only rp2040 device detected

Options:
  -l, --list        list all connected rp2040 devices
  -b, --bootloader  send the rp2040 device into bootloader mode
  --help            display usage information
```
