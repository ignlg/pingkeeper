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

use pipeliner::Pipeline;
use rand::{seq::SliceRandom, thread_rng};
use std::process;

use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

const DEFAULT_TIMEOUT: u64 = 2;

/// Ping errors
#[derive(Debug, PartialEq, Eq)]
pub enum NetworkError {
  NetworkUnreachable,
  NoHostsToCheck,
  NoPort,
  InvalidTimeout,
}

/// Ping a host and return if it is reachable
fn ping(ping_opt: &str, host: &str) -> bool {
  process::Command::new("/bin/sh")
    .arg("-c")
    .arg(&format!("ping {} {}", ping_opt, host))
    .output()
    .expect("No shell?")
    .status
    .success()
}

/// Check if it can connect to at least one addresses
fn can_connect_some(addresses: Vec<SocketAddr>, timeout: Duration) -> bool {
  if TcpStream::connect_timeout(&addresses[0], timeout).is_ok() {
    return true;
  }
  let n = addresses.len();
  for result in addresses
    .with_threads(n)
    .map(move |addr| TcpStream::connect_timeout(&addr, timeout).is_ok())
  {
    if result {
      return true;
    }
  }
  false
}

/// Ping
pub struct NetworkMonitor {
  hosts: Vec<String>,
  port: Option<u32>,
  ping_opt: Option<String>,
  timeout: Duration,
}

// Public
impl NetworkMonitor {
  pub fn new(hosts: Vec<String>) -> Self {
    NetworkMonitor {
      hosts,
      port: None,
      ping_opt: None,
      timeout: Duration::from_secs(DEFAULT_TIMEOUT),
    }
  }
  /// Check if ping has pong
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
    if ping(&ping_opt, &hosts[0]) {
      return Ok(());
    }
    let n = self.hosts.len();
    for result in hosts.with_threads(n).map(move |s| ping(&ping_opt, &s)) {
      if result {
        return Ok(());
      }
    }
    Err(NetworkError::NetworkUnreachable)
  }
  /// Check if network is reachable
  pub fn is_network_reachable(&self) -> Result<(), NetworkError> {
    if self.hosts.is_empty() {
      return Err(NetworkError::NoHostsToCheck);
    } else if self.port.is_none() {
      return Err(NetworkError::NoPort);
    }
    let port = self.port.unwrap();
    let mut addresses: Vec<SocketAddr> = self.get_addresses(port);
    if addresses.is_empty() {
      return Err(NetworkError::NoHostsToCheck);
    }
    let mut rng = thread_rng();
    addresses.shuffle(&mut rng);
    if can_connect_some(addresses, self.timeout) {
      Ok(())
    } else {
      Err(NetworkError::NetworkUnreachable)
    }
  }

  /// Set port, for is_network_reachable
  pub fn set_port(&mut self, port: u32) {
    self.port = Some(port);
  }
  /// Set ping options, for is_ping_pong
  pub fn set_ping_opt(&mut self, ping_opt: String) {
    self.ping_opt = Some(ping_opt);
  }
  /// Set timeout
  pub fn set_timeout(&mut self, secs: u64) -> Result<(), NetworkError> {
    if secs > 0 {
      self.timeout = Duration::from_secs(secs);
      Ok(())
    } else {
      Err(NetworkError::InvalidTimeout)
    }
  }
}

// Private
impl NetworkMonitor {
  /// Get hosts as network addresses
  fn get_addresses(&self, port: u32) -> Vec<SocketAddr> {
    self
      .hosts
      .iter()
      .map(|addr| {
        let ip_port = format!("{}:{}", addr, port);
        print!("{}", ip_port);
        ip_port.parse::<SocketAddr>()
      })
      .filter(|addr| addr.is_ok())
      .map(|addr| addr.unwrap())
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let hosts = vec![String::from("8.8.8.8")];
    let _ping = NetworkMonitor::new(hosts);
  }
  #[test]
  fn set_timeout() {
    let hosts = vec![String::from("8.8.8.8")];
    let mut network = NetworkMonitor::new(hosts);
    assert!(network.set_timeout(0).is_err());
    assert!(network.set_timeout(2).is_ok());
  }
  // Ping
  #[test]
  fn ping_pong() {
    let hosts = vec![String::from("127.0.0.1")];
    let ping_opt = String::from("-c1");
    let mut ping = NetworkMonitor::new(hosts);
    ping.set_ping_opt(ping_opt);
    assert!(ping.is_ping_pong().is_ok());
  }
  #[test]
  fn no_ping_pong() {
    let hosts = vec![String::from("256.0.0.0")];
    let ping_opt = String::from("-c1");
    let mut ping = NetworkMonitor::new(hosts);
    ping.set_ping_opt(ping_opt);
    let err = ping.is_ping_pong();
    assert!(err.is_err());
    assert_eq!(err.unwrap_err(), NetworkError::NetworkUnreachable);
  }
  // Network
  #[test]
  fn get_hosts_addresses() {
    let hosts = vec![String::from("127.0.0.1")];
    let port = 53;
    let mut network = NetworkMonitor::new(hosts);
    network.set_port(port);
    assert_eq!(network.get_addresses(port).len(), 1);
  }
  #[test]
  fn is_network_reachable() {
    // Requires internet connection
    let hosts = vec![String::from("1.0.0.1")];
    let mut network = NetworkMonitor::new(hosts);
    network.set_port(53);
    assert!(network.is_network_reachable().is_ok());
  }
  #[test]
  fn is_network_unreachable() {
    let hosts = vec![String::from("255.255.255.255")];
    let mut network = NetworkMonitor::new(hosts);
    network.set_port(53);
    let err = network.is_network_reachable();
    assert!(err.is_err());
    assert_eq!(err.unwrap_err(), NetworkError::NetworkUnreachable);
  }
  #[test]
  fn ping_function() {
    assert!(ping("-c1", "127.0.0.1"));
    assert!(!ping("-c1", "256.0.0.0"));
  }
}
