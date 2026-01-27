// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::NaApiError;
use crate::{EndpointGetAll, EndpointGetArgs, EndpointGetOne, NaClient};
use async_trait::async_trait;

///
/// Account Details #[derive(Debug)]
///
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetAll,
    EndpointGetOne,
)]
#[serde(rename_all = "snake_case")]
#[getone(path = "account/ssh_key/{}", args = 1)]
#[getall(path = "account/ssh_keys", args = 0)]
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
