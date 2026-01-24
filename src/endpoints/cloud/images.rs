// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGet, EndpointGetArgs, NaClient};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Image {
    pub id: u32,
    pub os: Option<String>,
    pub description: Option<String>,
    pub size: Option<String>,
    pub subtype: Option<String>,
    pub created: Option<String>,
    pub category: Option<String>,
    pub updated: Option<String>,
    pub iso: Option<String>,
    pub bits: Option<String>,
    pub tech: Option<String>,
    pub icon: Option<String>,
    pub private: Option<u32>,
}

//
// Images
//
#[async_trait]
impl EndpointGet for Image {
    type Endpoint = Image;
    /// Get a list of available OS images
    async fn get_all(
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<Vec<Image>, NaApiError> {
        match args {
            EndpointGetArgs::NoArgs => {
                let data = na_client.get_data("cloud/images").await?;
                let image_list: Vec<Image> =
                    serde_json::from_value(data).unwrap();
                Ok(image_list)
            }
            _ => {
                Err(NaApiError::UnknownError("No arguments allowed".to_owned()))
            }
        }
    }
}
