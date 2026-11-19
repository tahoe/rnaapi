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
#[getone(path = "cloud/package/{}", id: u32)]
#[getall(path = "cloud/packages")]
#[write(post, name = "cancel", path = "cloud/package/cancel", body: CancelPackage, returns = serde_json::Value)]
pub struct Package {
    pub mbpkgid: u32,
    pub package_status: String,
    pub fqdn: String,
    pub name: String,
    pub gid: u32,
    #[serde(rename = "domU_package")]
    pub domu_package: u32,
    pub rescue: u32,
    pub locked: u32,
    pub package: String,
    pub ipv6: String,
    pub city: String,
    pub ip: String,
    pub installed: u32,
    pub state: String,
    pub uptime: String,
    pub os: String,
    pub is_building: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancelPackage {
    pub mbpkgid: i64,
    /// `Server::plan_id`. The portal key is `domU_package`.
    #[serde(
        default,
        rename = "domU_package",
        skip_serializing_if = "Option::is_none"
    )]
    pub domu_package: Option<u32>,
    /// Always sent. `None` is JSON null, matching the portal cancel body.
    #[serde(default)]
    pub comments: Option<String>,
    pub cancel_type: String,
    pub agree: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Usage contract id (`ContractUsage.id`), not `contract_mbpkgid`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    /// Plan name from the server list, for example `VR4x1x25`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    /// `Server::pkg_id` written as a string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pkg_id: Option<String>,
    /// `Server::plan_id`, sent again beside `domU_package`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_package_keeps_comments_null_and_skips_missing_fields() {
        let body = CancelPackage {
            mbpkgid: 1001,
            domu_package: Some(42),
            comments: None,
            cancel_type: "Immediate".into(),
            agree: true,
            password: None,
            contract_id: Some(7),
            fqdn: Some("web-01.example".into()),
            package: Some("VR4x1x25".into()),
            pkg_id: Some("9".into()),
            plan_id: Some(42),
        };
        let value = serde_json::to_value(&body).unwrap();
        assert_eq!(value["mbpkgid"], 1001);
        assert_eq!(value["domU_package"], 42);
        assert!(value["comments"].is_null());
        assert_eq!(value["cancel_type"], "Immediate");
        assert_eq!(value["agree"], true);
        assert_eq!(value["contract_id"], 7);
        assert_eq!(value["fqdn"], "web-01.example");
        assert_eq!(value["package"], "VR4x1x25");
        assert_eq!(value["pkg_id"], "9");
        assert_eq!(value["plan_id"], 42);
        assert!(value.get("password").is_none(), "{value}");
        assert!(value.get("domu_package").is_none(), "{value}");
        let omitted = CancelPackage {
            domu_package: None,
            fqdn: None,
            package: None,
            pkg_id: None,
            plan_id: None,
            contract_id: None,
            ..body.clone()
        };
        let skipped = serde_json::to_value(&omitted).unwrap();
        assert!(skipped["comments"].is_null());
        assert_eq!(skipped["mbpkgid"], 1001);
        assert_eq!(skipped["agree"], true);
        assert_eq!(skipped["cancel_type"], "Immediate");
        for absent in [
            "domU_package",
            "fqdn",
            "package",
            "pkg_id",
            "plan_id",
            "contract_id",
            "password",
        ] {
            assert!(skipped.get(absent).is_none(), "{absent} {skipped}");
        }
        let parsed: CancelPackage = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, body);
        assert!(parsed.comments.is_none());
    }
}
