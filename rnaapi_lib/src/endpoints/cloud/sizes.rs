// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGetAll, EndpointGetArgs, NaClient};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/sizes/{}", args = 1)]
pub struct Image {
    pub plan_id: u32,
    pub plan: String,
    pub ram: String,
    pub disk: String,
    pub transfer: String,
    pub price: String,
    pub cpu: u32,
    pub port: String,
    pub available: u32,
}
