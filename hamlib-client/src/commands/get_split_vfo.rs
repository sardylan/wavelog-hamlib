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
    pub split: bool,
    pub tx_vfo: VFO,
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Query VFO: {} - Split: {} - TX VFO: {}", self.query_vfo, self.split, self.tx_vfo)
    }
}

pub fn parse(line: &str) -> Result<Response, RigCtlError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^get_split_vfo: (?P<query_vfo>[a-zA-Z0-9]+)\|Split: (?P<split>[01])\|TX VFO: (?P<tx_vfo>[a-zA-Z0-9]+)\|RPRT 0$").unwrap();
    }

    let captures = RE.captures(&line).ok_or(RigCtlError::ResponseParsing("Unable to capture groups".to_string()))?;
    log::trace!("Captures: {:?}", &captures);

    let query_vfo = captures.name("query_vfo").ok_or(RigCtlError::ResponseParsing("Invalid query_vfo group".to_string()))?.as_str();
    let split = captures.name("split").ok_or(RigCtlError::ResponseParsing("Invalid query_vfo group".to_string()))?.as_str();
    let tx_vfo = captures.name("tx_vfo").ok_or(RigCtlError::ResponseParsing("Invalid vfo group".to_string()))?.as_str();

    Ok(Response {
        query_vfo: VFO::from_str(&query_vfo)?,
        split: split == "1",
        tx_vfo: VFO::from_str(&tx_vfo)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vfo_split_enabled() {
        let input = r"get_split_vfo: None|Split: 1|TX VFO: VFOA|RPRT 0";
        let expected = Response { query_vfo: VFO::None, split: true, tx_vfo: VFO::VFOA };
        let actual = parse(input);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn test_get_vfo_split_disabled() {
        let input = r"get_split_vfo: None|Split: 0|TX VFO: VFOA|RPRT 0";
        let expected = Response { query_vfo: VFO::None, split: false, tx_vfo: VFO::VFOA };
        let actual = parse(input);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
