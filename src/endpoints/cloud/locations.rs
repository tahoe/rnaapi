// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGetAll, EndpointGetArgs, NaClient};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/locations", args = 0)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub iata_code: String,
    pub continent: String,
    pub flag: String,
    pub latitude: String,
    pub longitude: String,
    pub disabled: u32,
}
