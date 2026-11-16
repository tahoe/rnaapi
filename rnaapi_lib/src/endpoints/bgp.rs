// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
#[getone(path = "bgp/bgpsession/{}", id: u32)]
#[getall(path = "bgp/bgpsessions")]
#[write(post, form, name = "create", path = "bgp/bgpcreatesessions", body: CreateBgpSessions)]
#[write(post, form, name = "delete", path = "bgp/bgpsession/{}/delete", id: u32, returns = ())]
#[write(post, form, name = "refresh", path = "bgp/bgpsession/{}/refresh", id: u32, returns = ())]
#[write(post, form, name = "start", path = "bgp/bgpsession/{}/start", id: u32, returns = ())]
#[write(post, form, name = "stop", path = "bgp/bgpsession/{}/stop", id: u32, returns = ())]
pub struct BgpSession {
    pub id: i64,
    pub customer_peer_ip: String,
    pub group_id: i64,
    pub locked: i64,
    pub description: String,
    #[serde(default)]
    pub state: Value,
    #[serde(default)]
    pub routes_received: Value,
    #[serde(default)]
    pub last_update: Value,
    pub config_status: i64,
    #[serde(default)]
    pub password: Value,
    #[serde(default)]
    pub prefixes: Vec<BgpSessionPrefix>,
    pub export_list: String,
    #[serde(default)]
    pub community: Value,
    pub provider_peer_ip: String,
    pub location: String,
    pub latitude: String,
    pub longitude: String,
    pub group_name: String,
    pub provider_ip_type: String,
    #[serde(deserialize_with = "flex::id")]
    pub provider_asn: String,
    #[serde(deserialize_with = "flex::id")]
    pub customer_asn: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BgpSessionPrefix {
    pub id: i64,
    pub mb_id: i64,
    pub prefix: String,
    #[serde(default)]
    pub append: Value,
    pub rule_type: String,
    pub prefix_type: String,
    pub description: String,
    pub date: String,
    pub allowed_pps: i64,
    pub bgp_group_id: i64,
    pub prefix_id: i64,
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
#[getone(path = "bgp/bgpgroup/{}", id: u32)]
#[getall(path = "bgp/bgpgroups")]
#[write(post, name = "create", path = "bgp/bgpgroup", body: CreateBgpGroup)]
#[write(post, form, name = "refresh", path = "bgp/bgpgroup/{}/refresh", id: u32, returns = ())]
#[write(post, form, name = "start", path = "bgp/bgpgroup/{}/start", id: u32, returns = ())]
#[write(post, form, name = "stop", path = "bgp/bgpgroup/{}/stop", id: u32, returns = ())]
#[write(post, name = "bind_firewall", path = "bgp/bgp-groups/{}/firewall-sets", id: u32, body: BindBgpFirewall, returns = BgpFirewallBinding)]
#[write(delete, name = "unbind_firewall", path = "bgp/bgp-groups/{}/firewall-sets/{}", id: u32, firewall_set_id: u32, returns = ())]
pub struct BgpGroup {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub group_type: String,
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
#[getone(path = "bgp/bgpprefix/{}", id: u32)]
#[getall(path = "bgp/bgpprefixes")]
#[write(post, name = "buy", path = "bgp/bgpbuyprefixes", body: BuyBgpPrefixes)]
pub struct BgpPrefix {
    pub id: i64,
    pub name: String,
    pub prefix: String,
    pub group_id: i64,
    pub asn_id: i64,
    pub anycast_profile: i64,
    pub agreement_id: i64,
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
#[getone(path = "bgp/bgpasn?id={}", id: u32)]
#[getall(path = "bgp/bgpasns")]
pub struct BgpAsn {
    pub id: i64,
    pub asn: i64,
    pub name: String,
    pub group_type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "bgp/bgpsummary")]
pub struct BgpSummary(pub std::collections::HashMap<String, Value>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "bgp/dashboard")]
pub struct BgpDashboard(pub std::collections::HashMap<String, Value>);

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CreateBgpSessions {
    pub mbpkgid: i64,
    pub group_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBgpGroup {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuyBgpPrefixes {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_profile: Option<i64>,
    pub agreement_id: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindBgpFirewall {
    pub id: i64,
    pub firewall_set_id: i64,
    pub interface_number: i64,
    pub set_priority: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BgpFirewallBinding {
    pub id: i64,
    pub bgp2_group_id: i64,
    pub firewall_set_id: i64,
    pub interface_number: i64,
    pub set_priority: i64,
}
