// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne};

use super::super::flex;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetOne,
    EndpointGetAll,
)]
#[getone(path = "account/user-access-control-subnet/{}", id: String)]
#[getall(path = "account/user-access-control-subnet-list")]
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
