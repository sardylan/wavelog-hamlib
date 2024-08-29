/*
 * Copyright (C) 2024 Luca Cireddu <sardylan@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::error::RigCtlError;
use crate::vfo::VFO;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Response {
    pub query_vfo: VFO,
    pub frequency: u64,
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Query VFO: {} - Frequency: {}", self.query_vfo, self.frequency)
    }
}

pub fn parse(line: &str) -> Result<Response, RigCtlError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^get_freq: (?P<query_vfo>[a-zA-Z0-9]+)\|Frequency: (?P<frequency>[0-9]+)\|RPRT 0$").unwrap();
    }

    let captures = RE.captures(&line).ok_or(RigCtlError::ResponseParsing("Unable to capture groups".to_string()))?;
    log::trace!("Captures: {:?}", &captures);

    let query_vfo = captures.name("query_vfo").ok_or(RigCtlError::ResponseParsing("Invalid query_vfo group".to_string()))?.as_str();
    let frequency = captures.name("frequency").ok_or(RigCtlError::ResponseParsing("Invalid frequency group".to_string()))?.as_str();

    Ok(Response {
        query_vfo: VFO::from_str(&query_vfo)?,
        frequency: frequency.parse::<u64>().map_err(|e| { RigCtlError::ResponseParsing(e.to_string()) })?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_freq() {
        let input = r"get_freq: None|Frequency: 14074000|RPRT 0";
        let expected = Response { query_vfo: VFO::None, frequency: 14074000u64 };
        let actual = parse(input);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
