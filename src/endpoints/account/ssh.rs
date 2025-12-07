// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
// use crate::custom_datetime_format_microseconds;
use std::fmt::format;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::NaApiError;
use crate::NaClient;

///
/// Account Details #[derive(Debug)]
///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SSHKeys {
    pub id: u32,
    pub ssh_key: String,
    pub fingerprint: String,
    pub name: String,
    // #[serde(with = "custom_datetime_format_microseconds")]
    // pub created_at: NaiveDateTime,
    // #[serde(with = "custom_datetime_format_microseconds")]
    // pub updated_at: NaiveDateTime,
    // pub mb_id: u32,
    // pub created_at: String,
    // pub updated_at: String,
}

// Get Details
impl NaClient {
    /// Get an account's SSH keys
    pub async fn get_ssh_keys(&self) -> Result<Vec<SSHKeys>, NaApiError> {
        let data = self.get_data("account/ssh_keys").await?;
        let ssh_keys: Vec<SSHKeys> = serde_json::from_value(data).unwrap();
        Ok(ssh_keys)
    }
}
