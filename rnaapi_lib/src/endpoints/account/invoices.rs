// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::EndpointGetAll;

/// Account invoices.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "account/invoices")]
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
