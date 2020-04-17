# pingkeeper

[![Crate][crate-image]][crate-link]
[![GPLv3 license][license-image]][license-link]
![MSRV][rustc-image]
[![Safety Dance][safety-image]][safety-link]
[![Build Status][build-image]][build-link]
[![Release Date][releases-image]][releases-link]
![Stability stable][stability-image]

Command line application that monitorises that network is reachable (direct tcp connection or ping) and, in case of failure, runs a command. Optionally it can monitor that the command is permanently running and restart it if network is unreachable.

Proudly made from Barcelona with Rust 🦀.

## How does it work?

![Flow Chart][flowchart-image]

## Installation

Three options:

### Cargo & Go

1.  If you already have `cargo` installed, use:

        cargo install pingkeeper

### Manual download

1.  Download release binaries from [releases page][releases-link].

1.  _recommended_ Check the integrity of the downloaded file:

        sha512sum --check pingkeeper-macos-v3.0.0.tar.gz.sha512

    It should say: `pingkeeper-macos-v3.0.0.tar.gz: OK`

1.  Extract archive with:

        tar xvf pingkeeper-macos-v3.0.0.tar.gz

1.  _recommended_ Check the integrity of the binary file with:

        sha512sum --check pingkeeper.sha512

    It should say: `pingkeeper: OK`

1.  Copy `pingkeeper` binary file to somewhere within your `$PATH`, ie: `/usr/local/bin`.

### Build it yourself

This requires the stable version of `rust` & `cargo` installed. Visit [Rust website][rust-link] for more information.

1.  Run this command:

        cargo build --release

2.  You will find your executable at `./target/release/pingkeeper`.

## Usage

### Usage examples

- Keep your vpn connection alive using OpenVPN:

  ```shell
  sudo pingkeeper -k "openvpn /home/user/vpn_configuration.ovpn"
  ```

- Keep your vpn connection alive using [Hummingbird][hummingbird-link] without any logging:

  ```shell
  sudo pingkeeper --keep-alive --quite "hummingbird denmark.ovpn"
  ```

- Send an email to your boss when your network is down, using ping as test:

  ```shell
  pingkeeper --use-ping "mail -s \"Sorry, my network is down. I will be right back asap.\" myboss@example.com < /dev/null"
  ```

- Send yourself a [pushbullet-cli][pushbullet-cli-link] message when your home server seems down, using ping as test:

  ```shell
  pingkeeper --hosts "192.168.1.50" --use-ping "pb push \"Is home server down?\""
  ```

- Tweet when your [opentracker][opentracker-link] bittorrent tracker server seems down, using [t][t-link]:

  ```shell
  pingkeeper --hosts "10.1.1.28" --port 6969 "t update \"Dear users, the tracker is currently down :(\""
  ```

- Run your own script when the damn wifi seems down again:

  ```shell
  pingkeeper "/home/user/try_reset_router.sh"
  ```

### Usage manual

Help available running `pingkeeper --help`:

        USAGE:
        pingkeeper [FLAGS] [OPTIONS] <COMMAND>

        FLAGS:
        -h, --help
                Prints help information

        -k, --keep-alive
                Run command on start and restart it if command dies

        -q, --quiet
                Do not output anything from command output, also reduces -v by 1

                --use-ping
                Use ping command

        -V, --version
                Prints version information

        -v
                Verbose, -v -vv -vvv


        OPTIONS:
                --hosts <hosts>
                Hosts to ping, order is ignored [default: 8.8.8.8 8.8.6.6 1.1.1.1 1.0.0.1]

        -m, --max-errors <max-errors>
                Maximum number of command errors in a row, 0 for infinite [default: 0]

                --network-every <n>
                Check network again after this amount of seconds from the latest success [default: 5]

                --ping-opt <opts>
                Options for ping command, only valid with --use-ping [default: -c1]

                --port <port>
                Port to connect on every host, only valid without --use-ping [default: 53]

                --wait-after-exec <seconds>
                Seconds to check ping after executing command [default: 5]

        -s, --signal <signal>
                Signal to end command on command restart: `SIGINT`, `SIGTERM`, etc [default: SIGINT]

        -t, --timeout <timeout>
                Timeout in seconds, only valid without --use-ping [default: 2]


        ARGS:
        <COMMAND>
                Command to run

## Changelog

### v3.0.0

- [x] detect network connection directly.
- [x] opt `--use-ping`, use system ping instead of direct connection.
- [x] opt `-t --timeout`, seconds waiting for network connection.
- [x] opt `--max-errors`, number of keep-alive errors allowed in a row to keep running.
- [x] improve documentation.

### v2.0.0

- [x] rename opt ~~`--boot-time`~~ -> `--wait-after-exec`.
- [x] reanme opt ~~`--check-time`~~ -> `--ping-every`.
- [x] opt `--signal`; default: `"SIGINT"`.
- [x] flow chart.
- [x] move logic to subfiles.
- [x] tests.

### v1.0.0

- [x] launch command when ping fails.
- [x] opt `--hosts`; default: `"8.8.8.8 8.8.6.6 1.1.1.1 1.0.0.1"`.
- [x] detect if network is reachable.
- [x] opt `--ping-opt`, ping options; default: `"-c1"`.
- [x] opt `--boot-time`, seconds wait to check network after command; default: `5`.
- [x] opt `--check-time`, network check delay in seconds; default: `5`.
- [x] restart (send SIGINT signal) on network error.
- [x] opt `--keep-alive`, run command on init and restart on command exit.
- [x] opt `-v --verbose`, show log.
- [x] opt `-q --quiet`, hide stdout/stderr from subcommand.

## Backlog

- [x] add usage examples to docs.
- [ ] improve generated docs.
- [ ] export lib too.
- [ ] opt `--kill-cmd`, custom kill command.
- [ ] opt `--check-cmd`, custom check network command.
- [ ] pingkeeper tests with mocks.
- [ ] website.
- [ ] LaunchDaemon generator.
- [ ] macOS notifications: connection lost, connection recovered.
- [ ] opt `--disable-notifications`.
- [ ] detect SIGTERM on subprocess and stop.
- [ ] write pid to proc.
- [ ] opt `-f --force` to kill pid and remove pid from proc.
- [ ] check if interface is up.
- [ ] write logs to `/var/log`.

## License

Pingkeeper
Copyright (C) 2020 Ignacio Lago

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

[//]: # "assets"
[flowchart-image]: assets/Pingkeeper-flowchart.png
[//]: # "badges"
[crate-image]: https://img.shields.io/crates/v/pingkeeper
[downloads-image]: https://img.shields.io/crates/d/pingkeeper
[crate-link]: https://crates.io/crates/pingkeeper
[license-image]: https://img.shields.io/crates/l/pingkeeper
[license-link]: https://github.com/ignlg/pingkeeper/blob/next/LICENSE.md
[rustc-image]: https://img.shields.io/badge/rustc-1.36+-blue.svg
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[build-image]: https://travis-ci.org/ignlg/pingkeeper.svg?branch=master
[build-link]: https://travis-ci.org/ignlg/pingkeeper
[releases-image]: https://img.shields.io/github/release-date/ignlg/pingkeeper
[releases-link]: https://github.com/ignlg/pingkeeper/releases
[stability-image]: https://img.shields.io/badge/stability-stable-green
[//]: # "links"
[rust-link]: https://www.rust-lang.org/
[hummingbird-link]: https://gitlab.com/AirVPN/hummingbird
[pushbullet-cli-link]: https://github.com/GustavoKatel/pushbullet-cli
[opentracker-link]: https://erdgeist.org/arts/software/opentracker/
[t-link]: https://github.com/sferik/t
