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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum LogLevel {
  QUIET = 0,
  ERROR,
  WARN,
  INFO,
  DEBUG,
}

impl From<u32> for LogLevel {
  fn from(number: u32) -> Self {
    match number {
      x if x == LogLevel::QUIET as u32 => LogLevel::QUIET,
      x if x == LogLevel::ERROR as u32 => LogLevel::ERROR,
      x if x == LogLevel::WARN as u32 => LogLevel::WARN,
      x if x == LogLevel::INFO as u32 => LogLevel::INFO,
      _ => LogLevel::DEBUG,
    }
  }
}

/// Filter and write output to stdout/stderr
pub fn logger(verbose: LogLevel) -> impl Fn(LogLevel, String) -> () {
  move |level: LogLevel, message: String| match (level, verbose) {
    (LogLevel::ERROR, v) if v >= LogLevel::ERROR => eprintln!("PK error: {}", message),
    (LogLevel::WARN, v) if v >= LogLevel::WARN => println!("PK warn:  {}", message),
    (LogLevel::INFO, v) if v >= LogLevel::INFO => println!("PK info:  {}", message),
    (LogLevel::DEBUG, v) if v >= LogLevel::DEBUG => println!("PK debug: {}", message),
    (_, _) => (),
  }
}
