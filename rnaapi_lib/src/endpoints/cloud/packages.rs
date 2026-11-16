// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne, EndpointWrite};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetOne,
    EndpointGetAll,
    EndpointWrite,
)]
#[serde(rename_all = "snake_case")]
#[getone(path = "cloud/package/{}", id: u32)]
#[getall(path = "cloud/packages")]
#[write(post, name = "cancel", path = "cloud/package/cancel/", body: CancelPackage, returns = serde_json::Value)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancelPackage {
    pub mbpkgid: i64,
    #[serde(rename = "domU_package", skip_serializing_if = "Option::is_none")]
    pub domu_package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    pub cancel_type: String,
    pub agree: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
