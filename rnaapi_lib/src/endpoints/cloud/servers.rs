// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGetAll, EndpointGetArgs, EndpointGetOne, NaClient};
use async_trait::async_trait;

//
// Server struct
//
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetOne,
    EndpointGetAll,
)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/servers", args = 0)]
#[getone(path = "cloud/server?mbpkgid={}", args = 1)]
pub struct Server {
    pub city: String,
    pub fqdn: String,
    #[serde(rename = "domU_package")]
    pub domu_package: u32,
    pub mbpkgid: u32,
    pub os_id: u32,
    pub location_id: u32,
    pub ip: String,
    pub ipv6: String,
    pub plan_id: u32,
    pub pkg_id: u32,
    pub state: String,
    pub status: String,
    pub uptime: String,
    pub installed: u32,
    pub nic1_mac: String,
    pub nic2_mac: String,
}

// Job struct
// URL: https://vapi2.netactuate.com/api/cloud/server/{mbpkgid}/jobs/{jobid}
// URL: https://vapi2.netactuate.com/api/cloud/server/{mbpkgid}/jobs
//
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
#[getall(path = "cloud/server/{}/jobs", args = 1)]
#[getone(path = "cloud/server/{}/jobs/{}", args = 2)]
pub struct SrvJob {
    pub id: u32,
    // #[serde(with = "custom_datetime_format_seconds")]
    // pub ts_insert: NaiveDateTime,
    pub ts_insert: String,
    pub command: String,
    pub status: u32,
}

//
// Status struct
// URL: https://vapi2.netactuate.com/api/cloud/status/{mbpkgid}
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "cloud/status/{}", args = 1)]
pub struct SrvStatus {
    pub status: String,
}

//
// IPv4IP struct
// URL: https://vapi2.netactuate.com/api/cloud/ipv4?mbpkgid=<mbpkgid>&key=<api_key>
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/ipv4?mbpkgid={}", args = 1)]
pub struct IPv4 {
    pub id: u32,
    pub primary: u32,
    pub reverse: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub broadcast: String,
}

//
// IPv6IP struct
// URL: https://vapi2.netactuate.com/api/cloud/ipv6?mbpkgid=<mbpkgid>&key=<api_key>
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/ipv6?mbpkgid={}", args = 1)]
pub struct IPv6 {
    pub id: u32,
    pub primary: u32,
    pub reverse: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub broadcast: String,
}
