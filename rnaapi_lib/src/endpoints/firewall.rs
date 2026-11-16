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
#[getone(path = "firewall/sets/{}", id: u32)]
#[getall(path = "firewall/sets")]
#[write(post, form, name = "create", path = "firewall/sets", body: FirewallSetBody)]
#[write(put, form, name = "update", path = "firewall/sets/{}", id: u32, body: FirewallSetBody)]
#[write(delete, name = "delete", path = "firewall/sets/{}", id: u32, returns = ())]
#[write(put, form, name = "enable", path = "firewall/sets/{}/enable", id: u32, returns = ())]
#[write(put, form, name = "disable", path = "firewall/sets/{}/disable", id: u32, returns = ())]
#[write(post, form, name = "create_draft", path = "firewall/sets/{}/create-draft", id: u32)]
#[write(post, form, name = "publish_draft", path = "firewall/sets/publish-draft/{}", draft_id: u32)]
#[write(delete, name = "delete_draft", path = "firewall/sets/delete-draft/{}", draft_id: u32, returns = ())]
#[write(post, form, name = "sync_rules", path = "firewall/sets/{}/vm/sync-all", set_id: u32, returns = ())]
pub struct FirewallSet {
    pub id: i64,
    pub name: String,
    pub description: String,
    #[serde(deserialize_with = "flex::boolean")]
    pub enabled: bool,
    #[serde(deserialize_with = "flex::boolean")]
    pub is_draft: bool,
    pub draft_firewall_set_id: Option<i64>,
    pub created: String,
    pub last_updated: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FirewallMatchOptions {
    #[serde(default)]
    pub icmp_type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FirewallMatchCriteria {
    #[serde(default)]
    pub protocol: String,
    #[serde(default)]
    pub source_net: Vec<String>,
    #[serde(default)]
    pub destination_net: Vec<String>,
    pub source_port_start: Option<i64>,
    pub source_port_end: Option<i64>,
    pub destination_port_start: Option<i64>,
    pub destination_port_end: Option<i64>,
    pub ip_version_number: Option<i64>,
    pub options: Option<FirewallMatchOptions>,
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
#[getone(path = "firewall/sets/{}/rules/{}", set_id: u32, rule_id: u32)]
#[getall(path = "firewall/sets/{}/rules", set_id: u32)]
#[write(post, name = "create", path = "firewall/sets/{}/rules", set_id: u32, body: FirewallRuleBody)]
#[write(put, name = "update", path = "firewall/{}/{}", set_id: u32, rule_id: u32, body: FirewallRuleBody)]
#[write(delete, name = "delete", path = "firewall/{}/rules/{}", set_id: u32, rule_id: u32, returns = ())]
#[write(post, name = "reorder", path = "firewall/sets/{}/rules/re-order", set_id: u32, body: ReorderFirewallRules, returns = ())]
pub struct FirewallRule {
    pub id: i64,
    #[serde(deserialize_with = "flex::int")]
    pub firewall_set_id: i64,
    pub ip_version: String,
    pub direction: String,
    pub action: String,
    #[serde(deserialize_with = "flex::boolean")]
    pub enabled: bool,
    pub match_criteria: Option<FirewallMatchCriteria>,
    pub admin_comment: String,
    pub rule_priority: i64,
    pub created: String,
    pub last_updated: String,
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
#[getone(path = "firewall/external-ipsets/{}", id: u32)]
#[getall(path = "firewall/external-ipsets")]
pub struct FirewallExternalIpSet {
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "firewall/manage/enabled")]
pub struct FirewallManageEnabled {
    #[serde(deserialize_with = "flex::boolean")]
    pub enabled: bool,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetAll,
    EndpointWrite,
)]
#[serde(rename_all = "snake_case")]
#[getall(path = "firewall/sets/{}/vm-list", set_id: u32)]
#[write(post, name = "attach", path = "firewall/sets/{}/vm/attach", set_id: u32, body: AttachFirewallVms, returns = Vec<FirewallSetVm>)]
#[write(post, form, name = "detach", path = "firewall/sets/{}/vm/detach/{}", set_id: u32, mbpkgid: u32, returns = ())]
#[write(post, form, name = "detach_relation", path = "firewall/sets/vm/detach/{}", relation_id: u32, returns = ())]
#[write(post, form, name = "detach_all", path = "firewall/sets/{}/vm/detach-all", set_id: u32, returns = ())]
pub struct FirewallSetVm {
    #[serde(deserialize_with = "flex::int")]
    pub id: i64,
    #[serde(deserialize_with = "flex::int")]
    pub mbpkgid: i64,
    #[serde(deserialize_with = "flex::int")]
    pub interface_id: i64,
    #[serde(deserialize_with = "flex::int")]
    pub firewall_set_id: i64,
    #[serde(deserialize_with = "flex::int")]
    pub set_priority: i64,
    pub created: String,
    pub last_updated: String,
    pub iata_code: String,
    pub location: String,
    pub hostname: String,
}

/// `firewall/sets/{id}/available-vm-list` without optional filters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "firewall/sets/{}/available-vm-list", set_id: u32)]
pub struct FirewallAvailableVms(pub Vec<FirewallSetVm>);

/// `firewall/sets/vm/{mbpkgid}/related` without optional filters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "firewall/sets/vm/{}/related", mbpkgid: u32)]
pub struct FirewallRelatedVms(pub Vec<FirewallSetVm>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallSetBody {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleBody {
    pub ip_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    pub action: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_comment: Option<String>,
    pub match_criteria: Option<FirewallMatchCriteria>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderFirewallRules {
    pub move_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_id: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachFirewallVm {
    pub mbpkgid: i64,
    pub interface_id: i64,
    pub set_priority: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachFirewallVms {
    pub vm_list: Vec<AttachFirewallVm>,
}
