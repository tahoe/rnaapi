// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGet, EndpointGetArgs, NaClient};
use async_trait::async_trait;

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
#[async_trait]
impl EndpointGet for Location {
    type Endpoint = Location;
    /// Get a list of available locations
    async fn get_all(
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<Vec<Location>, NaApiError> {
        match args {
            EndpointGetArgs::NoArgs => {
                let data = na_client.get_data("cloud/locations").await?;
                let location_list: Vec<Location> =
                    serde_json::from_value(data).unwrap();
                Ok(location_list)
            }
            _ => {
                Err(NaApiError::UnknownError("No arguments allowed".to_owned()))
            }
        }
    }
}
