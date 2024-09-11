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

use clap::crate_name;
use clap::value_parser;
use clap::{ArgAction, Parser};
use log::Level;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Config {
    #[arg(
        short = 'l',
        long,
        action = ArgAction::Set,
        default_value = "WARN",
        help = "Log level",
        long_help = "Set the level of logging messages"
    )]
    pub log_level: Level,

    #[arg(
        short = 'i',
        long,
        action = ArgAction::Set,
        default_value = "2000",
        help = "Update interval",
        long_help = "Set the duration of the interval between two updates (milliseconds)"
    )]
    pub interval: u64,

    #[arg(
        short = 'w',
        long,
        action = ArgAction::Set,
        required = true,
        help = "Wavelog URL",
        long_help = "Set the main URL of Wavelog instance"
    )]
    pub wavelog_url: String,

    #[arg(
        short = 'k',
        long,
        action = ArgAction::Set,
        required = true,
        help = "Wavelog API-key",
        long_help = "Wavelog API-key of the user"
    )]
    pub wavelog_key: String,

    #[arg(short = 'r',
        long,
        action = ArgAction::Set,
        default_value = crate_name!(),
        help = "Wavelog radio name",
        long_help = "Set the name of the radio that appears on Wavelog"
    )]
    pub wavelog_radio: String,

    #[arg(
        short = 'H',
        long,
        action = ArgAction::Set,
        default_value = "localhost",
        help = "rigctld address",
        long_help = "rigctld hostname or address"
    )]
    pub rigctl_host: String,

    #[arg(short = 'p',
        long,
        action = ArgAction::Set,
        value_parser(value_parser!(u16)),
        default_value = "4532",
        help = "rigctld port",
        long_help = "Set the port in which rigctld is listening"
    )]
    pub rigctl_port: u16,

    #[arg(
        short = 't',
        long,
        action = ArgAction::Set,
        default_value = "3000",
        help = "rigctld timeout",
        long_help = "Set the amount of time to wait for rigctl response (in milliseconds)"
    )]
    pub rigctl_timeout: u64,
}
