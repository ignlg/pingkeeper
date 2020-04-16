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
    /// Command to run
    #[structopt(name = "COMMAND")]
    pub command: String,
    /// Hosts to ping, order is ignored
    #[structopt(long, default_value = "8.8.8.8 8.8.6.6 1.1.1.1 1.0.0.1")]
    pub hosts: String,
    /// Port to connect on every host, only valid without --use-ping
    #[structopt(long, default_value = "53")]
    pub port: u32,
    /// Timeout in seconds, only valid without --use-ping
    #[structopt(short, long, default_value = "2")]
    pub timeout: u32,

    /// Use ping command
    #[structopt(long)]
    pub use_ping: bool,
    /// Options for ping command, only valid with --use-ping
    #[structopt(long, name = "opts", default_value = "-c1")]
    pub ping_opt: String,

    /// Run command on start and restart it if command dies
    #[structopt(short, long)]
    pub keep_alive: bool,

    /// Seconds to check ping after executing command
    #[structopt(long, name = "seconds", default_value = "5")]
    pub wait_after_exec: usize,
    /// Check network again after this amount of seconds from the latest success
    #[structopt(long, name = "n", default_value = "5")]
    pub network_every: usize,

    /// Signal to end command on command restart: `SIGINT`, `SIGTERM`, etc
    #[structopt(short, long, default_value = "SIGINT")]
    pub signal: String,
    /// Maximum number of command errors in a row, 0 for infinite
    #[structopt(short, long, default_value = "0")]
    pub max_errors: usize,

    /// Verbose, -v -vv -vvv
    #[structopt(short, parse(from_occurrences))]
    pub verbose: u32,
    /// Do not output anything from command output, also reduces -v by 1
    #[structopt(short, long)]
    pub quiet: bool,
}
