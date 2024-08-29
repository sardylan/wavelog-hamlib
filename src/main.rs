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

mod errors;
mod config;
mod logging;
mod wavelog;

use crate::config::Config;
use crate::errors::WavelogHamlibError;
use crate::wavelog::Update;
use adif4rs::tag::mode::Mode;
use clap::Parser;
use hamlib_client::RigCtlClient;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    logging::configure();

    match program().await {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e)
        }
    }
}

async fn program() -> Result<(), WavelogHamlibError> {
    let configuration = Config::parse();

    log::trace!("Creating Wavelog client for {} with radio name \"{}\"", &configuration.wavelog_url, configuration.wavelog_radio);
    let wavelog_client = wavelog::Client::new(&configuration.wavelog_url, &configuration.wavelog_key);

    log::trace!("Creating client for {}:{}", &configuration.rigctl_host, configuration.rigctl_port);
    let mut rigctl = RigCtlClient::new(&configuration.rigctl_host, configuration.rigctl_port, None);
    rigctl.set_communication_timeout(configuration.rigctl_timeout);

    rigctl.connect().await?;
    log::info!("Connected");

    loop {
        log::debug!("Getting info from Hamlib");

        let vfo = rigctl.get_vfo().await?;
        log::trace!("Rig vfo: {}", &vfo);
        let rx_vfo = vfo.vfo;
        log::debug!("RX vfo: {}", &rx_vfo);

        let split_vfo = rigctl.get_split_vfo().await?;
        log::trace!("Rig split_vfo: {}", &split_vfo);
        let tx_vfo = split_vfo.tx_vfo;
        log::debug!("TX vfo: {}", &tx_vfo);

        let rx_mode = rigctl.get_mode(rx_vfo).await?;
        log::trace!("{}: {}", &rx_vfo, &rx_mode);
        let rx_freq = rigctl.get_freq(rx_vfo).await?;
        log::trace!("{}: {}", &rx_vfo, &rx_freq);

        let tx_mode = rigctl.get_split_mode(rx_vfo).await?;
        log::trace!("{}: {}", &tx_vfo, &tx_mode);
        let tx_freq = rigctl.get_split_freq(rx_vfo).await?;
        log::trace!("{}: {}", &tx_vfo, &tx_freq);

        log::debug!("Preparing update");
        let update = Update {
            radio: String::from(&configuration.wavelog_radio),
            frequency: tx_freq.frequency,
            mode: Mode::from(tx_mode.mode),
            frequency_rx: Some(rx_freq.frequency),
            mode_rx: Some(Mode::from(rx_mode.mode)),
            prop_mode: None,
            power: None,
            sat_name: None,
        };
        log::trace!("Update: {}", &update);

        log::debug!("Sending update to Wavelog");
        let result = wavelog_client.send_update(update).await;
        log::trace!("Result: {:?}", &result);
        match result {
            Ok(response) => {
                if !response {
                    log::warn!("Error sending update to Wavelog");
                }
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }

        log::debug!("Sleeping");
        sleep(Duration::from_millis(1000)).await;
    }
}
