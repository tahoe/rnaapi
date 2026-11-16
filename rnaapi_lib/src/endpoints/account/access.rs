// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne, EndpointWrite};

use super::super::flex;

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
#[getone(path = "account/user-access-control-subnet/{}", id: String)]
#[getall(path = "account/user-access-control-subnet-list")]
#[write(post, name = "create", path = "account/user-access-control-subnet", body: AccessControlSubnetBody)]
#[write(patch, name = "update", path = "account/user-access-control-subnet/{}", id: String, body: AccessControlSubnetUpdate)]
#[write(delete, name = "delete", path = "account/user-access-control-subnet/{}", id: String, returns = ())]
pub struct AccessControlSubnet {
    #[serde(deserialize_with = "flex::id")]
    pub id: String,
    pub label: String,
    pub subnet: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[getall(path = "account/agreements")]
pub struct AccountAgreement {
    pub id: i64,
    pub name: String,
    pub title: String,
    pub description: String,
    pub version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlSubnetBody {
    pub label: String,
    pub subnet: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AccessControlSubnetUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
}
