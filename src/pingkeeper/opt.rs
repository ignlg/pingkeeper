/*
    Pingkeeper
    Copyright (C) 2020  Ignacio Lago

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use structopt::StructOpt;

// Pingkeeper
/// Copyright (C) 2020  Ignacio Lago
///
/// This program comes with ABSOLUTELY NO WARRANTY.
/// This is free software, and you are welcome to redistribute it under certain conditions.

#[derive(StructOpt, Debug)]
#[structopt(name = "Pingkeeper")]
pub struct Opt {
    /// Command to run.
    #[structopt(name = "COMMAND")]
    pub command: String,
    /// Space separated list of addresses or hosts (ping).
    ///
    /// For direct connection: List of IPv4 and IPv6, with or without port.
    ///
    /// For ping: List of hosts.
    ///
    /// Order does not matter, list will be shuffled.
    #[structopt(short = "H", long, default_value = "8.8.8.8 8.8.6.6 1.1.1.1 1.0.0.1")]
    pub hosts: String,
    /// Default port to connect, ignored if `--use-ping`.
    ///
    /// Port to connect if host does not have a port specified.
    #[structopt(short, long, default_value = "53")]
    pub port: u16,
    /// Timeout in seconds, ignored if `--use-ping`.
    #[structopt(short, long, default_value = "2")]
    pub timeout: u32,

    /// Use `ping` to check connection.
    ///
    /// Use system's `ping` command to check network connection.
    #[structopt(short = "P", long)]
    pub use_ping: bool,
    /// Options for `ping` command, requires `--use-ping`.
    #[structopt(long, name = "opts", default_value = "-c1")]
    pub ping_opt: String,

    /// Keep COMMAND alive.
    ///
    /// Run COMMAND on start, also restart it when it dies.
    #[structopt(short, long)]
    pub keep_alive: bool,

    /// Execution delay, in seconds.
    ///
    /// Seconds to check network for the first time after executing COMMAND.
    #[structopt(short, long, name = "seconds", default_value = "5")]
    pub wait_after_exec: usize,

    /// Network check delay, in seconds.
    ///
    /// Check network again after this amount of seconds from the latest success.
    #[structopt(short, long, name = "n", default_value = "5")]
    pub network_every: usize,

    /// Signal to kill COMMAND.
    ///
    /// Could be any unix signal: `SIGINT`, `SIGTERM`, etc.
    #[structopt(short, long, default_value = "SIGINT")]
    pub signal: String,

    /// Maximum number of COMMAND errors in a row.
    ///
    /// 0 for infinite. Only used by `--keep-alive`.
    #[structopt(short, long, default_value = "0")]
    pub max_errors: usize,

    /// Verbosity, -v -vv -vvv.
    ///
    /// Log levels:
    /// 0 = error, 1 = warning, 2 = info, 3 = debug.
    #[structopt(short, parse(from_occurrences), default_value = "0")]
    pub verbose: u32,
    /// Do not output anything from COMMAND output, also reduces `-v` by one.
    #[structopt(short, long)]
    pub quiet: bool,
}
