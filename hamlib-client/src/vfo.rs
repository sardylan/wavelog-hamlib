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
pub enum VFO {
    VFOA,
    VFOB,
    VFOC,
    CurrVfo,
    VFO,
    MEM,
    Main,
    Sub,
    TX,
    RX,
    None,
}

impl Display for VFO {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            VFO::VFOA => write!(f, "VFOA"),
            VFO::VFOB => write!(f, "VFOB"),
            VFO::VFOC => write!(f, "VFOC"),
            VFO::CurrVfo => write!(f, "currVfo"),
            VFO::VFO => write!(f, "VFO"),
            VFO::MEM => write!(f, "MEM"),
            VFO::Main => write!(f, "Main"),
            VFO::Sub => write!(f, "Sub"),
            VFO::TX => write!(f, "TX"),
            VFO::RX => write!(f, "RX"),
            VFO::None => write!(f, "None"),
        }
    }
}

impl FromStr for VFO {
    type Err = RigCtlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VFOA" => Ok(VFO::VFOA),
            "VFOB" => Ok(VFO::VFOB),
            "VFOC" => Ok(VFO::VFOC),
            "currVFO" => Ok(VFO::CurrVfo),
            "VFO" => Ok(VFO::VFO),
            "MEM" => Ok(VFO::MEM),
            "Main" => Ok(VFO::Main),
            "Sub" => Ok(VFO::Sub),
            "TX" => Ok(VFO::TX),
            "RX" => Ok(VFO::RX),
            "None" => Ok(VFO::None),
            _ => Err(RigCtlError::RawDataError(format!("Unable to parse VFO with string \"{}\"", &s))),
        }
    }
}
