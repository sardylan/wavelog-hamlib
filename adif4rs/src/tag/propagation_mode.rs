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

use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum PropagationMode {
    AS,
    AUE,
    AUR,
    BS,
    ECH,
    EME,
    ES,
    F2,
    FAI,
    GWAVE,
    INTERNET,
    ION,
    IRL,
    LOS,
    MS,
    RPT,
    RS,
    SAT,
    TEP,
    TR,
    None,
}

impl Display for PropagationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PropagationMode::AS => { write!(f, "Aircraft Scatter") }
            PropagationMode::AUE => { write!(f, "Aurora-E") }
            PropagationMode::AUR => { write!(f, "Aurora") }
            PropagationMode::BS => { write!(f, "Back scatter") }
            PropagationMode::ECH => { write!(f, "EchoLink") }
            PropagationMode::EME => { write!(f, "Earth-Moon-Earth") }
            PropagationMode::ES => { write!(f, "Sporadic E") }
            PropagationMode::F2 => { write!(f, "F2 Reflection") }
            PropagationMode::FAI => { write!(f, "Field Aligned Irregularities") }
            PropagationMode::GWAVE => { write!(f, "Ground Wave") }
            PropagationMode::INTERNET => { write!(f, "Internet-assisted") }
            PropagationMode::ION => { write!(f, "Ionoscatter") }
            PropagationMode::IRL => { write!(f, "IRLP") }
            PropagationMode::LOS => { write!(f, "Line of Sight (includes transmission through obstacles such as walls)") }
            PropagationMode::MS => { write!(f, "Meteor scatter") }
            PropagationMode::RPT => { write!(f, "Terrestrial or atmospheric repeater or transponder") }
            PropagationMode::RS => { write!(f, "Rain scatter") }
            PropagationMode::SAT => { write!(f, "Satellite") }
            PropagationMode::TEP => { write!(f, "Trans-equatorial") }
            PropagationMode::TR => { write!(f, "Tropospheric ducting") }
            PropagationMode::None => { write!(f, "") }
        }
    }
}
