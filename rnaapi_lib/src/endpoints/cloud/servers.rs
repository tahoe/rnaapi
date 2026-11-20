// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne, EndpointWrite};

//
// Server struct
//
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
#[getall(path = "cloud/servers")]
#[getone(path = "cloud/server?mbpkgid={}", mbpkgid: u32)]
#[write(post, form, name = "buy", path = "cloud/server/buy_build", body: ServerBuildRequest, returns = ServerBuild)]
#[write(post, form, name = "rebuild", path = "cloud/server/build/{}", id: u32, body: ServerBuildRequest, returns = ServerBuild)]
// `DeleteServer` serializes to `{}`.
#[write(post, name = "delete", path = "cloud/server/delete/{}", mbpkgid: u32, body: DeleteServer, returns = DeletedServer)]
#[write(post, form, name = "unlink", path = "cloud/server/{}/unlink", mbpkgid: u32, returns = ())]
#[write(post, name = "start", path = "cloud/server/{}/start", mbpkgid: u32, body: ServerPower, returns = serde_json::Value)]
#[write(post, name = "shutdown", path = "cloud/server/{}/shutdown", mbpkgid: u32, body: ServerPower, returns = serde_json::Value)]
#[write(post, name = "reboot", path = "cloud/server/{}/reboot", mbpkgid: u32, body: ServerPower, returns = serde_json::Value)]
#[write(post, name = "rescue_start", path = "cloud/server/{}/rescue_start", mbpkgid: u32, body: RescueStart, returns = serde_json::Value)]
#[write(post, form, name = "rescue_stop", path = "cloud/server/{}/rescue_stop", mbpkgid: u32, returns = ())]
#[write(post, name = "reset_password", path = "cloud/server/{}/password", mbpkgid: u32, body: ResetRootPassword, returns = serde_json::Value)]
#[write(post, form, name = "fsck", path = "cloud/server/{}/fsck", mbpkgid: u32, returns = ())]
#[write(post, form, name = "reconfigure_network", path = "cloud/server/{}/netconfig", mbpkgid: u32, returns = ())]
#[write(post, json, name = "vnc", path = "cloud/server/{}/vnc", mbpkgid: u32, returns = serde_json::Value)]
#[write(post, name = "scale", path = "cloud/scale/{}", id: u32, body: ScaleServer, returns = i64)]
#[write(put, name = "update_options", path = "cloud/options/{}", mbpkgid: u32, body: ServerOptions, returns = serde_json::Value)]
#[write(post, name = "bind_firewall", path = "cloud/{}/firewall-sets", mbpkgid: u32, body: BindFirewallSet, returns = serde_json::Value)]
#[write(delete, name = "unbind_firewall", path = "cloud/{}/firewall-sets/{}", mbpkgid: u32, firewall_set: String, returns = ())]
#[write(post, name = "attempt_ssh", path = "cloud/servers/attempt-ssh", body: AttemptSsh, returns = serde_json::Value)]
pub struct Server {
    pub city: String,
    pub fqdn: String,
    #[serde(rename = "domU_package")]
    pub domu_package: u32,
    pub mbpkgid: u32,
    pub os_id: u32,
    pub location_id: u32,
    pub ip: String,
    pub ipv6: Option<String>,
    pub plan_id: u32,
    pub pkg_id: u32,
    /// Plan name from `cloud/servers`, for example `VR4x1x25`.
    #[serde(default)]
    pub package: Option<String>,
    pub state: Option<String>,
    pub status: Option<String>,
    pub uptime: Option<String>,
    pub installed: u32,
    pub nic1_mac: Option<String>,
    pub nic2_mac: Option<String>,
}

// Job struct
// URL: https://vapi2.netactuate.com/api/cloud/server/{mbpkgid}/jobs/{jobid}
// URL: https://vapi2.netactuate.com/api/cloud/server/{mbpkgid}/jobs
//
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetAll,
    EndpointGetOne,
)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/server/{}/jobs", mbpkgid: u32)]
#[getone(path = "cloud/server/{}/jobs/{}", mbpkgid: u32, jobid: u32)]
pub struct SrvJob {
    pub id: u32,
    // #[serde(with = "custom_datetime_format_seconds")]
    // pub ts_insert: NaiveDateTime,
    pub ts_insert: String,
    pub command: String,
    pub status: u32,
}

