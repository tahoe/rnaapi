//! Async client for the NetActuate API.
//!
//! `API_KEY` is required. `API_ADDRESS` is optional and defaults to
//! `https://vapi2.netactuate.com/api/`. The CLI lives in the `rnacli` crate.
//!
//! ```no_run
//! use anyhow::Result;
//! use rnaapi::config::Settings;
//! use rnaapi::endpoints::Server;
//! use rnaapi::NaClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let settings = Settings::new()?;
//!     let client = NaClient::new(settings.api_key, settings.api_url).await?;
//!     let servers = Server::list(&client).await?;
//!     for server in servers {
//!         println!("fqdn: {}, mbpkgid: {}", server.fqdn, server.mbpkgid);
//!     }
//!     Ok(())
//! }
//! ```
//!
// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use errors::NaApiError;
use reqwest::ClientBuilder;
use reqwest_hickory_resolver::HickoryResolver;
use serde::Serialize;
use serde_json::{Map, Value};
use std::sync::Arc;
use std::time::Duration;

pub mod config;
pub mod endpoints;
pub mod errors;
pub use rnaapi_derive::{EndpointGetAll, EndpointGetOne, EndpointWrite};

const HTTP_TIMEOUT: Duration = Duration::from_secs(30);

pub struct NaClient {
    pub address: String,
    pub api_key: String,
    pub http_client: reqwest::Client,
}

impl NaClient {
    /// Build a client that resolves DNS with Hickory and times out after 30 seconds.
    pub async fn new(
        api_key: String, address: String,
    ) -> Result<NaClient, NaApiError> {
        let http_client = ClientBuilder::new()
            .dns_resolver(Arc::new(HickoryResolver::default()))
            .timeout(HTTP_TIMEOUT)
            .user_agent(concat!("rnaapi/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|err| {
                NaApiError::Request(format!(
                    "failed to build HTTP client: {err}"
                ))
            })?;
        Ok(Self {
            api_key,
            address,
            http_client,
        })
    }

    async fn get(&self, path: &str) -> Result<Value, NaApiError> {
        let url = build_url(&self.address, path, &self.api_key)?;
        let response =
            self.http_client.get(url).send().await.map_err(|err| {
                NaApiError::Request(redact(
                    &format!("request failed: {}", err.without_url()),
                    &self.api_key,
                ))
            })?;
        let status = response.status();
        let body = response.text().await.map_err(|err| {
            NaApiError::Request(redact(
                &format!("reading response failed: {}", err.without_url()),
                &self.api_key,
            ))
        })?;
        if body.trim().is_empty() {
            return Err(NaApiError::Api(format!(
                "empty response (HTTP {status})"
            )));
        }
        let result_json: Value =
            serde_json::from_str(&body).map_err(|err| {
                NaApiError::Deserialize(redact(
                    &format!("HTTP {status}: {err}; body: {body}"),
                    &self.api_key,
                ))
            })?;
        if !status.is_success() {
            return Err(classify_api_error(
                status,
                &result_json,
                &self.api_key,
            ));
        }
        Ok(result_json)
    }

    /// Send a write request and return the JSON `data` field.
    ///
    /// A successful response with no `data` field yields `Null`, which is what
    /// delete and other empty writes return.
    pub(crate) async fn send_data(
        &self, method: &str, path: &str, content_type: Option<&str>,
        body: Option<Vec<u8>>,
    ) -> Result<Value, NaApiError> {
        let url = build_url(&self.address, path, &self.api_key)?;
        let http_method = reqwest::Method::from_bytes(method.as_bytes())
            .map_err(|err| {
                NaApiError::Request(format!("invalid method `{method}`: {err}"))
            })?;
        let mut request = self.http_client.request(http_method, url);
        if let Some(content_type) = content_type {
            request =
                request.header(reqwest::header::CONTENT_TYPE, content_type);
        }
        if let Some(body) = body {
            request = request.body(body);
        }
        let response = request.send().await.map_err(|err| {
            NaApiError::Request(redact(
                &format!("request failed: {}", err.without_url()),
                &self.api_key,
            ))
        })?;
        let status = response.status();
        let text = response.text().await.map_err(|err| {
            NaApiError::Request(redact(
                &format!("reading response failed: {}", err.without_url()),
                &self.api_key,
            ))
        })?;
        let result_json = if text.trim().is_empty() {
            if status.is_success() {
                Value::Object(Map::new())
            } else {
                return Err(NaApiError::Api(format!(
                    "empty response (HTTP {status})"
                )));
            }
        } else {
            serde_json::from_str(&text).map_err(|err| {
                NaApiError::Deserialize(redact(
                    &format!("HTTP {status}: {err}; body: {text}"),
                    &self.api_key,
                ))
            })?
        };
        if !status.is_success() {
            return Err(classify_api_error(
                status,
                &result_json,
                &self.api_key,
            ));
        }
        Ok(result_json.get("data").cloned().unwrap_or(Value::Null))
    }

