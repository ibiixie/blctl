# blctl
A D-Bus based **b**ack**l**ight **c**on**t**ro**l** daemon for Linux.

### Distribution support

Blctl can be compiled for and runs on any distribution but only includes a service configuration for runit/Void Linux. If you write your own service configuration, Blctl should work fine. Feel free to contribute your service configuration to `examples/daemon/` if you do! :)

Do note that Blctl requires read and write access to the sysfs backlight device directory (`/sys/class/backlight/`). If you do write your own service configuration, keep that in mind. For runit, this is a non-problem since it runs services as root.

## Usage

You can call into Blctl through the system-wide message bus.

Increase backlight brightness by 5%

```bash
$ busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Increase u 5
```

Decrease backlight brightness by 25%

```bash
$ busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Decrease u 25
```

## Config

No config file exists by default, but one with the name `config.toml` can be created in `/etc/blctl` (e.g.: `touch "/etc/blctl/config.toml"`.

**Config options:**

 * `interface_override`<br>*Allows for the automatically selected backlight interface to be overridden with a manually set path.*

**Example config**

```toml
# /etc/blctl/config.toml

# Allows for the automatically selected backlight interface to be overridden with a manually set path.
# interface_override = "/sys/class/backlight/example_backlight/"
```

### Backlight keys

If you want to setup Blctl to be connected to the backlight keys on your keyboard, you can do so by configuring your desktop environment or window manager to call Blctl via its system-wide message bus.

Here is an example of how I accomplish this using Sway:

```
# ~/.config/Sway/config

bindsym XF86MonBrightnessUp exec busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Increase u 5
bindsym XF86MonBrightnessDown exec busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Decrease u 5
```

## Compiling and installing

1. Download and install [Rustup](https://www.rust-lang.org/tools/install).
2. Clone this repository to a location of your choice and navigate to it in your terminal.
3. Run `cargo install`.

1. Download and install [Rustup](https://www.rust-lang.org/tools/install)
2. Clone this repository to a location of your choice
3. Navigate to where you cloned the repository
4. Run `cargo build --release`

This will compile Blctl and place the binary in `~/.cargo/bin/`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
