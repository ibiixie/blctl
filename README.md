# blctl
A D-Bus based **b**ack**l**ight **c**on**t**ro**l** daemon for Linux.

## Usage

You can call into blctl through the system-wide message bus.

Increase backlight brightness by 5%

```bash
$ busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Increase u 5
```

Decrease backlight brightness by 25%

```bash
$ busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Decrease u 25
```

## Config

No config file exists by default, but one with the name `config.toml` can be created in `/etc/blctl` (e.g.: `touch "/etc/blctl/config.toml"`).

**Config options:**

 * `interface_override`<br>*Allows for the automatically selected backlight interface to be overridden with a manually set path.*

**Example config**

```toml
# /etc/blctl/config.toml

# Allows for the automatically selected backlight interface to be overridden with a manually set path.
# interface_override = "/sys/class/backlight/example_backlight/"
```

### Backlight keys

If you want to setup blctl to be connected to the backlight keys on your keyboard, you can do so by configuring your desktop environment or window manager to call blctl via its system-wide message bus.

Here is an example of how I accomplish this using Sway:

```
# ~/.config/Sway/config

bindsym XF86MonBrightnessUp exec busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Increase u 5
bindsym XF86MonBrightnessDown exec busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Decrease u 5
```

## Compiling and installing

To compile and install blctl, simply run the `install.sh` script in the root directory of the repository:

```bash
$ ./install.sh
```

This will automatically install blctl to `/usr/bin/`.

If you use runit, you can pass `--runit` as a parameter to the installer and it will automatically copy a default service configuration file for you:

```bash
$ ./install.sh --runit
```

If you use another init system, you will have to create your own service configuration to use with blctl. Feel free to contribute your config to `./service-configs/` if you do! :)

Note that blctl at minimum requires read and write access to the sysfs backlight device directory (`/sys/class/backlight`) to function properly.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
