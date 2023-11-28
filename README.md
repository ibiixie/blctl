# blctl
A D-Bus based **b**ack**l**ight **c**on**t**ro**l** daemon for Linux.

### Usage examples

Increase backlight brightness by 5%

```bash
busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Increase u 5
```

Decrease backlight brightness by 25%

```bash
busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Decrease u 25
```

### Init system config

You will have to configure your init system to run blctl as root or at least with read/write access to the kernel's backlight interface file. As long as those requirements are fulfilled, any init system should be capable of working with blctl.

### Backlight keyboard keys

You can use blctl with any desktop environment as long as it allows you to call into the system-wide D-Bus in some way.

Here's an example of how I do it using Sway:

```
# ~/.config/Sway/config

bindsym XF86MonBrightnessUp exec busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Increase u 5
bindsym XF86MonBrightnessDown exec busctl --system call me.xela.blctl /me/xela/blctl me.xela.blctl1 Decrease u 5
```

### Supported distributions and devices

Blclt is distirbution-agnostic but needs to be compiled manually and requires you to write a config for whatever init system you use (or use one from the examples). As long as you do those, it will work on any combination of distribution and init system. I personally use blctl with Void Linux, runit and Sway.
