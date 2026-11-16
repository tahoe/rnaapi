// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGetAll, EndpointGetOne, EndpointWrite, NaClient};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/billing-packages")]
pub struct BillingPackage {
    pub id: i64,
    pub name: String,
    pub domu_label: Option<String>,
    pub packageid: i64,
    pub domain: String,
    pub amount: String,
    pub billingcycle: String,
    pub domainstatus: String,
    pub nextduedate: String,
    pub dedicatedip: String,
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
#[getone(path = "cloud/pools/{}", id: u32)]
#[getall(path = "cloud/pools")]
pub struct CloudPool {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub required_vcpu: Option<String>,
    #[serde(default)]
    pub hard_capabilities: Vec<String>,
    #[serde(default)]
    pub soft_capabilities: Vec<String>,
    pub private: i64,
    pub backup_cloud_pool_id: Option<i64>,
    pub default_ram_price: String,
    pub default_cpu_price: String,
    pub default_disk_price: String,
    pub last_updated: String,
    pub created: String,
    pub contract_id: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/capacity?cloud_pool_id={}&location_id={}", cloud_pool_id: u32, location_id: u32)]
pub struct CloudCapacity {
    pub pkg_id: i64,
    pub pkg_name: String,
    pub pkg_cpu: i64,
    pub pkg_ram: i64,
    pub pkg_disk: i64,
    pub pkg_net: i64,
    pub pkg_port: i64,
    pub monthly_price: f64,
    pub available: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/kernels")]
pub struct Kernel {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "cloud/locations/{}", id: u32)]
pub struct CloudLocation {
    pub id: i64,
    pub name: String,
    pub location: String,
    pub city: String,
    pub country: String,
    pub iata_code: String,
    pub flag: String,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/boot-profiles")]
pub struct BootProfile {
    pub id: i64,
    pub name: String,
    pub r#type: String,
    pub description: String,
    pub builder: String,
    pub kernel: String,
    pub boot: String,
    pub serial: String,
    pub disk_represent: String,
    pub image_template: i64,
    pub last_updated: String,
    pub extra: Option<String>,
    pub vncdisplay: Option<String>,
    pub disk_root: Option<String>,
    pub bootloader: Option<String>,
    pub ramdisk: Option<String>,
    pub initrd: Option<String>,
    pub created: Option<String>,
    pub pae: i64,
    pub acpi: i64,
    pub apic: i64,
    pub xlocaltime: i64,
    pub sdl: i64,
    pub vnc: i64,
    pub vncconsole: i64,
    pub vncunused: i64,
    pub hide: i64,
    pub kvm: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(path = "cloud/server/build_status/{}", build_id: u32)]
pub struct ServerBuildStatus {
    pub id: i64,
    pub status: String,
    pub percent: i64,
    pub response: String,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetOne,
    EndpointWrite,
)]
#[serde(rename_all = "snake_case")]
#[getone(path = "cloud/contract/usage")]
#[write(post, name = "create", path = "cloud/contract/usage", body: CreateUsageContract)]
pub struct ContractUsage {
    pub id: Option<i64>,
    pub contract_mbpkgid: Option<i64>,
    pub parent_contract_id: Option<i64>,
    pub brand: Option<String>,
    pub mb_id: Option<i64>,
    pub contract_type: Option<String>,
    pub is_free: Option<i64>,
    pub include_bandwidth: Option<i64>,
    pub customer_po: Option<String>,
    pub customer_description: Option<String>,
    pub po_monthly_limit: Option<i64>,
    pub monthly_discount: Option<i64>,
    pub hourly_discount: Option<i64>,
    pub max_cpus: Option<i64>,
    pub max_ram: Option<i64>,
    pub max_disk: Option<i64>,
    pub allow_overage: Option<i64>,
}

/// `cloud/servers/{mbpkgid}/contract`. `ContractUsage` owns the account-wide call.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/servers/{}/contract", mbpkgid: u32)]
pub struct ServerContract(pub ContractUsage);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "cloud/server/{}/ipv4", mbpkgid: u32)]
pub struct ServerAddress {
    pub id: i64,
    pub ip: String,
    pub reverse: Option<String>,
    pub netmask: Option<String>,
    pub gateway: Option<String>,
    pub r#type: Option<String>,
    pub primary: Option<i64>,
}

/// `cloud/server/{mbpkgid}/ipv6`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/server/{}/ipv6", mbpkgid: u32)]
pub struct ServerIpv6(pub Vec<ServerAddress>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/serversummary/{}", mbpkgid: u32)]
pub struct ServerSummary(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/servers/unprovisioned")]
pub struct UnprovisionedServers(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/servers/usage/info")]
pub struct ServerUsageInfo(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/bw_stats/{}", mbpkgid: u32)]
pub struct BandwidthStats(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/bw_stats_range/{}", mbpkgid: u32)]
pub struct BandwidthStatsRange(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/images-provisioning-jobs-count")]
pub struct ImageProvisioningJobs(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/iplimits/{}", mbpkgid: u32)]
pub struct IpLimits(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/extras/{}", mbpkgid: u32)]
pub struct CloudExtras(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/scaling/{}", mbpkgid: u32)]
pub struct ScalingOptions(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/server/deploy/info")]
pub struct ServerDeployInfo(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/server/vnc-status/{}", mbpkgid: u32)]
pub struct ServerVncStatus(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/server/{}/networkips", mbpkgid: u32)]
pub struct ServerNetworkIps(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/server/{}/sessions", mbpkgid: u32)]
pub struct ServerBgpSessions(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/disks/{}", mbpkgid: u32)]
pub struct ServerDisks(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "cloud/storage-locations")]
pub struct StorageLocations(pub serde_json::Value);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUsageContract {
    pub mb_id: i64,
}

/// Uploads a cloud-init file. The write derive sends JSON or form fields, and this call is a file upload.
pub struct CloudInit;

impl CloudInit {
    pub async fn parse(
        client: &NaClient, filename: &str, file: &[u8],
    ) -> Result<serde_json::Value, NaApiError> {
        if filename.contains(['"', '\r', '\n', '\\']) {
            return Err(NaApiError::Request(
                "cloud-init filename contains a character that cannot be sent"
                    .into(),
            ));
        }
        let boundary = "rnaapi-cloud-init";
        let mut body = Vec::new();
        body.extend(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
            )
            .into_bytes(),
        );
        body.extend(file);
        body.extend(format!("\r\n--{boundary}--\r\n").into_bytes());
        client
            .send_data(
                "POST",
                "cloud/parse-cloud-init",
                Some(&format!("multipart/form-data; boundary={boundary}")),
                Some(body),
            )
            .await
    }
}
