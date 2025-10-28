#![allow(unused)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use reqwest::ClientBuilder;
use reqwest_hickory_resolver::HickoryResolver;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

pub mod domain;

pub struct Application {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl Application {
    pub async fn new(address: String) -> Self {
        // build the client to use local resolver, IE Ipv4
        let mut builder = ClientBuilder::new();
        builder = builder.dns_resolver(Arc::new(HickoryResolver::default()));
        let http_client = builder.build().unwrap();
        Self {
            address,
            http_client,
        }
    }
}
