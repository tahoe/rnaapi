//! Light weight Rust API client library for the NetActuate API
//! Rust library for talking to the NetActuate API
//!
//! This library provides the methods for establishing a connection
//! and for retrieving data from most endpoints
//!
//! It also includes an example app that just prints out some information
//! The app can be installed with `cargo install rnaapi`
//!
//! # Usage
//!
//! ## Import the library
//!
//! ```rust
//! cargo add rnaapi
//! ```
//!
//! ## Set up your environment
//!
//! ```bash
//! export API_KEY='<your api key>'
//! export API_ADDRESS='https://vapi2.netactuate.com/api/cloud'
//! ```
//!
//! ## Import the config that uses the environment
//!
//! ```rust
//! use rnaapi::config::{API_ADDRESS, API_KEY}
//! use rnaapi::endpoints::{Server, ServerData, ServersData}
//! use rnaapi::NaClient
//! // whatever other libraries you want to use like serde_json, serde::Serialize...
//! ```
//!
//! ## Simplest example
//!
//! ```rust
//! // with above imports
//! let client = NaClient::new(API_KEY.to_owned(), API_ADDRESS.to_owned()).await;
//! let servers_result = client.get_servers().await;
//! let servers = servers_result.unwrap().servers;
//! for server in servers {
//!     println!("fqdn: {}, mbpkgid: {}", server.fqdn, server.mbpkgid);
//! }
//! ```
//!
//!
#![allow(unused)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use config::API_KEY;
use endpoints::images::{Image, ImageData, ImagesData};
use endpoints::locations::{Location, LocationsData};
use endpoints::packages::{Package, PackageData, PackagesData};
use endpoints::servers::{
    IPv4, IPv4Data, IPv6, IPv6Data, Server, ServerData, ServersData, SrvJob, SrvJobData,
    SrvJobsData, SrvStatus, SrvStatusData,
};
use reqwest::ClientBuilder;
use reqwest_hickory_resolver::HickoryResolver;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

pub mod config;
pub mod endpoints;

pub struct NaClient {
    pub address: String,
    pub api_key: String,
    pub http_client: reqwest::Client,
}

impl NaClient {
    pub async fn new(api_key: String, address: String) -> Self {
        // build the client to use local resolver, IE Ipv4
        let mut builder = ClientBuilder::new();
        builder = builder.dns_resolver(Arc::new(HickoryResolver::default()));
        let http_client = builder.build().unwrap();
        Self {
            api_key,
            address,
            http_client,
        }
    }

    //
    // Server endpoints
    //

    // Get Server
    pub async fn get_server(&self, mbpkgid: u32) -> Result<Server, reqwest::Error> {
        let server_data = self
            .http_client
            .get(format!(
                "{}server/?key={}&mbpkgid={mbpkgid}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::ServerData>()
            .await?;
        Ok(server_data.data)
    }

    // Get Servers
    pub async fn get_servers(&self) -> Result<Vec<Server>, reqwest::Error> {
        let servers_data = self
            .http_client
            .get(format!("{}servers?key={}", self.address, self.api_key))
            .send()
            .await?
            .json::<endpoints::ServersData>()
            .await?;
        Ok(servers_data.data)
    }

    // Get Job
    pub async fn get_job(&self, mbpkgid: u32, jobid: u32) -> Result<SrvJob, reqwest::Error> {
        let server_job_data = self
            .http_client
            .get(format!(
                "{}server/{mbpkgid}/jobs/{jobid}?key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::SrvJobData>()
            .await?;
        Ok(server_job_data.data)
    }

    // Get Jobs
    pub async fn get_jobs(&self, mbpkgid: u32) -> Result<Vec<SrvJob>, reqwest::Error> {
        let server_jobs_data = self
            .http_client
            .get(format!(
                "{}server/{mbpkgid}/jobs?key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::SrvJobsData>()
            .await?;
        Ok(server_jobs_data.data)
    }

    // Get IPv4 Data
    pub async fn get_ipv4(&self, mbpkgid: u32) -> Result<Vec<IPv4>, reqwest::Error> {
        let ipv4_data = self
            .http_client
            .get(format!(
                "{}ipv4?mbpkgid={mbpkgid}&key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::IPv4Data>()
            .await?;
        Ok(ipv4_data.data)
    }

    // Get IPv6 Data
    pub async fn get_ipv6(&self, mbpkgid: u32) -> Result<Vec<IPv6>, reqwest::Error> {
        let ipv6_data = self
            .http_client
            .get(format!(
                "{}ipv6?mbpkgid={mbpkgid}&key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::IPv6Data>()
            .await?;
        Ok(ipv6_data.data)
    }

    // Get Server Status
    pub async fn get_status(&self, mbpkgid: u32) -> Result<SrvStatus, reqwest::Error> {
        let srv_status_data = self
            .http_client
            .get(format!(
                "{}status/{mbpkgid}?key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::SrvStatusData>()
            .await?;
        Ok(srv_status_data.data)
    }

    //
    // Locations
    //
    pub async fn get_locations(&self) -> Result<Vec<Location>, reqwest::Error> {
        let locations_data = self
            .http_client
            .get(format!("{}locations?key={}", self.address, self.api_key))
            .send()
            .await?
            .json::<endpoints::LocationsData>()
            .await?;
        Ok(locations_data.data)
    }

    //
    // Packages
    //
    pub async fn get_packages(&self) -> Result<Vec<Package>, reqwest::Error> {
        let pkgs_data = self
            .http_client
            .get(format!("{}packages?key={}", self.address, self.api_key))
            .send()
            .await?
            .json::<endpoints::PackagesData>()
            .await?;
        Ok(pkgs_data.data)
    }

    //
    // Images
    //
    pub async fn get_images(&self) -> Result<Vec<Image>, reqwest::Error> {
        let imgs_data = self
            .http_client
            .get(format!("{}images?key={}", self.address, self.api_key))
            .send()
            .await?
            .json::<endpoints::ImagesData>()
            .await?;
        Ok(imgs_data.data)
    }
}
