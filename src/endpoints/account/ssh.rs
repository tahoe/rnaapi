// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
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
impl SSHKeys {
    /// Get an account's SSH keys
    pub async fn get_all(
        na_client: &NaClient,
    ) -> Result<Vec<SSHKeys>, NaApiError> {
        let data = na_client.get_data("account/ssh_keys").await?;
        let ssh_keys: Vec<SSHKeys> = serde_json::from_value(data).unwrap();
        Ok(ssh_keys)
    }

    pub async fn get_one(
        na_client: &NaClient, keyid: u32,
    ) -> Result<SSHKeys, NaApiError> {
        let data = na_client
            .get_data(&format!("account/ssh_key/{keyid}").to_owned())
            .await?;
        let ssh_key: SSHKeys = serde_json::from_value(data).unwrap();
        Ok(ssh_key)
    }
}
