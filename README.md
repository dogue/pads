# pads
A simple CLI tool for swtiching the default/active output device for PulseAudio.

## Installation
Clone the repo and install with cargo via `cargo install --path .`

## Usage
```
Switches the default PulseAudio output device and moves all active applications to that device

Usage: pads <COMMAND>

Commands:
  list  Lists available output devices
  set   Sets the default output device
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
  ```
