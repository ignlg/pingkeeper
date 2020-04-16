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

/// Ping errors
#[derive(Debug, PartialEq, Eq)]
pub enum PingError {
  NetworkUnreachable,
  NoHostsToPing,
}

/// Ping
pub struct Ping {
  ping_opt: String,
  hosts: Vec<String>,
}

impl Ping {
  pub fn new(hosts: Vec<String>, ping_opt: String) -> Self {
    Self { ping_opt, hosts }
  }
  /// Check if network is reachable
  pub fn is_network_reachable(&self) -> Result<(), PingError> {
    if self.hosts.is_empty() {
      return Err(PingError::NoHostsToPing);
    }
    let mut hosts = self.hosts.to_vec();
    let mut rng = thread_rng();
    hosts.shuffle(&mut rng);
    if ping(&self.ping_opt, &hosts[0]) {
      return Ok(());
    }
    let n = self.hosts.len();
    let ping_opt = String::from(&self.ping_opt);
    for result in hosts.with_threads(n).map(move |s| ping(&ping_opt, &s)) {
      if result {
        return Ok(());
      }
    }
    Err(PingError::NetworkUnreachable)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let hosts = vec![String::from("8.8.8.8")];
    let ping_opt = String::from("-c1");
    let _ping = Ping::new(hosts, ping_opt);
  }
  #[test]
  fn network_reachable() {
    let hosts = vec![String::from("127.0.0.1")];
    let ping_opt = String::from("-c1");
    let ping = Ping::new(hosts, ping_opt);
    assert!(ping.is_network_reachable().is_ok());
  }
  #[test]
  fn network_unreachable() {
    let hosts = vec![String::from("256.0.0.0")];
    let ping_opt = String::from("-c1");
    let ping = Ping::new(hosts, ping_opt);
    assert!(ping.is_network_reachable().is_err());
  }
  #[test]
  fn ping_function() {
    assert!(ping("-c1", "127.0.0.1"));
    assert!(!ping("-c1", "256.0.0.0"));
  }
}
