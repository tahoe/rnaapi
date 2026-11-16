// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne};

use super::flex;

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
#[getone(path = "cloud/networking/vlans/{}", id: u32)]
#[getall(path = "cloud/networking/vlans")]
pub struct Vlan {
    pub id: i64,
    pub mbid: i64,
    pub private: i64,
    pub allow_sriov: i64,
    pub display_name: String,
    pub description: String,
    pub last_updated: String,
    pub created: String,
    #[serde(default)]
    pub provisioned_locations: Vec<ProvisionedLocation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProvisionedLocation {
    pub provisioned: bool,
    pub name: String,
    pub location_id: i64,
    pub flag: String,
    pub iata_code: String,
}

/// VLANs at one location. `Vlan` already lists every VLAN.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/networking/locations/{}/vlans", location_id: u32)]
pub struct LocationVlans(pub Vec<Vlan>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/networking/nics/{}", mbpkgid: u32)]
pub struct ServerNic {
    #[serde(default, deserialize_with = "flex::int")]
    pub id: i64,
    #[serde(default, deserialize_with = "flex::int")]
    pub nic_id: i64,
    #[serde(default, deserialize_with = "flex::int")]
    pub mbpkgid: i64,
    #[serde(default, deserialize_with = "flex::int")]
    pub customer_vlan_id: i64,
    #[serde(default, deserialize_with = "flex::int")]
    pub attach_order: i64,
}
