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

use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

mod executor;
use executor::Executor;

mod ping;
use ping::Ping;

mod logger;
use logger::{logger, LogLevel};

#[derive(Debug)]
enum PingkeeperError {
    NoHostsToPing,
}

/// Time between loops
const CHECK_MS: usize = 100;

// Pingkeeper
/// Copyright (C) 2020  Ignacio Lago
///
/// This program comes with ABSOLUTELY NO WARRANTY.
/// This is free software, and you are welcome to redistribute it under certain conditions.

#[derive(StructOpt, Debug)]
#[structopt(name = "Pingkeeper")]
struct Opt {
    /// Command to run
    #[structopt(name = "COMMAND")]
    command: String,
    /// Hosts to ping, order is ignored
    #[structopt(long, default_value = "8.8.8.8 8.8.6.6 1.1.1.1 1.0.0.1")]
    hosts: String,
    /// Options for ping command
    #[structopt(long, name = "opts", default_value = "-c1")]
    ping_opt: String,
    /// Run command on start and restart it if command dies
    #[structopt(short, long)]
    keep_alive: bool,
    /// Seconds to check ping after executing command
    #[structopt(long, name = "seconds", default_value = "5")]
    wait_after_exec: usize,
    /// Check ping again after this amount of seconds from the latest success
    #[structopt(long, name = "n", default_value = "5")]
    ping_every: usize,
    /// Signal to end command on command restart: `SIGINT`, `SIGTERM`, etc
    #[structopt(short, long, default_value = "SIGINT")]
    signal: String,
    /// Verbose
    #[structopt(short, parse(from_occurrences))]
    verbose: u32,
    /// Do not output anything from command output, also reduces -v by 1
    #[structopt(short, long)]
    quiet: bool,
}

fn main() -> Result<(), PingkeeperError> {
    // opts
    let opt = Opt::from_args();
    // logger
    let logger = if !opt.quiet {
        // show errors by default
        logger(LogLevel::from(opt.verbose + 1))
    } else {
        logger(LogLevel::from(opt.verbose))
    };
    // hosts to ping
    let hosts: Vec<String> = opt.hosts.split(' ').map(str::to_string).collect();
    if hosts.is_empty() {
        return Err(PingkeeperError::NoHostsToPing);
    }
    // ping
    let ping = Ping::new(hosts, opt.ping_opt);
    // executor
    let mut executor = Executor::new(opt.command);
    // signal
    executor.set_signal(&opt.signal);
    // wait options to millis
    let wait_boot_ms = opt.wait_after_exec * 1000;
    let wait_check_ms = opt.ping_every * 1000;
    // flags and counters
    let mut is_boot = false;
    let mut time_since_last_check: usize = 0;
    loop {
        let should_spawn;
        match executor.is_alive() {
            Ok(is_alive) => {
                if is_alive || !opt.keep_alive {
                    // Do nothing if too early after boot or after check
                    if (is_boot && time_since_last_check < wait_boot_ms)
                        || (!is_boot && time_since_last_check < wait_check_ms)
                    {
                        should_spawn = false;
                    } else {
                        is_boot = false;
                        if ping.is_network_reachable().is_ok() {
                            // Network is working
                            logger(LogLevel::DEBUG, String::from("Network reachable"));
                            should_spawn = false;
                        } else {
                            // Network unreachable
                            logger(LogLevel::WARN, String::from("Network unreachable"));
                            should_spawn = true;
                        }
                        time_since_last_check = 0;
                    }
                } else {
                    logger(LogLevel::WARN, String::from("Child process is dead"));
                    should_spawn = true;
                }
            }
            Err(err) => {
                logger(LogLevel::ERROR, format!("Command error -> {}", err));
                should_spawn = true;
            }
        }

        // Check process launch
        if should_spawn {
            // Reset time to check
            time_since_last_check = 0;
            logger(
                LogLevel::DEBUG,
                String::from("Should spawn a child process"),
            );
            // If previous child, SIGINT
            if let Some(pid) = executor.get_pid() {
                if executor.kill().is_ok() {
                    logger(LogLevel::INFO, format!("Sent SIGINT to pid {}", pid));
                } else {
                    logger(
                        LogLevel::ERROR,
                        format!("Could not send SIGINT to pid {}", pid),
                    );
                }
            } else {
                executor.execute(opt.quiet);
                if let Some(pid) = executor.get_pid() {
                    logger(
                        LogLevel::INFO,
                        format!("Child process starting with pid {}", pid),
                    );
                    is_boot = true;
                } else {
                    logger(
                        LogLevel::ERROR,
                        String::from("Child process is dead on boot"),
                    );
                }
            }
        } else if !is_boot && time_since_last_check >= wait_check_ms {
            // Time to check network again
            time_since_last_check = 0;
        }
        sleep(Duration::from_millis(CHECK_MS as u64));
        // Add time to timer
        time_since_last_check += CHECK_MS;
    }
}
