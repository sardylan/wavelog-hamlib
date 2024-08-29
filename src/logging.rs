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

use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

pub fn configure() {
    let console_encoder = PatternEncoder::new("{date(%Y-%m-%dT%H:%M:%S%.6f%:z)(utc)} {module} {highlight({level})} {message}{n}");

    let console_appender = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(console_encoder))
        .build();

    log4rs::init_config(Config::builder()
        .appender(Appender::builder().build("stderr", Box::new(console_appender)))
        .build(Root::builder().appender("stderr").build(LevelFilter::Trace))
        .unwrap()
    ).unwrap();
}