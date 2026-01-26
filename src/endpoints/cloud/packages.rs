// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGetAll, EndpointGetArgs, NaClient};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[getall(path = "cloud/packages", args = 0)]
pub struct Package {
    pub mbpkgid: u32,
    pub package_status: String,
    pub fqdn: String,
    pub name: String,
    pub gid: u32,
    #[serde(rename = "domU_package")]
    pub domu_package: u32,
    pub rescue: u32,
    pub locked: u32,
    pub package: String,
    pub ipv6: String,
    pub city: String,
    pub ip: String,
    pub installed: u32,
    pub state: String,
    pub uptime: String,
    pub os: String,
    pub is_building: u32,
}
