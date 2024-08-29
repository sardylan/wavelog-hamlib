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

pub mod error;
pub mod mode;
pub mod vfo;
pub mod commands;

use crate::commands::{get_freq, get_info, get_mode, get_split_freq, get_split_mode, get_split_vfo, get_vfo};
use crate::error::RigCtlError;
use crate::vfo::VFO;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time;

pub struct RigCtlClient {
    host: String,
    port: u16,
    stream: Option<TcpStream>,
    timeout: Duration,
}

impl RigCtlClient {
    pub fn new(host: &str, port: u16, timeout: Option<u64>) -> Self {
        Self {
            host: String::from(host),
            port,
            stream: None,
            timeout: Duration::from_millis(timeout.unwrap_or(1000)),
        }
    }

    pub async fn connect(&mut self) -> Result<(), RigCtlError> {
        if self.is_connected() {
            return Err(RigCtlError::AlreadyConnected);
        }

        let connection_string = format!("{}:{}", self.host, self.port);

        let stream = TcpStream::connect(connection_string).await?;
        self.stream = Some(stream);

        Ok(())
    }

    pub fn disconnect(&mut self) {
        if !self.is_connected() {
            return;
        }

        self.stream = None;
    }

    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    pub fn set_communication_timeout(&mut self, timeout: u64) {
        self.timeout = Duration::from_millis(timeout);
    }

    pub async fn get_info(&mut self) -> Result<get_info::Response, RigCtlError> {
        let cmd = "get_info".to_string();
        let response = self.execute_command(&cmd).await?;
        get_info::parse(&response)
    }

    pub async fn get_mode(&mut self, vfo: VFO) -> Result<get_mode::Response, RigCtlError> {
        let cmd = format!("get_mode {}", vfo);
        let response = self.execute_command(&cmd).await?;
        get_mode::parse(&response)
    }

    pub async fn get_freq(&mut self, vfo: VFO) -> Result<get_freq::Response, RigCtlError> {
        let cmd = format!("get_freq {}", vfo);
        let response = self.execute_command(&cmd).await?;
        get_freq::parse(&response)
    }

    pub async fn get_vfo(&mut self) -> Result<get_vfo::Response, RigCtlError> {
        let cmd = "get_vfo".to_string();
        let response = self.execute_command(&cmd).await?;
        get_vfo::parse(&response)
    }

    pub async fn get_split_vfo(&mut self) -> Result<get_split_vfo::Response, RigCtlError> {
        let cmd = "get_split_vfo 0".to_string();
        let response = self.execute_command(&cmd).await?;
        get_split_vfo::parse(&response)
    }

    pub async fn get_split_mode(&mut self, vfo: VFO) -> Result<get_split_mode::Response, RigCtlError> {
        let cmd = format!("get_split_mode {}", vfo);
        let response = self.execute_command(&cmd).await?;
        get_split_mode::parse(&response)
    }

    pub async fn get_split_freq(&mut self, vfo: VFO) -> Result<get_split_freq::Response, RigCtlError> {
        let cmd = format!("get_split_freq {}", vfo);
        let response = self.execute_command(&cmd).await?;
        get_split_freq::parse(&response)
    }

    fn compose_command(&self, command: &str) -> String {
        format!("|\\{}", command)
    }

    async fn execute_command(&mut self, command: &str) -> Result<String, RigCtlError> {
        let cmd = self.compose_command(&command);
        self.write_line(&cmd).await?;
        self.read_line().await
    }

    async fn read_line(&mut self) -> Result<String, RigCtlError> {
        log::debug!("Reading line");

        let mut buf = [0u8; 4096];
        let bytes_read = time::timeout(
            self.timeout,
            self.stream.
                as_mut().unwrap()
                .read(&mut buf))
            .await
            .map_err(|_| RigCtlError::CommunicationTimeout)??;

        let line = String::from_utf8(buf[0..bytes_read].to_owned())?
            .trim_end().to_string();

        log::trace!(" <<< [{}] ({} bytes)", line, line.len());

        Ok(line)
    }

    async fn write_line(&mut self, data: &str) -> Result<(), RigCtlError> {
        log::debug!("Writing line");
        log::trace!(" >>> [{}] ({} bytes)", data, data.len());
        time::timeout(
            self.timeout,
            self.stream
                .as_mut().unwrap()
                .write_all(format!("{}\n", data).as_bytes()))
            .await
            .map_err(|_| RigCtlError::CommunicationTimeout)??;
        Ok(())
    }
}
