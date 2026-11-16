// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "services")]
pub struct Service {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub description: String,
}

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
#[getone(path = "services/colocation/{}", id: u32)]
#[getall(path = "services/colocation")]
pub struct ColocationService {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub service_id: i64,
    #[serde(default)]
    pub datacenter_id: i64,
    #[serde(default)]
    pub rack_identifier: String,
    #[serde(default)]
    pub power_details: String,
    #[serde(default)]
    pub description: String,
}

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
#[getone(path = "services/iptransit/{}", id: u32)]
#[getall(path = "services/iptransit")]
pub struct IpTransitService {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub service_id: i64,
    #[serde(default)]
    pub datacenter_id: i64,
    #[serde(default)]
    pub bgp_group_id: i64,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "services/iptransit/ips")]
pub struct IpTransitAddress {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub service_iptransit_id: i64,
    #[serde(default)]
    pub ip: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "services/iptransit/ports")]
pub struct IpTransitPort {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub service_iptransit_id: i64,
    #[serde(default)]
    pub name: String,
}

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
#[getone(path = "services/transport/{}", id: u32)]
#[getall(path = "services/transport")]
pub struct TransportService {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub service_id: i64,
    #[serde(default)]
    pub datacenter_id: i64,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "services/transport/ports")]
pub struct TransportPort {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub service_transport_id: i64,
    #[serde(default)]
    pub name: String,
}
