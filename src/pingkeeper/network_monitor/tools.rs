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
use std::process;

use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

/// Pings a host and returns if it is reachable
fn ping(ping_opt: &str, host: &str) -> bool {
    process::Command::new("/bin/sh")
        .arg("-c")
        .arg(&format!("ping {} {}", ping_opt, host))
        .output()
        .expect("No shell?")
        .status
        .success()
}

/// Checks if a ping replies from one host at least
pub fn can_ping_some(hosts: Vec<String>, ping_opt: String) -> bool {
    if ping(&ping_opt, &hosts[0]) {
        return true;
    }
    let n = hosts.len();
    for result in hosts.with_threads(n).map(move |s| ping(&ping_opt, &s)) {
        if result {
            return true;
        }
    }
    false
}

/// Checks if a connection can be established to one address at least
pub fn can_connect_some(addresses: Vec<SocketAddr>, timeout: Duration) -> bool {
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

/// Gets hosts as network addresses
pub fn hosts_to_addresses(hosts: &[String], port: Option<u16>) -> Vec<SocketAddr> {
    hosts
        .iter()
        .map(|host| host_to_address(host, port))
        .filter(|addr| addr.is_some())
        .map(|addr| addr.unwrap())
        .collect()
}

/// Gets host as network address
fn host_to_address(host: &str, port: Option<u16>) -> Option<SocketAddr> {
    if let Ok(addr) = host.parse::<SocketAddr>() {
        Some(addr)
    } else {
        // IPv6 or IPv4
        match (port, host.parse::<IpAddr>()) {
            (Some(port), Ok(ip)) => Some(SocketAddr::new(ip, port)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_function() {
        assert!(ping("-c1", "127.0.0.1"));
        assert!(!ping("-c1", "256.0.0.0"));
    }

    #[test]
    fn host_with_port_to_address() {
        for host in ["1.1.1.1:53", "[2020::1]:8080", "[::1]:1"].iter() {
            let address = host_to_address(host, None);
            println!("{:?}", address);
            assert!(address.is_some());
        }
    }
    #[test]
    fn host_without_port_to_address() {
        let port = Some(8080);
        for host in ["127.0.0.1", "::1", "2020::1", "::ffff:10.1.2.3"].iter() {
            let address = host_to_address(host, port);
            println!("{:?}", address);
            assert!(address.is_some());
        }
    }
}
