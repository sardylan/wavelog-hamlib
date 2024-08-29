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

use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum RigCtlError {
    ConnectionError(String),
    AlreadyConnected,
    RawDataError(String),
    ResponseParsing(String),
    CommunicationTimeout,
}

impl Display for RigCtlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RigCtlError::ConnectionError(message) => { write!(f, "Connection error: {}", message) }
            RigCtlError::AlreadyConnected => { write!(f, "Already connected") }
            RigCtlError::RawDataError(message) => { write!(f, "Raw data error: {}", message) }
            RigCtlError::ResponseParsing(message) => { write!(f, "Response parsing error: {}", message) }
            RigCtlError::CommunicationTimeout => { write!(f, "Communication timeout") }
        }
    }
}

impl From<Error> for RigCtlError {
    fn from(value: Error) -> Self {
        RigCtlError::ConnectionError(value.to_string())
    }
}

impl From<FromUtf8Error> for RigCtlError {
    fn from(value: FromUtf8Error) -> Self {
        RigCtlError::RawDataError(value.to_string())
    }
}
