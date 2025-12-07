// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use std::fmt::format;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::NaClient;
use crate::errors::NaApiError;

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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

// Get zone/domain info
impl NaClient {
    /// Get a single DNS Zone (domain)
    /// Requires a Zone ID
    pub async fn get_zone(&self, zoneid: u32) -> Result<Zone, NaApiError> {
        let data = self
            .get_data(&format!("dns/zone/{zoneid}").to_owned())
            .await?;
        let zone: Zone = serde_json::from_value(data).unwrap();
        Ok(zone)
    }

    /// Get all my DNS Zones
    pub async fn get_zones(&self) -> Result<Vec<Zone>, NaApiError> {
        let data = self.get_data("dns/zones?type=NATIVE").await?;
        let zones: Vec<Zone> = serde_json::from_value(data).unwrap();
        Ok(zones)
    }
}

//
// Define an Record
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

// Get zone/domain info
impl NaClient {
    /// Get a single DNS record in a zone
    /// Requires a Record ID
    pub async fn get_record(
        &self, recordid: u32,
    ) -> Result<Record, NaApiError> {
        let data = self
            .get_data(&format!("dns/record/{recordid}").to_owned())
            .await?;
        let record: Record = serde_json::from_value(data).unwrap();
        Ok(record)
    }

    /// Get all my DNS records for a zone
    /// Requires a Zone ID
    pub async fn get_records(
        &self, zoneid: u32,
    ) -> Result<Vec<Record>, NaApiError> {
        let data = self.get_data("dns/records/{zoneid}").await?;
        let records: Vec<Record> = serde_json::from_value(data).unwrap();
        Ok(records)
    }
}