    /// Fetch `path` and return the JSON `data` field.
    ///
    /// The API key is sent as a query parameter and is stripped from error text.
    pub async fn get_data(&self, path: &str) -> Result<Value, NaApiError> {
        let result = self.get(path).await?;
        match result.get("data") {
            Some(inner) => Ok(inner.clone()),
            None => Err(classify_api_error(
                reqwest::StatusCode::OK,
                &result,
                &self.api_key,
            )),
        }
    }
}

pub(crate) fn form_body<T: Serialize>(body: &T) -> Result<Vec<u8>, NaApiError> {
    let value = serde_json::to_value(body)
        .map_err(|err| NaApiError::Deserialize(format!("form body: {err}")))?;
    Ok(form_encode(&value)?.into_bytes())
}

fn form_encode(value: &Value) -> Result<String, NaApiError> {
    let Some(object) = value.as_object() else {
        return Err(NaApiError::Deserialize(
            "form body must be a JSON object".into(),
        ));
    };
    let mut url =
        reqwest::Url::parse("http://form.invalid/").expect("static url");
    for (key, item) in object {
        append_form_value(&mut url, key, item);
    }
    Ok(url.query().unwrap_or("").to_string())
}

fn append_form_value(url: &mut reqwest::Url, key: &str, item: &Value) {
    match item {
        Value::Null => {}
        Value::Bool(flag) => {
            url.query_pairs_mut()
                .append_pair(key, if *flag { "1" } else { "0" });
        }
        Value::Number(number) => {
            url.query_pairs_mut().append_pair(key, &number.to_string());
        }
        Value::String(text) => {
            url.query_pairs_mut().append_pair(key, text);
        }
        Value::Array(items) if items.is_empty() => {
            url.query_pairs_mut().append_pair(&format!("{key}[]"), "");
        }
        Value::Array(items) => {
            let item_key = format!("{key}[]");
            for item in items {
                append_form_value(url, &item_key, item);
            }
        }
        Value::Object(_) => {
            url.query_pairs_mut().append_pair(key, &item.to_string());
        }
    }
}

fn join_endpoint(address: &str, path: &str) -> String {
    match (address.ends_with('/'), path.starts_with('/')) {
        (true, true) => format!("{}{}", address, &path[1..]),
        (false, false) => format!("{address}/{path}"),
        _ => format!("{address}{path}"),
    }
}

fn build_url(
    address: &str, path: &str, api_key: &str,
) -> Result<reqwest::Url, NaApiError> {
    let base = join_endpoint(address, path);
    let mut url = reqwest::Url::parse(&base).map_err(|err| {
        NaApiError::Request(format!("invalid url `{base}`: {err}"))
    })?;
    url.query_pairs_mut().append_pair("key", api_key);
    Ok(url)
}

fn redact(text: &str, secret: &str) -> String {
    if secret.is_empty() {
        return text.to_string();
    }
    let redacted = text.replace(secret, "[REDACTED]");
    let encoded = percent_encode_component(secret);
    if encoded != secret {
        redacted.replace(&encoded, "[REDACTED]")
    } else {
        redacted
    }
}

fn percent_encode_component(value: &str) -> String {
    let mut url =
        reqwest::Url::parse("http://redact.invalid/").expect("static url");
    url.query_pairs_mut().append_pair("k", value);
    url.query()
        .unwrap_or("")
        .trim_start_matches("k=")
        .to_string()
}

fn json_text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

