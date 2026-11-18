// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne};

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
#[getall(path = "cloud/locations")]
#[getone(path = "cloud/locations/{}", id: u32)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub iata_code: String,
    pub continent: String,
    pub flag: String,
    pub latitude: String,
    pub longitude: String,
    pub disabled: u32,
    /// Present on `cloud/locations/{id}`. The list response omits it.
    #[serde(default)]
    pub status: Vec<LocationStatus>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LocationStatus {
    pub status_group_id: i64,
    pub status_name: String,
    pub container_status: String,
    pub last_updated: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_list_row_omits_status_and_a_single_location_includes_it() {
        let row: Location = serde_json::from_value(json!({
            "id": 3,
            "name": "San Jose, CA",
            "iata_code": "SJC",
            "continent": "NA",
            "flag": "us",
            "latitude": "0",
            "longitude": "0",
            "disabled": 0
        }))
        .unwrap();
        assert!(row.status.is_empty());

        let one: Location = serde_json::from_value(json!({
            "id": 3,
            "name": "San Jose, CA",
            "iata_code": "SJC",
            "continent": "NA",
            "flag": "us",
            "latitude": "0",
            "longitude": "0",
            "disabled": 0,
            "status": [{
                "status_group_id": 1,
                "status_name": "api",
                "container_status": "operational",
                "last_updated": "2026-01-01"
            }]
        }))
        .unwrap();
        assert_eq!(one.status[0].status_name, "api");
    }
}
