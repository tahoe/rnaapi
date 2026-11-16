// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "ddos/attacks")]
pub struct DdosAttack {
    pub id: i64,
    pub date_start: String,
    pub date_end: String,
    pub status: i64,
    pub ip: String,
    pub prefix: String,
    pub direction: String,
    pub pps: i64,
    pub rule_id: i64,
    pub rule_type: String,
    pub ban_duration: i64,
    pub rule_name: String,
}

/// `ddos/attacks/active`. `DdosAttack` already lists every attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "ddos/attacks/active")]
pub struct DdosActiveAttacks(pub Vec<DdosAttack>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "ddos/dashboard")]
pub struct DdosDashboard {
    pub total_attacks: i64,
    pub active_rules: i64,
    pub longest_attack_seconds: i64,
    #[serde(default)]
    pub top_attacks: Vec<serde_json::Value>,
    pub period: i64,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetOne,
    EndpointGetAll,
)]
#[serde(rename_all = "snake_case")]
#[getone(path = "ddos/rule/{}", id: u32)]
#[getall(path = "ddos/rules")]
pub struct DdosRule {
    pub id: i64,
    pub rule_name: String,
    pub description: String,
    #[serde(default)]
    pub prefixes: Vec<DdosRulePrefix>,
    #[serde(default)]
    pub rules: Vec<DdosRuleAction>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DdosRulePrefix {
    pub id: i64,
    pub prefix: String,
    pub prefix_type: String,
    pub description: String,
    pub allowed_pps: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DdosRuleAction {
    pub name: String,
    pub action_type: String,
    pub run_order: i64,
    pub action_name: String,
    pub action_description: String,
}
