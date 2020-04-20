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

use rand::{seq::SliceRandom, thread_rng};

use std::net::SocketAddr;
use std::process;
use std::time::Duration;

mod tools;
use tools::*;

const DEFAULT_TIMEOUT: u64 = 2;

/// Network monitor errors
#[derive(Debug, PartialEq, Eq)]
pub enum NetworkError {
    NetworkUnreachable,
    NoHostsToCheck,
    InvalidTimeout,
}

/// Network monitor
pub struct NetworkMonitor {
    hosts: Vec<String>,
    addresses: Vec<SocketAddr>,
    ping_opt: Option<String>,
    timeout: Duration,
}

// Public
impl NetworkMonitor {
    /// Instantiates a new NetworkMonitor
    pub fn new(hosts: Vec<String>, port: Option<u16>) -> Self {
        let addresses = hosts_to_addresses(&hosts, port);
        NetworkMonitor {
            hosts,
            addresses,
            ping_opt: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
        }
    }
    /// Checks network status
    pub fn check(&self, check_cmd: &Option<String>, use_ping: bool) -> Result<(), NetworkError> {
        if let Some(cmd) = check_cmd {
            self.check_custom_cmd(cmd)
        } else if use_ping {
            self.is_ping_pong()
        } else {
            self.is_network_reachable()
        }
    }
    /// Checks if ping answers with a pong
    pub fn is_ping_pong(&self) -> Result<(), NetworkError> {
        if self.hosts.is_empty() {
            return Err(NetworkError::NoHostsToCheck);
        }
        let mut hosts = self.hosts.to_vec();
        let mut rng = thread_rng();
        hosts.shuffle(&mut rng);
        let mut ping_opt = String::new();
        if let Some(opt) = &self.ping_opt {
            ping_opt = String::from(opt)
        };
        if can_ping_some(hosts, ping_opt) {
            Ok(())
        } else {
            Err(NetworkError::NetworkUnreachable)
        }
    }
    /// Checks if network is reachable
    pub fn is_network_reachable(&self) -> Result<(), NetworkError> {
        if self.addresses.is_empty() {
            return Err(NetworkError::NoHostsToCheck);
        }
        let mut rng = thread_rng();
        let mut addresses = self.addresses.clone();
        addresses.shuffle(&mut rng);
        if can_connect_some(addresses, self.timeout) {
            Ok(())
        } else {
            Err(NetworkError::NetworkUnreachable)
        }
    }
    /// Checks custom command exit status
    pub fn check_custom_cmd(&self, cmd: &str) -> Result<(), NetworkError> {
        let success = process::Command::new("/bin/sh")
            .arg("-c")
            .arg(cmd)
            .env("__PK_HOSTS", self.hosts.join(" "))
            .output()
            .expect("No shell?")
            .status
            .success();
        if success {
            Ok(())
        } else {
            Err(NetworkError::NetworkUnreachable)
        }
    }

    // /// Sets port, for is_network_reachable
    // pub fn set_port(&mut self, port: u16) {
    //   self.port = Some(port);
    //   self.addresses = hosts_to_addresses(&self.hosts, self.port);
    // }
    /// Sets ping options, for is_ping_pong
    pub fn set_ping_opt(&mut self, ping_opt: String) {
        self.ping_opt = Some(ping_opt);
    }
    /// Sets timeout for direct connection
    pub fn set_timeout(&mut self, secs: u64) -> Result<(), NetworkError> {
        if secs > 0 {
            self.timeout = Duration::from_secs(secs);
            Ok(())
        } else {
            Err(NetworkError::InvalidTimeout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let hosts = vec![String::from("8.8.8.8")];
        let _ping = NetworkMonitor::new(hosts, None);
    }
    #[test]
    fn set_timeout() {
        let hosts = vec![String::from("8.8.8.8")];
        let mut network = NetworkMonitor::new(hosts, None);
        assert!(network.set_timeout(0).is_err());
        assert!(network.set_timeout(2).is_ok());
    }
    // Custom command
    #[test]
    fn custom_command() {
        let hosts = vec![String::from("127.0.0.1")];
        let custom = NetworkMonitor::new(hosts, None);
        assert!(custom.check_custom_cmd("echo").is_ok());
        assert!(custom.check_custom_cmd("cat __pk__test__file__").is_err());
    }
    // Ping
    #[test]
    fn ping_pong() {
        let hosts = vec![String::from("127.0.0.1")];
        let ping_opt = String::from("-c1");
        let mut ping = NetworkMonitor::new(hosts, None);
        ping.set_ping_opt(ping_opt);
        assert!(ping.is_ping_pong().is_ok());
    }
    #[test]
    fn no_ping_pong() {
        let hosts = vec![String::from("256.0.0.0")];
        let ping_opt = String::from("-c1");
        let mut ping = NetworkMonitor::new(hosts, None);
        ping.set_ping_opt(ping_opt);
        let err = ping.is_ping_pong();
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), NetworkError::NetworkUnreachable);
    }
    // Network
    #[test]
    fn is_network_reachable() {
        // Requires internet connection
        let hosts = vec![String::from("1.0.0.1")];
        let network = NetworkMonitor::new(hosts, Some(53));
        assert!(network.is_network_reachable().is_ok());
    }
    #[test]
    fn is_network_unreachable() {
        let hosts = vec![String::from("255.255.255.255")];
        let network = NetworkMonitor::new(hosts, Some(53));
        let err = network.is_network_reachable();
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), NetworkError::NetworkUnreachable);
    }
    // Check priorities
    #[test]
    fn check_priority_custom() {
        let hosts = vec![String::from("127.0.0.2")];
        let custom = NetworkMonitor::new(hosts, None);
        // ping and tcp would fail
        assert!(custom.check(&Some(String::from("echo")), true).is_ok());
    }
    #[test]
    fn check_priority_ping() {
        let hosts = vec![String::from("127.0.0.1")];
        let mut custom = NetworkMonitor::new(hosts, Some(0));
        custom.set_ping_opt(String::from("-c1"));
        // tcp would fail
        assert!(custom.check(&None, true).is_ok());
    }
    #[test]
    fn check_priority_tcp() {
        let hosts = vec![String::from("1.0.0.1")];
        let custom = NetworkMonitor::new(hosts, Some(53));
        // tcp would fail
        assert!(custom.check(&None, false).is_ok());
    }
}
