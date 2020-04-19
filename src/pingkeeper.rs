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

mod opt;
pub use opt::Opt;

mod executor;
use executor::Executor;

mod network_monitor;
use network_monitor::NetworkMonitor;

mod logger;
use logger::{logger, LogLevel};

// ---------------------- Spawn ----------------------
/// Spawn errors
#[derive(Debug, Eq, PartialEq)]
enum SpawnErr {
    KillErr(u32),
    SpawnErr,
}
/// Spawn errors
#[derive(Debug, Eq, PartialEq)]
enum SpawnOk {
    KillOk(u32),
    SpawnOk(u32),
}

fn spawn_controller(
    executor: &mut Executor,
    kill_cmd: &Option<String>,
    quiet: bool,
) -> Result<SpawnOk, SpawnErr> {
    // If previous child pid, kill
    if let Some(pid) = executor.get_pid() {
        if let Some(cmd) = kill_cmd {
            if executor.kill_custom_cmd(cmd).is_ok() {
                Ok(SpawnOk::KillOk(pid))
            } else {
                Err(SpawnErr::KillErr(pid))
            }
        } else if executor.kill().is_ok() {
            Ok(SpawnOk::KillOk(pid))
        } else {
            Err(SpawnErr::KillErr(pid))
        }
    } else {
        executor.spawn(quiet);
        if let Some(pid) = executor.get_pid() {
            Ok(SpawnOk::SpawnOk(pid))
        } else {
            Err(SpawnErr::SpawnErr)
        }
    }
}

// ---------------------- Pingkeeper ----------------------

/// Pingkeeper errors
#[derive(Debug, Eq, PartialEq)]
pub enum PingkeeperError {
    NoHostsToPing,
    TooManyErrors,
    InvalidTimeout,
}

/// Time between loops
const CHECK_MS: usize = 100;

