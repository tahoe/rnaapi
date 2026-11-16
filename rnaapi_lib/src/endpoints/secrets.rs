// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne, EndpointWrite};

use super::flex;

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
#[getone(path = "secrets/lists/{}", id: u32)]
#[getall(path = "secrets/lists")]
#[write(post, form, name = "create", path = "secrets/lists", body: SecretListName)]
#[write(post, form, name = "update", path = "secrets/lists/{}", id: u32, body: SecretListName)]
#[write(delete, name = "delete", path = "secrets/lists/{}", id: u32, returns = ())]
pub struct SecretList {
    pub id: i64,
    pub name: String,
}

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
#[getone(path = "secrets/lists/{}/values/{}", list_id: u32, value_id: u32)]
#[getall(path = "secrets/lists/{}/values", list_id: u32)]
#[write(post, form, name = "create", path = "secrets/lists/{}/values", list_id: u32, body: SecretValueBody)]
#[write(post, form, name = "update", path = "secrets/lists/{}/values/{}", list_id: u32, value_id: u32, body: SecretValueBody)]
#[write(delete, name = "delete", path = "secrets/lists/{}/values/{}", list_id: u32, value_id: u32, returns = ())]
pub struct SecretListValue {
    pub id: i64,
    #[serde(deserialize_with = "flex::int")]
    pub secret_list_id: i64,
    pub secret_key: String,
    pub secret_value: String,
}

/// `secrets/all-values`. `SecretListValue` already lists one secret list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "secrets/all-values")]
pub struct SecretValues(pub Vec<SecretListValue>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretListName {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueBody {
    pub secret_key: String,
    pub secret_value: String,
}
