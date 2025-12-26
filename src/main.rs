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

mod config;
mod errors;
mod log;
mod wavelog;

use crate::config::Config;
use crate::errors::WavelogHamlibError;
use crate::wavelog::Update;
use clap::Parser;
use hamlib_client::adif::{Mode, PropagationMode};
use hamlib_client::RigCtlClient;
use std::string::String;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() {
    let configuration = Config::parse();

    log::configure(&configuration.log_level);

    match program(&configuration).await {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e)
        }
    }
}

async fn program(configuration: &Config) -> Result<(), WavelogHamlibError> {
    trace!(
        "Creating Wavelog client for {} with radio name \"{}\"",
        &configuration.wavelog_url,
        configuration.wavelog_radio
    );
    let wavelog_client =
        wavelog::Client::new(&configuration.wavelog_url, &configuration.wavelog_key);

    trace!(
        "Creating client for {}:{}",
        &configuration.rigctl_host,
        configuration.rigctl_port
    );
    let mut rigctl = RigCtlClient::new(&configuration.rigctl_host, configuration.rigctl_port, None);
    rigctl.set_communication_timeout(configuration.rigctl_timeout);

    rigctl.connect().await?;
    info!("Connected");

    loop {
        debug!("Getting info from Hamlib");

        let vfo = rigctl.get_vfo().await?;
        trace!("Rig vfo: {}", &vfo);
        let rx_vfo = vfo.vfo;
        debug!("RX vfo: {}", &rx_vfo);

        let split_vfo = rigctl.get_split_vfo().await?;
        trace!("Rig split_vfo: {}", &split_vfo);
        let tx_vfo = split_vfo.tx_vfo;
        debug!("TX vfo: {}", &tx_vfo);

        let force_mode = Mode::from(&configuration.force_mode.as_str());
        debug!("Force mode: {}", &force_mode);

        let (rx_mode, tx_mode) = if force_mode == Mode::None {
            let rx_mode = rigctl.get_mode(rx_vfo).await?;
            let tx_mode = rigctl.get_mode(rx_vfo).await?;
            (Mode::from(rx_mode.mode), Mode::from(tx_mode.mode))
        } else {
            (force_mode, force_mode)
        };
        trace!("{} Mode: {}", &rx_vfo, &rx_mode);
        trace!("{} Mode: {}", &tx_vfo, &tx_mode);

        let rx_freq = rigctl.get_freq(rx_vfo).await?;
        trace!("{}: {}", &rx_vfo, &rx_freq);
        let tx_freq = rigctl.get_freq(tx_vfo).await?;
        trace!("{}: {}", &tx_vfo, &tx_freq);

        let update_prop_mode = if !&configuration.sat.is_empty() {
            debug!("Enabling SAT propagation mode");
            Some(PropagationMode::SAT)
        } else {
            None
        };

        let update_tx_freq =
            if &configuration.sat == "QO-100" && tx_freq.frequency == rx_freq.frequency {
                debug!("Manually setting TX Frequency for QO-100 satellite activity");
                tx_freq.frequency - 8089500000
            } else {
                tx_freq.frequency
            };

        debug!("Preparing update");
        let update = Update {
            radio: String::from(&configuration.wavelog_radio),
            frequency: update_tx_freq,
            mode: tx_mode,
            frequency_rx: Some(rx_freq.frequency),
            mode_rx: Some(rx_mode),
            prop_mode: update_prop_mode,
            power: None,
            sat_name: Some(String::from(&configuration.sat)).filter(String::is_empty),
        };
        trace!("Update: {}", &update);

        debug!("Sending update to Wavelog");
        let result = wavelog_client.send_update(update).await;
        trace!("Result: {:?}", &result);
        match result {
            Ok(response) => {
                if !response {
                    warn!("Error sending update to Wavelog");
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }

        debug!("Sleeping");
        sleep(Duration::from_millis(configuration.interval)).await;
    }
}
