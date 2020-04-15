# pingkeeper

[![Build Status](https://travis-ci.org/ignlg/pingkeeper.svg?branch=master)](https://travis-ci.org/ignlg/pingkeeper)

Launch a network-related subprocess and monitor network ping to keep it alive

## Installation

### Download binary

Download release binary from [releases page](https://github.com/ignlg/pingkeeper/releases).

### Build your own

Build with

```
cargo build --release
```

You will find your executable at `./target/release/pingkeeper`.

## Usage

```
USAGE:
    pingkeeper [FLAGS] [OPTIONS] <COMMAND>

FLAGS:
    -h, --help
            Prints help information

    -k, --keep-alive
            Keep command alive?

    -q, --quiet
            Do not output anything from command output, also reduces -v by 1

    -V, --version
            Prints version information

    -v
            Verbose


OPTIONS:
        --boot-time <boot-time>
            Seconds to check network after executing command [default: 5]

        --check-time <check-time>
            Seconds to check again after network is reachable [default: 5]

        --hosts <hosts>
            Hosts to ping, order is ignored [default: 8.8.8.8 8.8.6.6 1.1.1.1 1.0.0.1]

        --ping-opt <opts>
            Options for ping command [default: -c1]


ARGS:
    <COMMAND>
            Command to run
```

## Changelog / Roadmap

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

### v1.1.0

- [ ] opt `-s --signal`; default: `"SIGINT"`.

### v2.0.0

- [ ] opt `--kill-cmd`.
- [ ] opt `--check-cmd`.

### v3.0.0

- [ ] website.
- [ ] LaunchDaemon generator.
- [ ] macOS notifications: connection lost, connection recovered.
- [ ] opt `--disable-notifications`.

### Backlog

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
