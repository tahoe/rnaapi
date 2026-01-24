// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::errors::NaApiError;
use crate::{EndpointGet, EndpointGetArgs, NaClient};
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
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<Server, NaApiError> {
        match args {
            EndpointGetArgs::OneInt(mbpkgid) => {
                let data = na_client
                    .get_data(
                        &format!("cloud/server?mbpkgid={mbpkgid}").to_owned(),
                    )
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
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<Vec<Server>, NaApiError> {
        match args {
            EndpointGetArgs::NoArgs => {
                let data = na_client.get_data("cloud/servers").await?;
                let servers: Vec<Server> =
                    serde_json::from_value(data).unwrap();
                Ok(servers)
            }
            _ => Err(NaApiError::UnknownError("No args allowed".to_owned())),
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

#[async_trait]
impl EndpointGet for SrvJob {
    type Endpoint = SrvJob;
    async fn get_one(
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<SrvJob, NaApiError> {
        match args {
            EndpointGetArgs::TwoInt(mbpkgid, jobid) => {
                let data = na_client
                    .get_data(
                        &format!("cloud/server/{mbpkgid}/{jobid}").to_owned(),
                    )
                    .await?;
                let srvjob: SrvJob = serde_json::from_value(data).unwrap();
                Ok(srvjob)
            }
            _ => Err(NaApiError::UnknownError(
                "Two u32 args are required".to_owned(),
            )),
        }
    }

    async fn get_all(
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<Vec<SrvJob>, NaApiError> {
        match args {
            EndpointGetArgs::OneInt(mbpkgid) => {
                let data = na_client
                    .get_data(&format!("cloud/server/{mbpkgid}/jobs"))
                    .await?;
                let srvjobs: Vec<SrvJob> =
                    serde_json::from_value(data).unwrap();
                Ok(srvjobs)
            }
            _ => Err(NaApiError::UnknownError("No args allowed".to_owned())),
        }
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

#[async_trait]
impl EndpointGet for SrvStatus {
    type Endpoint = SrvStatus;
    async fn get_one(
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<SrvStatus, NaApiError> {
        match args {
            EndpointGetArgs::OneInt(mbpkgid) => {
                let data = na_client
                    .get_data(&format!("cloud/status/{mbpkgid}"))
                    .await?;
                let srvstatus: SrvStatus =
                    serde_json::from_value(data).unwrap();
                Ok(srvstatus)
            }
            _ => {
                Err(NaApiError::UnknownError("Only one arg allowed".to_owned()))
            }
        }
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

#[async_trait]
impl EndpointGet for IPv4 {
    type Endpoint = IPv4;
    async fn get_all(
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<Vec<IPv4>, NaApiError> {
        match args {
            EndpointGetArgs::OneInt(mbpkgid) => {
                let data = na_client
                    .get_data(&format!("cloud/ipv4?mbpkgid={mbpkgid}"))
                    .await?;
                let ipv4: Vec<IPv4> = serde_json::from_value(data).unwrap();
                Ok(ipv4)
            }
            _ => {
                Err(NaApiError::UnknownError("Only one arg allowed".to_owned()))
            }
        }
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

#[async_trait]
impl EndpointGet for IPv6 {
    type Endpoint = IPv6;
    async fn get_all(
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<Vec<IPv6>, NaApiError> {
        match args {
            EndpointGetArgs::OneInt(mbpkgid) => {
                let data = na_client
                    .get_data(&format!("cloud/ipv6?mbpkgid={mbpkgid}"))
                    .await?;
                let ipv6: Vec<IPv6> = serde_json::from_value(data).unwrap();
                Ok(ipv6)
            }
            _ => {
                Err(NaApiError::UnknownError("Only one arg allowed".to_owned()))
            }
        }
    }
}
