// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{EndpointGetAll, EndpointGetOne};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "dedicated/capacity")]
pub struct DedicatedCapacity {
    pub device_id: i64,
    pub location_id: i64,
    pub looking_glass: String,
    pub mbpkgid: i64,
    pub name: String,
    pub nps_enabled: i64,
    pub pub_description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "dedicated/locations")]
pub struct DedicatedLocation {
    pub short_name: String,
    pub pub_description: String,
    pub location_id: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(transparent)]
#[getall(path = "dedicated/plans/{}", location_id: u32)]
pub struct DedicatedPlan(pub serde_json::Map<String, Value>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "dedicated/disklayouts/{}", os_id: u32)]
pub struct DedicatedDiskLayout {
    pub id: i64,
    pub name: String,
    pub profile: String,
    pub min_disks: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "dedicated/os")]
pub struct DedicatedOsProfile {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub group_name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub disklayouts: Value,
    #[serde(default)]
    pub scripts: Value,
    pub default_disklayout: Option<i64>,
    #[serde(default)]
    pub default_scripts: Vec<i64>,
    pub allow_ssh_keys: i64,
    pub set_root_password: i64,
    pub rescue_image: i64,
    pub r#public: i64,
    pub enabled: i64,
    pub created: String,
    pub last_updated: String,
    pub profile_id: i64,
    pub arch: String,
    pub flavor: String,
    pub location_id: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "dedicated/os/rescue-system-list")]
pub struct DedicatedRescueOs(pub Vec<DedicatedOsProfile>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "dedicated/os/device/{}", device_id: u32)]
pub struct DedicatedDeviceOs(pub Vec<DedicatedOsProfile>);

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
#[getone(path = "dedicated/servers/{}", id: u32)]
#[getall(path = "dedicated/servers")]
pub struct MetalServer {
    pub id: i64,
    pub datacenter_id: i64,
    pub canceling: i64,
    pub mbpkgid: i64,
    pub price: String,
    pub hostname: String,
    pub eth0_mac: String,
    pub eth1_mac: Option<String>,
    pub ipmi_mac: String,
    pub primary_ip: String,
    pub primary_ipv6: Option<String>,
    pub nps_installed: i64,
    pub nps_os: String,
    pub mb_model: Option<String>,
    pub cpu0_model: String,
    pub cpu1_model: Option<String>,
    pub total_ram: i64,
    pub ipmi_pubip: String,
    pub ipmi_cxuser: Option<String>,
    pub ipmi_cxpass: Option<String>,
    pub ipmi_status: i64,
    pub locked: i64,
    pub locked_msg: Option<String>,
    pub ipmi_status_time: String,
    pub ob_id: Value,
    pub info: String,
    pub title: String,
    pub ipmi_refresh_enabled: i64,
    pub location: String,
    pub ip_subnet_id: i64,
    pub ip_subnet_name: String,
    pub package_status: String,
    #[serde(default)]
    pub building: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "dedicated/server/build_status/{}", build_id: u32)]
pub struct MetalBuildStatus {
    pub mbpkgid: i64,
    pub response: String,
    pub status: String,
    pub percent: i64,
    pub image_name: String,
}
