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

use lazy_static::lazy_static;

pub struct Config {
    pub wavelog_url: String,
    pub wavelog_key: String,
    pub wavelog_radio: String,
    pub rigctl_host: String,
    pub rigctl_port: u16,
    pub rigctl_communication_timeout: u64,
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

pub fn get_config() -> Config {
    Config {
        wavelog_url: "".to_string(),
        wavelog_key: "".to_string(),
        wavelog_radio: "".to_string(),
        rigctl_host: "".to_string(),
        rigctl_port: 4532,
        rigctl_communication_timeout: 3000,
    }
}