//
// Status struct
// URL: https://vapi2.netactuate.com/api/cloud/status/{mbpkgid}
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "cloud/status/{}", mbpkgid: u32)]
pub struct SrvStatus {
    pub status: String,
}

//
// IPv4IP struct
// URL: https://vapi2.netactuate.com/api/cloud/ipv4?mbpkgid=<mbpkgid>&key=<api_key>
//
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
#[getall(path = "cloud/ipv4?mbpkgid={}", mbpkgid: u32)]
#[write(put, name = "update_reverse", path = "cloud/ipv4/{}", id: u32, body: ReverseDns, returns = ())]
pub struct IPv4 {
    pub id: u32,
    pub primary: u32,
    pub reverse: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub broadcast: String,
}

//
// IPv6IP struct
// URL: https://vapi2.netactuate.com/api/cloud/ipv6?mbpkgid=<mbpkgid>&key=<api_key>
//
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
#[getall(path = "cloud/ipv6?mbpkgid={}", mbpkgid: u32)]
#[write(put, name = "update_reverse", path = "cloud/ipv6/{}", id: u32, body: ReverseDns, returns = ())]
pub struct IPv6 {
    pub id: u32,
    pub primary: u32,
    pub reverse: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub broadcast: String,
}

//
// Monthly bandwidth for one server.
// URL: https://vapi2.netactuate.com/api/cloud/servermonthlybw/{mbpkgid}
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/servermonthlybw/{}", mbpkgid: u32)]
pub struct MonthlyBw {
    pub date: String,
    pub rx: u32,
    pub tx: u32,
}

/// `cloud/servers/current`. `Server` already owns `get` and `list`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/servers/current")]
pub struct CurrentServer(pub Server);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServerBuild {
    pub mbpkgid: i64,
    pub status: String,
    pub build: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServerBuildRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_billing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_billing_contract_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_pool_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<i64>,
}

/// `POST cloud/server/delete/{mbpkgid}`. The body is `{}`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeleteServer {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedServer {
    pub id: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServerPower {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RescueStart {
    pub rescue_pass: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetRootPassword {
    pub rootpass: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleServer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg_id: Option<i64>,
    pub allow_reboot: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServerOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorescue: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindFirewallSet {
    pub firewall_set_id: i64,
    pub interface_id: i64,
    pub set_priority: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttemptSsh {
    pub mbpkgid: i64,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReverseDns {
    pub reverse: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_new_server_accepts_null_strings() {
        let mut value = json!({
            "city": "Example City",
            "fqdn": "web-01.example",
            "domU_package": 1,
            "mbpkgid": 1001,
            "os_id": 1,
            "location_id": 3,
            "ip": "192.0.2.10",
            "ipv6": null,
            "plan_id": 1,
            "pkg_id": 1,
            "state": null,
            "status": null,
            "uptime": null,
            "installed": 0,
            "nic1_mac": null,
            "nic2_mac": null,
            "virtual": null
        });
        let server: Server = serde_json::from_value(value.clone()).unwrap();
        assert!(server.ipv6.is_none());
        assert!(server.state.is_none());
        assert!(server.status.is_none());
        assert!(server.uptime.is_none());
        assert!(server.nic1_mac.is_none());
        assert!(server.nic2_mac.is_none());
        assert!(server.package.is_none());
        assert_eq!(server.fqdn, "web-01.example");
        assert_eq!(server.mbpkgid, 1001);
        value["package"] = json!("VR4x1x25");
        let named: Server = serde_json::from_value(value).unwrap();
        assert_eq!(named.package.as_deref(), Some("VR4x1x25"));
    }

    #[test]
    fn delete_server_body_is_empty() {
        let value = serde_json::to_value(DeleteServer::default()).unwrap();
        assert_eq!(value, json!({}));
        let parsed: DeleteServer = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, DeleteServer::default());
    }
}
