// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::EndpointGetOne;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PackageDetails {
    pub dc_name: String,
    pub iata_code: String,
    pub bw_commit: String,
    pub overage_type: String,
    pub overage_rate: String,
    pub agg_bw_mbpkgid: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "colo/package/{}", mbpkgid: u32)]
pub struct ColocationPackage {
    pub mbpkgid: i64,
    pub package_status: String,
    pub fqdn: String,
    pub billingcycle: String,
    pub nextduedate: String,
    pub amount: String,
    pub details: PackageDetails,
    pub status: String,
}

/// `colo/packages` is a map keyed by package id, not an array.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "colo/packages")]
pub struct ColocationPackages(
    pub std::collections::HashMap<String, ColocationPackage>,
);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "transit/package/{}", mbpkgid: u32)]
pub struct TransitPackage {
    pub mbpkgid: i64,
    pub package_status: String,
    pub fqdn: String,
    pub billingcycle: String,
    pub nextduedate: String,
    pub amount: String,
    pub details: PackageDetails,
    pub status: String,
}

/// `transit/packages` is a map keyed by package id, not an array.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "transit/packages")]
pub struct TransitPackages(
    pub std::collections::HashMap<String, TransitPackage>,
);
