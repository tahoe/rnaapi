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
    pub async fn get_server(&self, mbpkgid: u32) -> Result<ServerData, reqwest::Error> {
        self.http_client
            .get(format!(
                "{}server/?key={}&mbpkgid={mbpkgid}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::ServerData>()
            .await
    }

    // Get Servers
    pub async fn get_servers(&self) -> Result<ServersData, reqwest::Error> {
        self.http_client
            .get(format!("{}servers?key={}", self.address, self.api_key))
            .send()
            .await?
            .json::<endpoints::ServersData>()
            .await
    }

    // Get Job
    pub async fn get_job(&self, mbpkgid: u32, jobid: u32) -> Result<SrvJobData, reqwest::Error> {
        self.http_client
            .get(format!(
                "{}server/{mbpkgid}/jobs/{jobid}?key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::SrvJobData>()
            .await
    }

    // Get Jobs
    pub async fn get_jobs(&self, mbpkgid: u32) -> Result<SrvJobsData, reqwest::Error> {
        self.http_client
            .get(format!(
                "{}server/{mbpkgid}/jobs?key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::SrvJobsData>()
            .await
    }

    // Get IPv4 Data
    pub async fn get_ipv4(&self, mbpkgid: u32) -> Result<IPv4Data, reqwest::Error> {
        self.http_client
            .get(format!(
                "{}ipv4?mbpkgid={mbpkgid}&key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::IPv4Data>()
            .await
    }

    // Get IPv6 Data
    pub async fn get_ipv6(&self, mbpkgid: u32) -> Result<IPv6Data, reqwest::Error> {
        self.http_client
            .get(format!(
                "{}ipv6?mbpkgid={mbpkgid}&key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::IPv6Data>()
            .await
    }

    // Get Server Status
    pub async fn get_status(&self, mbpkgid: u32) -> Result<SrvStatusData, reqwest::Error> {
        self.http_client
            .get(format!(
                "{}status/{mbpkgid}?key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<endpoints::SrvStatusData>()
            .await
    }

    //
    // Locations
    //
    pub async fn get_locations(&self) -> Result<LocationsData, reqwest::Error> {
        self.http_client
            .get(format!("{}locations?key={}", self.address, self.api_key))
            .send()
            .await?
            .json::<endpoints::LocationsData>()
            .await
    }

    //
    // Packages
    //
    pub async fn get_packages(&self) -> Result<PackagesData, reqwest::Error> {
        self.http_client
            .get(format!("{}packages?key={}", self.address, self.api_key))
            .send()
            .await?
            .json::<endpoints::PackagesData>()
            .await
    }

    //
    // Images
    //
    pub async fn get_images(&self) -> Result<ImagesData, reqwest::Error> {
        self.http_client
            .get(format!("{}images?key={}", self.address, self.api_key))
            .send()
            .await?
            .json::<endpoints::ImagesData>()
            .await
    }
}