/// Monitorizes that network is reachable and, in case of failure, runs a command
pub fn pingkeeper(opt: Opt) -> Result<(), PingkeeperError> {
    // logger
    let logger = if !opt.quiet {
        // show errors by default
        logger(LogLevel::from(opt.verbose + 1))
    } else {
        logger(LogLevel::from(opt.verbose))
    };
    // hosts to ping
    let hosts: Vec<String> = opt
        .hosts
        .trim()
        .split(' ')
        .filter(|h| !h.is_empty())
        .map(str::to_string)
        .collect();
    if hosts.is_empty() {
        return Err(PingkeeperError::NoHostsToPing);
    }
    // network monitor
    let mut network = NetworkMonitor::new(hosts, Some(opt.port));
    network.set_ping_opt(opt.ping_opt);
    if network.set_timeout(opt.timeout as u64).is_err() {
        return Err(PingkeeperError::InvalidTimeout);
    }
    // executor
    let mut executor = Executor::new(opt.command);
    // signal
    executor.set_signal(&opt.signal);
    // wait options to millis
    let wait_boot_ms = opt.wait_after_exec * 1000;
    let wait_check_ms = opt.network_every * 1000;
    // flags and counters
    let mut is_executing = false;
    let mut time_since_last_check: usize = 0;
    let mut errors_in_a_row: usize = 0;
    loop {
        let should_spawn;
        match executor.is_alive() {
            Ok(is_alive) => {
                // Clean exit?
                if !is_alive && opt.max_errors > 0 {
                    // Allowed?
                    if !opt.keep_alive {
                        errors_in_a_row = 0;
                    } else {
                        errors_in_a_row += 1;
                    }
                }
                if opt.keep_alive && !is_alive {
                    logger(LogLevel::WARN, String::from("Child process is dead"));
                    should_spawn = true;
                } else if (is_executing && time_since_last_check < wait_boot_ms)
                    || (!is_executing && time_since_last_check < wait_check_ms)
                {
                    should_spawn = false;
                } else {
                    is_executing = false;
                    if network.check(&opt.check_cmd, opt.use_ping).is_ok() {
                        logger(LogLevel::DEBUG, String::from("Network reachable"));
                        should_spawn = false;
                    } else {
                        logger(LogLevel::WARN, String::from("Network unreachable"));
                        should_spawn = true;
                    }
                    time_since_last_check = 0;
                }
            }
            Err(err) => {
                if opt.max_errors > 0 {
                    errors_in_a_row += 1;
                }
                logger(LogLevel::ERROR, format!("Command error -> {}", err));
                should_spawn = true;
            }
        }

        if opt.max_errors > 0 && errors_in_a_row > opt.max_errors {
            return Err(PingkeeperError::TooManyErrors);
        }

        // Check process launch
        if should_spawn {
            // Reset time to check
            time_since_last_check = 0;
            logger(
                LogLevel::DEBUG,
                String::from("Should spawn a child process"),
            );
            match spawn_controller(&mut executor, &opt.kill_cmd, opt.quiet) {
                // Kill
                Ok(SpawnOk::KillOk(pid)) => {
                    logger(LogLevel::INFO, format!("Kill done for pid {}", pid))
                }
                Err(SpawnErr::KillErr(pid)) => {
                    logger(LogLevel::ERROR, format!("Cannot kill pid {}", pid))
                }
                // Spawn
                Ok(SpawnOk::SpawnOk(pid)) => {
                    is_executing = true;
                    logger(
                        LogLevel::INFO,
                        format!("Child process starting with pid {}", pid),
                    )
                }
                Err(SpawnErr::SpawnErr) => logger(
                    LogLevel::ERROR,
                    String::from("Child process is dead on boot"),
                ),
            };
        } else if !is_executing && time_since_last_check >= wait_check_ms {
            // Time to check network again
            time_since_last_check = 0;
        }
        sleep(Duration::from_millis(CHECK_MS as u64));
        // Add time to timer
        time_since_last_check += CHECK_MS;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use executor::Executor;

    #[test]
    fn without_hosts() {
        let opt = Opt {
            command: String::from("echo"),
            hosts: String::new(),
            keep_alive: false,
            max_errors: 0,
            network_every: 5,
            ping_opt: String::from("-c1"),
            port: 53,
            quiet: true,
            signal: String::from("SIGINT"),
            use_ping: false,
            verbose: 0,
            wait_after_exec: 5,
            timeout: 2,
            check_cmd: None,
            kill_cmd: None,
        };
        let error = pingkeeper(opt);
        assert!(error.is_err());
        assert_eq!(error.unwrap_err(), PingkeeperError::NoHostsToPing);
    }
    #[test]
    fn max_errors() {
        let opt = Opt {
            command: String::from("__pingkeeper__test__command__"),
            hosts: String::from("0.0.0.0"),
            keep_alive: true,
            max_errors: 2,
            network_every: 5,
            ping_opt: String::from("-c1"),
            port: 53,
            quiet: true,
            signal: String::from("SIGINT"),
            use_ping: false,
            verbose: 0,
            wait_after_exec: 1,
            timeout: 2,
            check_cmd: None,
            kill_cmd: None,
        };
        let error = pingkeeper(opt);
        assert!(error.is_err());
        assert_eq!(error.unwrap_err(), PingkeeperError::TooManyErrors);
    }
    #[test]
    fn spawn_controller_test() {
        let mut executor = Executor::new(String::from("cat"));
        // Should spawn
        match spawn_controller(&mut executor, &None, true) {
            Ok(SpawnOk::SpawnOk(_)) => {}
            res => panic!(format!("Invalid result {:?}", res)),
        }
        // Should kill
        match spawn_controller(&mut executor, &None, true) {
            Ok(SpawnOk::KillOk(_)) => {}
            res => panic!(format!("Invalid result {:?}", res)),
        }
        // Should spawn
        match spawn_controller(&mut executor, &None, true) {
            Ok(SpawnOk::SpawnOk(_)) => {}
            res => panic!(format!("Invalid result {:?}", res)),
        }
    }
}
