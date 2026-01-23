// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::NaClient;
use crate::errors::NaApiError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub iata_code: String,
    pub continent: String,
    pub flag: String,
    pub latitude: String,
    pub longitude: String,
    pub disabled: u32,
}

//
// Locations
//
impl Location {
    /// Get a list of available locations
    pub async fn get_all(
        na_client: &NaClient,
    ) -> Result<Vec<Location>, NaApiError> {
        let data = na_client.get_data("cloud/locations").await?;
        let location_list: Vec<Location> =
            serde_json::from_value(data).unwrap();
        Ok(location_list)
    }
}
/*

// Location is an API response message of available deployment locations
type Location struct {
    ID        int    `json:"id"`
    Name      string `json:"name"`
    IATACode  string `json:"iata_code"`
    Continent string `json:"continent"`
    Flag      string `json:"flat"`
    Disabled  int    `json:"disabled"`
}

// GetLocations public method on Client to get a list of locations
func (c *Client) GetLocations() ([]Location, error) {
    r := make([]Location, 0)
    if err := c.get("cloud/locations", &r); err != nil {
        return nil, err
    }
    return r, nil
}
*/
