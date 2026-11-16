// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne, EndpointWrite};

/// SSH public keys stored on the account.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetAll,
    EndpointGetOne,
    EndpointWrite,
)]
#[serde(rename_all = "snake_case")]
#[getone(path = "account/ssh_key/{}", id: u32)]
#[getall(path = "account/ssh_keys")]
#[write(post, form, name = "create", path = "account/ssh_key", body: SshKeyBody)]
#[write(patch, name = "update", path = "account/ssh_key/{}", id: u32, body: SshKeyBody)]
#[write(delete, name = "delete", path = "account/ssh_key/{}", id: u32, returns = ())]
pub struct SSHKeys {
    pub id: u32,
    pub ssh_key: String,
    pub fingerprint: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SshKeyBody {
    pub name: String,
    pub ssh_key: String,
}
