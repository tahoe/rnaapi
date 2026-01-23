// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndPointGetArgs, EndpointGet, NaClient};
use async_trait::async_trait;

//
// Server struct
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Server {
    pub city: String,
    pub fqdn: String,
    #[serde(rename = "domU_package")]
    pub domu_package: u32,
    pub mbpkgid: u32,
    pub os_id: u32,
    pub location_id: u32,
    pub ip: String,
    pub ipv6: String,
    pub plan_id: u32,
    pub pkg_id: u32,
    pub state: String,
    pub status: String,
    pub uptime: String,
    pub installed: u32,
    pub nic1_mac: String,
    pub nic2_mac: String,
}

#[async_trait]
impl EndpointGet for Server {
    type Endpoint = Server;
    async fn get_one(
        na_client: &NaClient, args: EndPointGetArgs,
    ) -> Result<Server, NaApiError> {
        match args {
            EndPointGetArgs::OneInt(id) => {
                let data = na_client
                    .get_data(&format!("cloud/server?mbpkgid={id}").to_owned())
                    .await?;
                let server: Server = serde_json::from_value(data).unwrap();
                Ok(server)
            }
            _ => Err(NaApiError::UnknownError(
                "Only one argument allowed".to_owned(),
            )),
        }
    }

    async fn get_all(
        na_client: &NaClient, args: EndPointGetArgs,
    ) -> Result<Vec<Server>, NaApiError> {
        match args {
            EndPointGetArgs::NoArgs => {
                let data = na_client.get_data("cloud/servers").await?;
                let servers: Vec<Server> =
                    serde_json::from_value(data).unwrap();
                Ok(servers)
            }
            _ => Err(NaApiError::UnknownError(
                "Only one argument allowed".to_owned(),
            )),
        }
    }
}

//
// Job struct
// URL: https://vapi2.netactuate.com/api/cloud/server/{mbpkgid}/jobs/{jobid}
// URL: https://vapi2.netactuate.com/api/cloud/server/{mbpkgid}/jobs
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SrvJob {
    pub id: u32,
    // #[serde(with = "custom_datetime_format_seconds")]
    // pub ts_insert: NaiveDateTime,
    pub ts_insert: String,
    pub command: String,
    pub status: u32,
}

impl SrvJob {
    pub async fn get_one(
        na_client: &NaClient, mbpkgid: u32, jobid: u32,
    ) -> Result<SrvJob, NaApiError> {
        let data = na_client
            .get_data(&format!("cloud/server/{mbpkgid}/{jobid}").to_owned())
            .await?;
        let srvjob: SrvJob = serde_json::from_value(data).unwrap();
        Ok(srvjob)
    }

    pub async fn get_all(
        na_client: &NaClient, mbpkgid: u32,
    ) -> Result<Vec<SrvJob>, NaApiError> {
        let data = na_client
            .get_data(&format!("cloud/server/{mbpkgid}/jobs"))
            .await?;
        let srvjobs: Vec<SrvJob> = serde_json::from_value(data).unwrap();
        Ok(srvjobs)
    }
}

//
// Status struct
// URL: https://vapi2.netactuate.com/api/cloud/status/{mbpkgid}
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SrvStatus {
    pub status: String,
}

impl SrvStatus {
    pub async fn get_all(
        na_client: &NaClient, mbpkgid: u32,
    ) -> Result<SrvStatus, NaApiError> {
        let data = na_client
            .get_data(&format!("cloud/status/{mbpkgid}"))
            .await?;
        let srvstatus: SrvStatus = serde_json::from_value(data).unwrap();
        Ok(srvstatus)
    }
}

//
// IPv4IP struct
// URL: https://vapi2.netactuate.com/api/cloud/ipv4?mbpkgid=<mbpkgid>&key=<api_key>
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IPv4 {
    pub id: u32,
    pub primary: u32,
    pub reverse: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub broadcast: String,
}

impl IPv4 {
    pub async fn get_all(
        na_client: &NaClient, mbpkgid: u32,
    ) -> Result<Vec<IPv4>, NaApiError> {
        let data = na_client
            .get_data(&format!("cloud/ipv4?mbpkgid={mbpkgid}"))
            .await?;
        let ipv4: Vec<IPv4> = serde_json::from_value(data).unwrap();
        Ok(ipv4)
    }
}

//
// IPv6IP struct
// URL: https://vapi2.netactuate.com/api/cloud/ipv6?mbpkgid=<mbpkgid>&key=<api_key>
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IPv6 {
    pub id: u32,
    pub primary: u32,
    pub reverse: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub broadcast: String,
}

impl IPv6 {
    pub async fn get_all(
        na_client: &NaClient, mbpkgid: u32,
    ) -> Result<Vec<IPv6>, NaApiError> {
        let data = na_client
            .get_data(&format!("cloud/ipv6?mbpkgid={mbpkgid}"))
            .await?;
        let ipv6: Vec<IPv6> = serde_json::from_value(data).unwrap();
        Ok(ipv6)
    }
}
