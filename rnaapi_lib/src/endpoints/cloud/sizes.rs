// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne};

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

/// `cloud/sizes` with no location. `Sizes` already lists one location id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/sizes")]
pub struct Plan {
    pub plan_id: i64,
    pub plan: String,
    pub ram: String,
    pub disk: String,
    pub transfer: String,
    pub price: String,
    pub available: f64,
}

/// Deploy sizes for a location name. `Sizes::list` takes a numeric location id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/sizes/{}", location: String)]
pub struct DeploySizes(pub Vec<Sizes>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/sizes/plan-id/{}", plan: String)]
pub struct PlanId(pub serde_json::Value);
