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

use hamlib_client::error::RigCtlError;
use std::fmt;

#[derive(Debug)]
pub enum WavelogHamlibError {
    Hamlib(RigCtlError),
    Wavelog(reqwest::Error),
}

impl From<RigCtlError> for WavelogHamlibError {
    fn from(value: RigCtlError) -> Self {
        WavelogHamlibError::Hamlib(value)
    }
}

impl From<reqwest::Error> for WavelogHamlibError {
    fn from(value: reqwest::Error) -> Self {
        WavelogHamlibError::Wavelog(value)
    }
}

impl fmt::Display for WavelogHamlibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            WavelogHamlibError::Hamlib(e) => write!(f, "Hamlib error: {}", &e),
            WavelogHamlibError::Wavelog(e) => write!(f, "Wavelog error: {}", &e),
        }
    }
}
