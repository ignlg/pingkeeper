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

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::fmt;
use std::io::{self};
use std::process;
use std::str::FromStr;

/// Executor errors
#[derive(Debug, PartialEq, Eq)]
pub enum ExecutorError {
  NoStatus,
  SignalNotSent,
}
impl fmt::Display for ExecutorError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

/// Executor
#[derive(Debug)]
pub struct Executor {
  command: String,
  signal: Option<Signal>,
  child: Option<process::Child>,
  error: Option<io::Error>,
}

// Public impl
impl Executor {
  /// Create a new Executor instance
  pub fn new(command: String) -> Self {
    Self {
      command,
      signal: Some(Signal::SIGINT),
      child: None,
      error: None,
    }
  }
  /// Spawn a child process
  pub fn execute(&mut self, quiet: bool) -> bool {
    let mut cmd = process::Command::new("/bin/sh");
    cmd.arg("-c").arg(&self.command);
    if quiet {
      cmd
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped());
    }
    match cmd.spawn() {
      Ok(child) => {
        self.child = Some(child);
        self.error = None;
        true
      }
      Err(error) => {
        self.error = Some(error);
        self.child = None;
        false
      }
    }
  }
  /// Send signal to child process, if any
  pub fn kill(&mut self) -> Result<(), ExecutorError> {
    if let Some(child) = &mut self.child {
      if kill(Pid::from_raw(child.id() as i32), self.signal).is_err() {
        return Err(ExecutorError::SignalNotSent);
      }
      child.wait().ok();
      self.child = None;
    }
    Ok(())
  }
  /// Is child process alive?
  pub fn is_alive(&mut self) -> Result<bool, ExecutorError> {
    if let Some(child) = &mut self.child {
      let status = child.try_wait();
      return match status {
        Ok(None) => Ok(true),
        Err(_) => Err(ExecutorError::NoStatus),
        Ok(_) => Ok(false),
      };
    }
    Ok(false)
  }
  /// Get child process PID, if any
  pub fn get_pid(&mut self) -> Option<u32> {
    if let Ok(is_alive) = self.is_alive() {
      if is_alive {
        if let Some(child) = &mut self.child {
          return Some(child.id());
        }
      }
    }
    None
  }
  /// Set signal
  pub fn set_signal(&mut self, signal: &str) {
    if let Ok(signal) = Signal::from_str(signal) {
      self.signal = Some(signal);
    } else {
      self.signal = None;
    };
  }
}
