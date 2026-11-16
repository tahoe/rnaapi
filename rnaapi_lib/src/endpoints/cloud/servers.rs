// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne};

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
#[getall(path = "cloud/servers")]
#[getone(path = "cloud/server?mbpkgid={}", mbpkgid: u32)]
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
#[getall(path = "cloud/server/{}/jobs", mbpkgid: u32)]
#[getone(path = "cloud/server/{}/jobs/{}", mbpkgid: u32, jobid: u32)]
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
#[getone(path = "cloud/status/{}", mbpkgid: u32)]
pub struct SrvStatus {
    pub status: String,
}

//
// IPv4IP struct
// URL: https://vapi2.netactuate.com/api/cloud/ipv4?mbpkgid=<mbpkgid>&key=<api_key>
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/ipv4?mbpkgid={}", mbpkgid: u32)]
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
#[getall(path = "cloud/ipv6?mbpkgid={}", mbpkgid: u32)]
pub struct IPv6 {
    pub id: u32,
    pub primary: u32,
    pub reverse: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub broadcast: String,
}

//
// Monthly bandwidth for one server.
// URL: https://vapi2.netactuate.com/api/cloud/servermonthlybw/{mbpkgid}
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/servermonthlybw/{}", mbpkgid: u32)]
pub struct MonthlyBw {
    pub date: String,
    pub rx: u32,
    pub tx: u32,
}

/// `cloud/servers/current`. `Server` already owns `get` and `list`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/servers/current")]
pub struct CurrentServer(pub Server);
