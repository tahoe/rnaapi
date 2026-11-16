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
    /// Default NetActuate API root. `API_ADDRESS` overrides this.
    pub const API_ADDRESS: &str = "https://vapi2.netactuate.com/api/";

    pub fn new() -> Result<Settings, NaApiError> {
        dotenv().ok();
        Ok(Settings {
            api_key: api_key_from_env()?,
            api_url: api_address_from_env()?,
        })
    }
}

fn api_key_from_env() -> Result<String, NaApiError> {
    match std_env::var("API_KEY") {
        Ok(key) if !key.is_empty() => Ok(key),
        Ok(_) => Err(NaApiError::APIKeyInvalid(
            "API_KEY is set but empty".to_string(),
        )),
        Err(_) => Err(NaApiError::APIKeyInvalid(
            "API_KEY not set in the environment".to_string(),
        )),
    }
}

fn api_address_from_env() -> Result<String, NaApiError> {
    match std_env::var("API_ADDRESS") {
        Ok(address) if !address.is_empty() => Ok(address),
        Ok(_) => Err(NaApiError::Config(
            "API_ADDRESS is set but empty".to_string(),
        )),
        Err(_) => Ok(Settings::API_ADDRESS.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        api_key: Option<String>,
        api_address: Option<String>,
    }

    impl EnvGuard {
        fn set(api_key: Option<&str>, api_address: Option<&str>) -> Self {
            let previous = Self {
                api_key: std_env::var("API_KEY").ok(),
                api_address: std_env::var("API_ADDRESS").ok(),
            };
            set_var("API_KEY", api_key);
            set_var("API_ADDRESS", api_address);
            previous
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            set_var("API_KEY", self.api_key.as_deref());
            set_var("API_ADDRESS", self.api_address.as_deref());
        }
    }

    fn set_var(key: &str, value: Option<&str>) {
        match value {
            Some(value) => unsafe { std_env::set_var(key, value) },
            None => unsafe { std_env::remove_var(key) },
        }
    }

    #[test]
    fn missing_api_key_is_an_error() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guard = EnvGuard::set(None, None);
        let err = Settings::new().unwrap_err();
        assert!(matches!(err, NaApiError::APIKeyInvalid(_)));
    }

    #[test]
    fn empty_api_key_is_an_error() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guard = EnvGuard::set(Some(""), None);
        let err = Settings::new().unwrap_err();
        assert!(matches!(err, NaApiError::APIKeyInvalid(_)));
    }

    #[test]
    fn api_address_defaults_when_unset() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guard = EnvGuard::set(Some("test-key"), None);
        let settings = Settings::new().unwrap();
        assert_eq!(settings.api_key, "test-key");
        assert_eq!(settings.api_url, Settings::API_ADDRESS);
    }

    #[test]
    fn api_address_can_be_overridden() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guard =
            EnvGuard::set(Some("test-key"), Some("https://example.test/api/"));
        let settings = Settings::new().unwrap();
        assert_eq!(settings.api_url, "https://example.test/api/");
    }

    #[test]
    fn empty_api_address_is_an_error() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guard = EnvGuard::set(Some("test-key"), Some(""));
        let err = Settings::new().unwrap_err();
        assert!(matches!(err, NaApiError::Config(_)));
    }
}
