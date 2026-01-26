// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGetAll, EndpointGetArgs, NaClient};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/images", args = 0)]
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
