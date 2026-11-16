// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::EndpointGetAll;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "tags/{}/resources", tag_id: u32)]
pub struct TagResource {
    pub id: i64,
    pub resource_tag_id: i64,
    pub resource_name: String,
    pub identifier: i64,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "tags")]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub is_default: i64,
    pub is_favorite: i64,
    pub is_locked: i64,
    pub show_dashboard: i64,
    pub created_at: String,
    pub mb_id: i64,
    pub resources_count: i64,
    #[serde(default)]
    pub resources: Vec<TagResource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(transparent)]
#[getall(path = "tags/resource/{}/id/{}", resource_name: String, resource_id: u32)]
pub struct ResourceTag(pub Tag);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "tags/{}/logs", tag_id: u32)]
pub struct TagLog {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub tag_id: i64,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub created_at: String,
}
