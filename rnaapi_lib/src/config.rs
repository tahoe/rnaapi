// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use dotenvy::dotenv;
use serde::Deserialize;
use std::env as std_env;

use crate::errors::NaApiError;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api_key: String,
    pub api_url: String,
}

impl Settings {
    // manually set api address
    pub const API_ADDRESS: &str = "https://vapi2.netactuate.com/api/";

    pub fn new() -> Result<Settings, NaApiError> {
        Ok(Settings {
            api_key: set_key()?,
            api_url: Self::API_ADDRESS.to_string(),
        })
    }
}

fn set_key() -> Result<String, NaApiError> {
    dotenv().ok();
    let apikey = match std_env::var("API_KEY") {
        Ok(key) => {
            if key.is_empty() {
                return Err(NaApiError::APIKeyInvalid(
                    "API_KEY is set but empty!".to_string(),
                ));
            }
            Ok(key)
        }
        Err(_) => {
            return Err(NaApiError::APIKeyInvalid(
                "API_KEY not set in ENV".to_string(),
            ));
        }
    };
    Ok(apikey)?
}
