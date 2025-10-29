#![allow(unused)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use config::API_KEY;
use endpoints::servers::{Server, ServerData, ServersData};
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

    pub async fn get_servers(&self) -> Result<ServersData, reqwest::Error> {
        self.http_client
            .get(format!("{}servers/?key={}", self.address, self.api_key))
            .send()
            .await?
            .json::<endpoints::ServersData>()
            .await
    }
}
