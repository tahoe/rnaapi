// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne, EndpointWrite};

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
#[getone(path = "cloud/images/{}", id: u32)]
#[getall(path = "cloud/images")]
#[write(post, form, name = "create", path = "cloud/images/create", body: CreateImage, returns = ImageJob)]
#[write(patch, name = "edit", path = "cloud/images/{}/edit", id: u32, body: EditImage, returns = ())]
#[write(delete, name = "delete", path = "cloud/images/{}/delete", id: u32, returns = ImageJob)]
#[write(post, name = "replace", path = "cloud/images/{}/replace_image", id: u32, body: ReplaceImage, returns = serde_json::Value)]
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

/// `cloud/images/my`. `Image` already owns `list` for `cloud/images`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/images/my")]
pub struct MyImages(pub Vec<Image>);

/// `cloud/images/base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/images/base")]
pub struct BaseImages(pub Vec<Image>);

/// `cloud/images/private`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/images/private")]
pub struct PrivateImages(pub Vec<Image>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "cloud/images/queue_status/{}", queue_id: u32)]
pub struct ImageQueueStatus {
    pub status: String,
    pub percent: i64,
    pub response: String,
    pub image_id: i64,
    pub image_name: String,
    pub image_help: String,
    pub location: String,
    pub mb_pkgid: i64,
    pub fqdn: String,
    pub os: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateImage {
    pub mbpkgid: i64,
    pub image_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_ssh_userdirs: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditImage {
    pub os: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceImage {
    pub replace_id: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageJob {
    pub queue_id: i64,
}
