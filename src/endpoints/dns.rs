// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGetAll, EndpointGetArgs, EndpointGetOne, NaClient};
use async_trait::async_trait;

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
)]
#[serde(rename_all = "snake_case")]
#[getone(path = "dns/zone/{}", args = 1)]
#[getall(path = "dns/zones?type=NATIVE", args = 0)]
pub struct Zone {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub zone_type: String,
    pub master: Option<u32>,
    pub ttl: Option<TtlType>,
    pub soa: Option<SOA>,
    pub records: Option<Vec<Record>>,
    pub ns: Option<Vec<String>>,
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
)]
#[serde(rename_all = "snake_case")]
#[getone(path = "dns/record/{}", args = 1)]
#[getall(path = "dns/records/{}", args = 1)]
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
