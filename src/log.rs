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

use tracing::Level;
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn configure(level: &Level) {
    tracing_log::LogTracer::init().expect("Failed to set logger");

    let crate_name = clap::crate_name!().replace('-', "_");

    let filter = EnvFilter::try_new(format!("{}={},warn", crate_name, level))
        .unwrap_or_else(|_| EnvFilter::new(format!("{}={},warn", crate_name, Level::INFO)));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .try_init()
        .ok();
}
