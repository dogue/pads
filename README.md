# PulseAudio Device Switcher
A simple CLI tool for swtiching the default/active output device for PulseAudio.

## Installation
Clone the repo and install with cargo via `cargo install --path .` or install from Crates.io with `cargo install pads`.

## Usage
```
Switches the default PulseAudio output device and moves all active applications to that device

Usage: pads [OPTIONS] <COMMAND>

Commands:
  list  List available output devices
  next  Cycle to the next output device
  set   Set the active device
  help  Print this message or the help of the given subcommand(s)

Options:
  -j, --json     Format output as JSON
  -h, --help     Print help
  -V, --version  Print version
  ```
