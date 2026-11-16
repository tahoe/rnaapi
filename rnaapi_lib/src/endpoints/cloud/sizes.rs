// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::EndpointGetAll;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/sizes/{}", id: u32)]
pub struct Sizes {
    pub plan_id: u32,
    pub plan: String,
    pub ram: String,
    pub disk: String,
    pub transfer: String,
    pub price: String,
    pub cpu: u32,
    pub port: String,
    pub available: f32,
}