fn classify_api_error(
    status: reqwest::StatusCode, body: &Value, secret: &str,
) -> NaApiError {
    let code = body
        .get("code")
        .map(json_text)
        .unwrap_or_else(|| status.as_u16().to_string());
    let message = body.get("message").map(json_text).unwrap_or_else(|| {
        if body.get("data").is_none() {
            "response did not include data".to_string()
        } else {
            status.to_string()
        }
    });
    let summary = redact(&body.to_string(), secret);
    let blob = redact(&format!("{code}: {message} {summary}"), secret)
        .to_ascii_lowercase();
    if status == reqwest::StatusCode::UNAUTHORIZED
        || blob.contains("api key")
        || blob.contains("invalid key")
        || blob.contains("key invalid")
    {
        NaApiError::APIKeyInvalid(summary)
    } else if blob.contains("ip")
        && (blob.contains("not allowed")
            || blob.contains("denied")
            || blob.contains("forbidden"))
    {
        NaApiError::IpNotAllowed(summary)
    } else {
        NaApiError::Api(summary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn url_appends_an_encoded_key() {
        let url = build_url(
            "https://example.com/api/",
            "cloud/server?mbpkgid=5",
            "a&b",
        )
        .unwrap();
        let key = url
            .query_pairs()
            .find(|(name, _)| name == "key")
            .map(|(_, value)| value.into_owned());
        assert_eq!(key.as_deref(), Some("a&b"));
        assert!(url.as_str().contains("mbpkgid=5"));
        assert!(url.as_str().contains("key=a%26b"));
        assert!(!url.as_str().contains("key=a&b"));
    }

    #[test]
    fn form_encode_skips_null_and_uses_one_or_zero() {
        let body = serde_json::json!({
            "name": "edge",
            "enabled": true,
            "draft": false,
            "skip": null,
            "tag_list": ["a", "b"]
        });
        let encoded = form_encode(&body).unwrap();
        assert!(encoded.contains("name=edge"), "{encoded}");
        assert!(encoded.contains("enabled=1"), "{encoded}");
        assert!(encoded.contains("draft=0"), "{encoded}");
        assert!(!encoded.contains("skip"), "{encoded}");
        assert!(encoded.contains("tag_list%5B%5D=a"), "{encoded}");
        assert!(encoded.contains("tag_list%5B%5D=b"), "{encoded}");
    }

    #[test]
    fn join_inserts_a_single_slash() {
        assert_eq!(
            join_endpoint("https://example.com/api", "account/details"),
            "https://example.com/api/account/details"
        );
        assert_eq!(
            join_endpoint("https://example.com/api/", "/account/details"),
            "https://example.com/api/account/details"
        );
    }

    #[test]
    fn redact_hides_the_raw_and_encoded_key() {
        let text = "failed key=a%26b and also a&b";
        let redacted = redact(text, "a&b");
        assert!(!redacted.contains("a&b"));
        assert!(!redacted.contains("a%26b"));
        assert!(redacted.contains("[REDACTED]"));
    }

    #[test]
    fn other_api_errors_are_not_reported_as_an_invalid_key() {
        let body =
            serde_json::json!({"code": 404, "message": "server not found"});
        let err =
            classify_api_error(reqwest::StatusCode::NOT_FOUND, &body, "secret");
        assert!(matches!(err, NaApiError::Api(_)));
        assert!(!err.to_string().contains("secret"));
    }

    #[test]
    fn unauthorized_is_an_invalid_key() {
        let body = serde_json::json!({"code": 401, "message": "nope"});
        let err = classify_api_error(
            reqwest::StatusCode::UNAUTHORIZED,
            &body,
            "secret",
        );
        assert!(matches!(err, NaApiError::APIKeyInvalid(_)));
    }

    #[tokio::test]
    async fn get_data_returns_the_data_field() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/account/details"))
            .and(query_param("key", "secret"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({
                    "data": {"ok": true}
                }),
            ))
            .mount(&server)
            .await;

        let client =
            NaClient::new("secret".into(), server.uri()).await.unwrap();
        let data = client.get_data("account/details").await.unwrap();
        assert_eq!(data["ok"], true);
    }

    #[tokio::test]
    async fn error_bodies_do_not_include_the_api_key() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/account/details"))
            .respond_with(ResponseTemplate::new(400).set_body_json(
                serde_json::json!({
                    "code": 12,
                    "message": "rejected secret-value"
                }),
            ))
            .mount(&server)
            .await;

        let client = NaClient::new("secret-value".into(), server.uri())
            .await
            .unwrap();
        let err = client.get_data("account/details").await.unwrap_err();
        let text = err.to_string();
        assert!(matches!(err, NaApiError::Api(_)));
        assert!(text.contains("[REDACTED]"));
        assert!(!text.contains("secret-value"), "{text}");
    }

    #[tokio::test]
    async fn connection_errors_do_not_include_the_api_key() {
        let client = NaClient::new(
            "super-secret-key".into(),
            "http://127.0.0.1:1/".into(),
        )
        .await
        .unwrap();
        let err = client.get_data("account/details").await.unwrap_err();
        let text = err.to_string();
        assert!(matches!(err, NaApiError::Request(_)), "{text}");
        assert!(!text.contains("super-secret-key"), "{text}");
    }
}
