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
//! let servers = client.get_servers().await;
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
// use endpoints::servers::{
//     IPv4, IPv4Data, IPv6, IPv6Data, Server, ServerData, ServersData, SrvJob, SrvJobData,
//     SrvJobsData, SrvStatus, SrvStatusData,
// };
use reqwest::ClientBuilder;
use reqwest_hickory_resolver::HickoryResolver;
use serde_json::Value;
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
    // Get Data method on client for use by endpoints to fetch the data attribute
    //
    pub async fn get_data(&self, path: &str) -> Result<Value, reqwest::Error> {
        let mut api_key = self.api_key.clone();
        if path.contains("?") {
            api_key = "&key=".to_owned() + &self.api_key;
        } else {
            api_key = "?key=".to_owned() + &self.api_key;
        }
        let result = self
            .http_client
            .get(format!("{}{}{}", self.address, path, api_key))
            .send()
            .await?
            .json::<Value>()
            .await?;
        // Get the value for "data" key out of the result since this is
        // what we want to build our structs from
        let inner_data: Option<&Value> = result.get("data");
        Ok(inner_data.unwrap().clone())
    }
}
