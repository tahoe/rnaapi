// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne, EndpointWrite};

//
// Zone ttl key type since it changes between
// a single zone get and a get all zones
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TtlType {
    StringKey(String),
    IntegerKey(u32),
}

//
// Just Zone struct (from ID)
//
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
#[getone(path = "dns/zone/{}", id: u32)]
#[getall(path = "dns/zones?type=NATIVE")]
#[write(post, form, name = "create", path = "dns/zone", body: ZoneBody)]
#[write(delete, name = "delete", path = "dns/zone/{}", id: u32, returns = ())]
pub struct Zone {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub zone_type: String,
    pub master: Option<u32>,
    pub ttl: Option<TtlType>,
    pub soa: Option<SOA>,
    pub records: Option<Vec<Record>>,
    pub ns: Option<Vec<Record>>,
}

//
// Define an SOA (part of Zone, no separate request for this)
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SOA {
    pub primary: String,
    pub hostmaster: String,
    pub serial: String,
    pub refresh: String,
    pub retry: String,
    pub expire: String,
    pub default_ttl: String,
}

//
// Define an Record
//
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetAll,
    EndpointGetOne,
    EndpointWrite,
)]
#[serde(rename_all = "snake_case")]
#[getone(path = "dns/record/{}", id: u32)]
#[getall(path = "dns/records/{}", id: u32)]
#[write(post, form, name = "create", path = "dns/record", body: RecordBody)]
#[write(put, form, name = "update", path = "dns/record", body: RecordUpdate)]
#[write(delete, name = "delete", path = "dns/record/{}", id: u32, returns = ())]
pub struct Record {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub ttl: Option<u32>,
    pub prio: Option<u32>,
    pub content: String,
    pub domain_id: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneBody {
    pub name: String,
    #[serde(rename = "type")]
    pub zone_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordBody {
    pub domain_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub record_type: String,
    pub record_content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prio: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordUpdate {
    pub id: i64,
    pub domain_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub record_type: String,
    pub record_content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prio: Option<i64>,
}
