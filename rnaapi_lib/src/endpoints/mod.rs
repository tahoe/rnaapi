pub mod account;
pub mod bgp;
pub mod cloud;
pub mod ddos;
pub mod dedicated;
pub mod dns;
pub mod firewall;
pub mod hosting;
pub mod networking;
pub mod platform;
pub mod secrets;
pub mod services;
pub mod support;
pub mod tags;

mod flex;

pub use account::*;
pub use bgp::*;
pub use cloud::*;
pub use ddos::*;
pub use dedicated::*;
pub use dns::*;
pub use firewall::*;
pub use hosting::*;
pub use networking::*;
pub use platform::*;
pub use secrets::*;
pub use services::*;
pub use support::*;
pub use tags::*;

#[cfg(test)]
mod shape_tests {
    use super::{BgpSession, ColocationPackages, FirewallSet, PlatformStatus};
    use serde_json::json;

    #[test]
    fn colocation_packages_deserialize_from_a_map() {
        let body = json!({
            "10": {
                "mbpkgid": 10,
                "package_status": "Active",
                "fqdn": "colo.example",
                "billingcycle": "monthly",
                "nextduedate": "2026-12-01",
                "amount": "1.00",
                "details": {
                    "dc_name": "iad",
                    "iata_code": "IAD",
                    "bw_commit": "1",
                    "overage_type": "95",
                    "overage_rate": "0",
                    "agg_bw_mbpkgid": "0"
                },
                "status": "ok"
            }
        });
        let packages: ColocationPackages =
            serde_json::from_value(body).unwrap();
        assert_eq!(packages.0["10"].mbpkgid, 10);
    }

    #[test]
    fn firewall_set_accepts_numeric_flags() {
        let body = json!({
            "id": 1,
            "name": "edge",
            "description": "",
            "enabled": 1,
            "is_draft": "0",
            "draft_firewall_set_id": null,
            "created": "2026-01-01",
            "last_updated": "2026-01-02"
        });
        let set: FirewallSet = serde_json::from_value(body).unwrap();
        assert!(set.enabled);
        assert!(!set.is_draft);
    }

    #[test]
    fn bgp_session_accepts_string_asns() {
        let body = json!({
            "id": 4,
            "customer_peer_ip": "203.0.113.2",
            "group_id": 1,
            "locked": 0,
            "description": "",
            "config_status": 1,
            "export_list": "",
            "provider_peer_ip": "203.0.113.1",
            "location": "iad",
            "latitude": "0",
            "longitude": "0",
            "group_name": "edge",
            "provider_ip_type": "ipv4",
            "provider_asn": "65000",
            "customer_asn": 65001
        });
        let session: BgpSession = serde_json::from_value(body).unwrap();
        assert_eq!(session.provider_asn, "65000");
        assert_eq!(session.customer_asn, "65001");
    }

    #[test]
    fn platform_status_deserializes_from_a_map() {
        let body = json!({
            "api": {
                "component_id": "api",
                "locations": {
                    "iad": {
                        "container_id": "c1",
                        "status": "operational",
                        "last_updated": "2026-01-01"
                    }
                }
            }
        });
        let status: PlatformStatus = serde_json::from_value(body).unwrap();
        assert_eq!(status.0["api"].locations["iad"].status, "operational");
    }
}
