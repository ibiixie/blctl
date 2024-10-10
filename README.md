# blctl

A minimal **b**ack**l**ight **c**on**t**ro**l** daemon for Linux.

## Features

- Set brightness using `blctl set [OPTIONS] <LEVEL>`
- Increase brightness using `blctl increase [OPTIONS] <AMOUNT>`
- Decrease brightness using `blctl decrease [OPTIONS] <AMOUNT>`
- Get brightness using `blctl get [OPTIONS]`
- Get raw maximum brightness using `blctl get-max`
- Optionally use raw backlight device driver values using `--from-raw` and `--to-raw`

## Usage

Using `blctl` is simple thanks to its easy-to-use command-line interface.

To list all commands and their options, run `blctl --help` and `blctl [COMMAND] --help` in your terminal.

By default, most commands take as input and print as a result to stdout a value
between 0-100 inclusive. If an error is encountered, `blctl` will print the
error to stderr and exit with a non-zero exit code.

The function to map arbitrary backlight brightness levels to a 0-100 inclusive
range is lossy in precision. If losslessness is required, use the `--from-raw`
and `--to-raw` flags to specify brightness and print results in the
arbitrary brightness range defined by your backlight device driver rather
than the more portable but lossy mapped range.

## Platforms

While only a NixOS flake is provided, `blctl` should in theory work on any
Linux distribution with Sysfs support. For distributions other than NixOS,
manual packaging is required. Systemd and Runit service configurations can
be found in the `./service-configs/` directory of this repository.

### NixOS

After adding this repository as a flake, `blctl` can be installed either by
specifying the package in your  directly, or by using the module provided by the flake.

## Internals

Internally, `blctl` is written only in safe Rust with no reliance on any
non-Rust dependencies. It uses Sysfs to communicate with the backlight device
driver using simple file reads and writes, and uses a Unix domain socket for
inter-process communication between the command-line interface (`blctl`)
and the daemon (`blctld`).

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
