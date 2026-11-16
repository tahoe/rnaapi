// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{EndpointGetAll, EndpointGetOne};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlatformStatusService {
    pub component_id: String,
    #[serde(default)]
    pub locations: HashMap<String, PlatformStatusLocation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlatformStatusLocation {
    pub container_id: String,
    pub status: String,
    pub last_updated: String,
}

/// `platform/status` is a map of service name to status, not an array.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "platform/status")]
pub struct PlatformStatus(pub HashMap<String, PlatformStatusService>);

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
#[getone(path = "platform/change-log/{}", id: u32)]
#[getall(path = "platform/change-log")]
pub struct PlatformChangeLogEntry {
    #[serde(rename = "id")]
    pub change_log_id: Value,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub short_description: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[getall(path = "platform/datacenters/{}", location: String)]
pub struct Datacenter {
    pub id: i64,
    pub name: String,
    pub iata: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[getone(path = "platform/datacenters-by-iata/{}", iata: String)]
pub struct DatacenterByIata {
    pub id: i64,
    pub name: String,
    pub iata: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "platform/looking-glass/init")]
pub struct LookingGlassInit(pub Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "graphs/graph?port={}&time={}", port: u32, time: String)]
pub struct Graph(pub Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "location")]
pub struct LocationByIp {
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub location: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "platform/maintenance-info/{}", id: u32)]
pub struct MaintenanceInfo(pub Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlatformEvent {
    pub event_id: String,
    pub r#type: String,
    pub name: String,
    pub status: String,
    pub start_time: String,
    pub end_time: String,
    #[serde(default)]
    pub components: Vec<String>,
    #[serde(default)]
    pub containers: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "platform/incidents/{}", location: String)]
pub struct PlatformEvents {
    #[serde(default)]
    pub active: Vec<PlatformEvent>,
    #[serde(default)]
    pub upcoming: Vec<PlatformEvent>,
    #[serde(default)]
    pub historic: Vec<PlatformEvent>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "platform/incidents/history/{}", location: String)]
pub struct PlatformIncidentHistory(pub PlatformEvents);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "platform/maintenance/{}", location: String)]
pub struct PlatformMaintenance(pub PlatformEvents);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "platform/maintenance/history/{}", location: String)]
pub struct PlatformMaintenanceHistory(pub PlatformEvents);
