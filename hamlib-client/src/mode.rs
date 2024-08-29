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
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    USB,
    LSB,
    CW,
    CWR,
    RTTY,
    RTTYR,
    AM,
    FM,
    WFM,
    AMS,
    PKTLSB,
    PKTUSB,
    PKTFM,
    ECSSUSB,
    ECSSLSB,
    FAX,
    SAM,
    SAL,
    SAH,
    DSB,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Mode::USB => write!(f, "USB"),
            Mode::LSB => write!(f, "LSB"),
            Mode::CW => write!(f, "CW"),
            Mode::CWR => write!(f, "CWR"),
            Mode::RTTY => write!(f, "RTTY"),
            Mode::RTTYR => write!(f, "RTTYR"),
            Mode::AM => write!(f, "AM"),
            Mode::FM => write!(f, "FM"),
            Mode::WFM => write!(f, "WFM"),
            Mode::AMS => write!(f, "AMS"),
            Mode::PKTLSB => write!(f, "PKTLSB"),
            Mode::PKTUSB => write!(f, "PKTUSB"),
            Mode::PKTFM => write!(f, "PKTFM"),
            Mode::ECSSUSB => write!(f, "ECSSUSB"),
            Mode::ECSSLSB => write!(f, "ECSSLSB"),
            Mode::FAX => write!(f, "FAX"),
            Mode::SAM => write!(f, "SAM"),
            Mode::SAL => write!(f, "SAL"),
            Mode::SAH => write!(f, "SAH"),
            Mode::DSB => write!(f, "DSB"),
        }
    }
}

impl FromStr for Mode {
    type Err = RigCtlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USB" => Ok(Mode::USB),
            "LSB" => Ok(Mode::LSB),
            "CW" => Ok(Mode::CW),
            "CWR" => Ok(Mode::CWR),
            "RTTY" => Ok(Mode::RTTY),
            "RTTYR" => Ok(Mode::RTTYR),
            "AM" => Ok(Mode::AM),
            "FM" => Ok(Mode::FM),
            "WFM" => Ok(Mode::WFM),
            "AMS" => Ok(Mode::AMS),
            "PKTLSB" => Ok(Mode::PKTLSB),
            "PKTUSB" => Ok(Mode::PKTUSB),
            "PKTFM" => Ok(Mode::PKTFM),
            "ECSSUSB" => Ok(Mode::ECSSUSB),
            "ECSSLSB" => Ok(Mode::ECSSLSB),
            "FAX" => Ok(Mode::FAX),
            "SAM" => Ok(Mode::SAM),
            "SAL" => Ok(Mode::SAL),
            "SAH" => Ok(Mode::SAH),
            "DSB" => Ok(Mode::DSB),
            _ => Err(RigCtlError::RawDataError(format!("Unable to parse Mode with string \"{}\"", &s))),
        }
    }
}

impl From<Mode> for adif4rs::tag::mode::Mode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::USB => { Self::SSB }
            Mode::LSB => { Self::SSB }
            Mode::CW => { Self::CW }
            Mode::CWR => { Self::CW }
            Mode::RTTY => { Self::RTTY }
            Mode::RTTYR => { Self::RTTY }
            Mode::AM => { Self::AM }
            Mode::FM => { Self::FM }
            Mode::WFM => { Self::FM }
            Mode::AMS => { Self::AM }
            Mode::PKTLSB => { Self::PKT }
            Mode::PKTUSB => { Self::PKT }
            Mode::PKTFM => { Self::PKT }
            Mode::ECSSUSB => { Self::AM }
            Mode::ECSSLSB => { Self::AM }
            Mode::FAX => { Self::FAX }
            Mode::SAM => { Self::AM }
            Mode::SAL => { Self::AM }
            Mode::SAH => { Self::AM }
            Mode::DSB => { Self::AM }
        }
    }
}
