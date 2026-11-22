![Plugin Icon](assets/icon.png)

# OpenDeck CyberAxon

An unofficial [OpenDeck](https://github.com/nekename/OpenDeck) plugin that lets you use the programmable
LCD keys of the **RayCue CyberAxon D3** docking station on **Linux**: assign actions and custom icons
to each key, just like on a Stream Deck.

This project is not affiliated with RayCue.

Fork of the Ajazz AKP03 / Mirabox N3 plugin by Andrey Viktorov:

- https://github.com/4ndv/opendeck-akp03
- https://github.com/4ndv/mirajazz

## OpenDeck version

Requires OpenDeck 2.5.0 or newer

## Supported devices

| Device | USB ID | Notes |
|---|---|---|
| RayCue CyberAxon D3 | `0600:3002` | 12 LCD keys, no encoders |

The device identifies itself as `HOTSPOTEKUSB HID DEMO`. Check yours with:

```sh
lsusb | grep -i 0600:3002
```

RayCue states that the CyberAxon D5 shares the same hardware as the D3 and differs only in the screen
function. The D5 has **not** been tested with this plugin.

## Platform support

- Linux: tested and working. It is the only platform the author needed and tested.
- macOS and Windows: not supported.

## Installation

1. Download `opendeck-cyberaxon.plugin.zip` and the udev rules from the
   [latest release](https://github.com/JJHF7/cyberaxon/releases/latest)
2. In OpenDeck: Plugins -> Install from file, and select the `.zip`
3. Install the [udev rules](./40-opendeck-cyberaxon.rules) by copying them into `/etc/udev/rules.d/` and running
   `sudo udevadm control --reload-rules`
4. Unplug and plug again the device, then restart OpenDeck

The device should appear in OpenDeck as **RayCue CyberAxon D3**.

## Known limitations

- Only tested on one unit and one firmware version. If yours behaves differently, please open an issue.
- Screen brightness is fixed by the plugin at start-up.
- The manufacturer does not document the protocol. Support relies on the protocol shared by other
  Mirabox/HSV-based devices (via [mirajazz](https://github.com/4ndv/mirajazz)), which this device appears to use.

## Troubleshooting

- **The device is not detected:** check that `lsusb` shows `0600:3002`, that the udev rules are installed,
  then unplug and plug the device again and restart OpenDeck.
- **The screen shows a screensaver instead of the keys:** the device waits for the host to initialise it.
  If it stays that way with the plugin installed, open an issue and attach the plugin log.
- **Permission errors on `/dev/hidraw*`:** the udev rules are missing or were not reloaded.

When opening an issue, please include the output of `lsusb -v -d 0600:3002`, your Linux distribution and the
OpenDeck version.

## Building

### Prerequisites

You'll need:

- A Linux OS of some sort
- Rust 1.87 and up with the `x86_64-unknown-linux-gnu` target installed
- [just](https://just.systems)
- `zip`

Adding the Rust target:

```sh
rustup target add x86_64-unknown-linux-gnu
```

### Building a release package

```sh
just package
```

The package is generated at `build/opendeck-cyberaxon.plugin.zip`.

## Acknowledgments

This plugin is 100% based on the work of Andrey Viktorov and the contributors of
https://github.com/4ndv/opendeck-akp03 and https://github.com/4ndv/mirajazz.

Licensed under the GNU General Public License v3.0, like the original project. See [LICENSE](LICENSE).

"RayCue" and "CyberAxon" are trademarks of their respective owners.
