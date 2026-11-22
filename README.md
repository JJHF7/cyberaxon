![Plugin Icon](assets/icon.png)

# OpenDeck CyberAxon
# https://github.com/JJHF7/cyberaxon
# Fork from Ajazz AKP03 / Mirabox N3 Plugin
# by Andrey Viktorov
# https://github.com/4ndv/opendeck-akp03
# https://github.com/4ndv/mirajazz

An unofficial plugin for CyberAxon D3 devices

## OpenDeck version

Requires OpenDeck 2.5.0 or newer

## Supported devices

- CyberAxon D3 (0600:3002)

## Platform support

- Linux: Guaranteed (it was the only I needed and tested)

## Installation

1. Download an archive from [releases](https://github.com/JJHF7/cyberaxon/releases)
2. In OpenDeck: Plugins -> Install from file
3. Download [udev rules](./40-opendeck-cyberaxon.rules) and install them by copying into `/etc/udev/rules.d/` and running `sudo udevadm control --reload-rules`
4. Unplug and plug again the device, restart OpenDeck

## Adding new devices

Read [this wiki page](https://github.com/4ndv/opendeck-akp03/wiki/Adding-support-for-new-devices) for more information.

## Building

### Prerequisites

You'll need:

- A Linux OS of some sort
- Rust 1.87 and up with `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-gnu` targets installed
- [just](https://just.systems)

On Arch Linux:

```sh
sudo pacman -S just mingw-w64-gcc mingw-w64-binutils
```

Adding rust targets:

```sh
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
```

### Building a release package

```sh
$ just package
```

## Acknowledgments

This plugin is 100% based on the work of Andrey Viktorov and contributors of https://github.com/4ndv/opendeck-akp03 | https://github.com/4ndv/mirajazz
