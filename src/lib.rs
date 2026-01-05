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
//! ## Set up your environment, note that the API_ADDRESS will be appended
//! to based on the endpoints
//!
//! ```bash
//! export API_KEY='<your api key>'
//! export API_ADDRESS='https://vapi2.netactuate.com/api/'
//! ```
//!
//! ## Import the config that uses the environment
//!
//! ```rust
//! use rnaapi::config::Settings;
//! use rnaapi::NaClient;
//! // whatever other libraries you want to use like serde_json, serde::Serialize...
//! ```
//!
//! ## Simplest example
//!
//! ```rust
//! // with above imports
//! let settings = Settings.new();
//! let client = NaClient::new(settings.api_key, settings.api_url).await;
//! let servers = client.get_servers().await;
//! for server in servers {
//!     println!("fqdn: {}, mbpkgid: {}", server.fqdn, server.mbpkgid);
//! }
//! ```
//!
//!
// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use errors::NaApiError;
use reqwest::ClientBuilder;
use reqwest_hickory_resolver::HickoryResolver;
use serde_json::Value;
use std::sync::Arc;

pub mod config;
pub mod endpoints;
pub mod errors;

pub struct NaClient {
    pub address: String,
    pub api_key: String,
    pub http_client: reqwest::Client,
}

impl NaClient {
    /// build the client to use local resolver, IE Ipv4
    pub async fn new(api_key: String, address: String) -> Self {
        let mut builder = ClientBuilder::new();
        builder = builder.dns_resolver(Arc::new(HickoryResolver::default()));
        let http_client = builder.build().unwrap();
        Self {
            api_key,
            address,
            http_client,
        }
    }

    /// Make a request for the client
    async fn get(&self, path: &str) -> Result<Value, NaApiError> {
        let api_key = if path.contains("?") {
            format!("&key={}", &self.api_key)
        } else {
            format!("?key={}", &self.api_key)
        };
        let result = self
            .http_client
            .get(format!("{}{}{}", self.address, path, api_key))
            .send()
            .await
            .map_err(|e| {
                NaApiError::UnknownError(format!(
                    "Failed to finish request with error: {e}"
                ))
            })?;
        let result_json = result.json::<Value>().await.map_err(|e| {
            NaApiError::UnknownError(format!(
                "Failed to finish request with error: {e}"
            ))
        })?;
        Ok(result_json)
    }

    /// Call the get and parse the results
    /// We want to get the "data" attribute from the response for the calling
    /// endpoint. Exit with error message if "data" is not present
    /// This is shitty but it is safe enough so far as I can tell at this point
    pub async fn get_data(&self, path: &str) -> Result<Value, NaApiError> {
        // Get the response from get method
        let result = self
            .get(path)
            .await
            .map_err(|e| NaApiError::UnknownError(format!("Got error: {e}")))?;

        // Try to pull the "data" key from the response
        let result_value: Option<&Value> = result.get("data");
        if let Some(inner_data) = result_value {
            Ok(inner_data.clone())
        } else {
            let result_message = result.get("message");
            if let Some(message) = result_message {
                let code = result.get("code").unwrap();
                Err(NaApiError::APIKeyInvalid(format!("{code}: {message}")))
            } else {
                Err(NaApiError::UnknownError(format!(
                    "Could not reach: {}{}",
                    self.api_key, path
                )))
            }
        }
    }
}
