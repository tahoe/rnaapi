// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use crate::NaApiError;
use crate::{EndpointGet, EndpointGetArgs, NaClient};
use async_trait::async_trait;

///
/// Account Details #[derive(Debug)]
///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Invoices {
    pub id: u32,
    pub userid: u32,
    pub date: String,
    pub duedate: String,
    pub subtotal: String,
    pub credit: String,
    pub status: String,
    pub paymentmethod: String,
    // #[serde(with = "custom_datetime_format_seconds")]
    // pub datepaid: NaiveDateTime,
    // #[serde(with = "custom_datetime_format_seconds")]
    // pub created_at: NaiveDateTime,
    // #[serde(with = "custom_datetime_format_seconds")]
    // pub updated_at: NaiveDateTime,
    //
    // pub datepaid: String,
    // pub created_at: String,
    // pub updated_at: String,
}

// Get Details
#[async_trait]
impl EndpointGet for Invoices {
    type Endpoint = Invoices;
    /// Get all invoices
    async fn get_all(
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<Vec<Invoices>, NaApiError> {
        match args {
            EndpointGetArgs::NoArgs => {
                let data = na_client.get_data("account/invoices").await?;
                // println!("Data: {data}");
                let voices: Vec<Invoices> =
                    serde_json::from_value(data).unwrap();
                Ok(voices)
            }
            _ => {
                Err(NaApiError::UnknownError("No arguments allowed".to_owned()))
            }
        }
    }
}
