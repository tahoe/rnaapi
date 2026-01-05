// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::NaClient;
use crate::errors::NaApiError;

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
impl NaClient {
    /// Get a list of available OS images
    pub async fn get_images(&self) -> Result<Vec<Image>, NaApiError> {
        let data = self.get_data("cloud/images").await?;
        let image_list: Vec<Image> = serde_json::from_value(data).unwrap();
        Ok(image_list)
    }
}
/*
// OS is a struct for storing the attributes of an OS
type OS struct {
    ID      int    `json:"id"`
    Os      string `json:"os"`
    Type    string `json:"type"`
    Subtype string `json:"subtype"`
    Size    string `json:"size"`
    Bits    string `json:"bits"`
    Tech    string `json:"tech"`
}

// GetOSs returns a list of OS objects from the api
func (c *Client) GetOSs() ([]OS, error) {
    var osList []OS
    if err := c.get("cloud/images", &osList); err != nil {
        return nil, err
    }
    return osList, nil
}
*/
