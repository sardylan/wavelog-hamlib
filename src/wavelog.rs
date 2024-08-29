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

use chrono::{DateTime, Utc};
use hamlib_client::adif::{Mode, PropagationMode};
use reqwest::{Method, Url};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Update {
    pub radio: String,
    pub frequency: u64,
    pub mode: Mode,
    pub frequency_rx: Option<u64>,
    pub mode_rx: Option<Mode>,
    pub prop_mode: Option<PropagationMode>,
    pub power: Option<u32>,
    pub sat_name: Option<String>,
}

impl Display for Update {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.radio, self.frequency, self.mode)
    }
}

#[derive(Serialize)]
struct Request {
    key: String,
    #[serde(serialize_with = "serialize_timestamp")]
    timestamp: DateTime<Utc>,
    radio: String,
    frequency: u64,
    mode: String,
    frequency_rx: u64,
    mode_rx: String,
    prop_mode: String,
    power: u32,
    sat_name: String,
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} - {}: {} {} {} {} {} {} {}",
               self.key, self.timestamp, self.radio, self.frequency,
               self.mode, self.frequency_rx, self.mode_rx, self.prop_mode,
               self.power, self.sat_name)
    }
}

impl Request {
    fn generate(key: &String, update: Update) -> Self {
        Self {
            key: key.clone(),
            timestamp: Utc::now(),
            radio: update.radio.clone(),
            frequency: update.frequency,
            mode: update.mode.to_string(),
            frequency_rx: update.frequency_rx.unwrap_or(0),
            mode_rx: update.mode_rx.unwrap_or(Mode::None).to_string(),
            prop_mode: update.prop_mode.unwrap_or(PropagationMode::None).to_string(),
            power: update.power.unwrap_or(0),
            sat_name: update.sat_name.unwrap_or("".to_string()),
        }
    }
}

#[derive(Deserialize)]
struct Response {
    pub status: String,
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct Client {
    url: Url,
    key: String,
}

impl Client {
    pub(crate) fn new(url: &str, key: &str) -> Self {
        Self {
            url: Url::from_str(url).unwrap(),
            key: key.to_string(),
        }
    }

    pub async fn send_update(&self, update: Update) -> Result<bool, reqwest::Error> {
        log::debug!("Sending data to Wavelog");

        let request_body = Request::generate(&self.key, update);
        log::trace!("Request: {}", &request_body);

        let url = self.url.join("/api/radio").unwrap();
        log::trace!("URL: {}", &url);

        let response_body: Response = reqwest::Client::new()
            .request(Method::POST, url.as_str())
            .json(&request_body)
            .send().await?
            .json().await?;
        log::trace!("Response: {}", &response_body);
        Ok(response_body.status == "success")
    }
}

fn serialize_timestamp<S>(timestamp: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = timestamp.format("%Y/%m/%d %H:%M").to_string();
    serializer.serialize_str(&format)
}
