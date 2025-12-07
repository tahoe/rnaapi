// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
// use crate::custom_datetime_format_seconds;
use std::fmt::format;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::NaApiError;
use crate::NaClient;

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
impl NaClient {
    /// Get all invoices
    pub async fn get_acct_invoices(&self) -> Result<Vec<Invoices>, NaApiError> {
        let data = self.get_data("account/invoices").await?;
        // println!("Data: {data}");
        let voices: Vec<Invoices> = serde_json::from_value(data).unwrap();
        Ok(voices)
    }
}
