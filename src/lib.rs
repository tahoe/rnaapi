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
#![allow(unused)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use errors::NaApiError;
use reqwest::ClientBuilder;
use reqwest_hickory_resolver::HickoryResolver;
use serde_json::Value;
use std::sync::Arc;
use std::{error::Error, process};

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
    async fn make_request(&self, path: &str) -> Result<Value, NaApiError> {
        let mut api_key = self.api_key.clone();
        if path.contains("?") {
            api_key = "&key=".to_owned() + &self.api_key;
        } else {
            api_key = "?key=".to_owned() + &self.api_key;
        }
        // println!("{}{}{}", self.address, path, api_key);
        let result = self
            .http_client
            .get(format!("{}{}{}", self.address, path, api_key))
            .send()
            .await
            .map_err(|e| {
                NaApiError::UnknownError(format!("Failed to finish request with error: {e}"))
            })?;
        let result_json = result.json::<Value>().await.map_err(|e| {
            NaApiError::UnknownError(format!("Failed to finish request with error: {e}"))
        })?;
        Ok(result_json)
    }

    /// Call the make_request and parse the results
    /// We want to get the "data" attribute from the response for the calling
    /// endpoint. Exit with error message if "data" is not present
    /// This is shitty but it is safe enough so far as I can tell at this point
    pub async fn get_data(&self, path: &str) -> Result<Value, NaApiError> {
        // Get the response from make_request method
        let result = self
            .make_request(path)
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

// // Define a module to hold the custom serialization/deserialization logic.
// // This is kind of BS to have to do...
// mod custom_datetime_format_seconds {
//     use chrono::{NaiveDateTime, ParseResult};
//     use serde::{self, Deserialize, Deserializer, Serializer};
//
//     const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
//
//     // The signature for a `serialize_with` function must take the value being
//     // serialized and a serializer.
//     pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = format!("{}", date.format(FORMAT));
//         serializer.serialize_str(&s)
//     }
//
//     // The signature for a `deserialize_with` function must take a deserializer.
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
//     }
// }
//
// // Define a module to hold the custom serialization/deserialization logic.
// // This is kind of BS to have to do...
// mod custom_datetime_format_microseconds {
//     use chrono::{NaiveDateTime, ParseResult};
//     use serde::{self, Deserialize, Deserializer, Serializer};
//
//     const FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";
//
//     // The signature for a `serialize_with` function must take the value being
//     // serialized and a serializer.
//     pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = format!("{}", date.format(FORMAT));
//         serializer.serialize_str(&s)
//     }
//
//     // The signature for a `deserialize_with` function must take a deserializer.
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
//     }
// }
//
/*
// Package gona provides a simple golang interface to the NetActuate
// Rest API at https://vapi2.netactuate.com/

// Version, BaseEndpoint, ContentType constants
const (
    Version      = "0.2.0"
    BaseEndpoint = "https://vapi2.netactuate.com/api/"
    ContentType  = "application/json"
)

// Client is the main object (struct) to which we attach most
// methods/functions.
// It has the following fields:
// (client, userAgent, endPoint, apiKey)
type Client struct {
    client    *http.Client
    userAgent string
    endPoint  *url.URL
    apiKey    string
}

// GetKeyFromEnv is a simple function to grab the value for
// "NA_API_KEY" from the environment
func GetKeyFromEnv() string {
    return os.Getenv("NA_API_KEY")
}

// NewClientCustom is the main entrypoint for instantiating a Client struct.
// It takes your API Key as it's sole argument
// and returns the Client struct ready to talk to the API
func NewClientCustom(apikey string, apiurl string) *Client {
    useragent := "gona/" + Version
    transport := &http.Transport{
        TLSNextProto: make(
            map[string]func(string, *tls.Conn) http.RoundTripper,
        ),
    }
    client := http.DefaultClient
    client.Transport = transport
    endpoint, _ := url.Parse(apiurl)

    return &Client{
        userAgent: useragent,
        client:    client,
        endPoint:  endpoint,
        apiKey:    apikey,
    }
}

// NewClient takes an apikey and calls NewClientCustom with the hardcoded
// BaseEndpoint constant API URL
func NewClient(apikey string) *Client {
    return NewClientCustom(apikey, BaseEndpoint)
}

// apiKeyPath is just a short internal function for appending the key to the url
func apiKeyPath(path, apiKey string) string {
    if strings.Contains(path, "?") {
        return path + "&key=" + apiKey
    }
    return path + "?key=" + apiKey
}

func (c *Client) debugLog(format string, v ...any) {
    if os.Getenv("NA_API_DEBUG") == "" {
        return
    }
    log.Printf("[DEBUG] "+format, v...)
}

// get internal method on Client struct for providing the HTTP GET call
func (c *Client) get(path string, data interface{}) error {
    req, err := c.newRequest("GET", path, nil)
    if err != nil {
        return err
    }
    return c.do(req, data)
}

// post internal method on Client struct for providing the HTTP POST call
func (c *Client) post(path string, values []byte, data interface{}) error {
    c.debugLog("POST data for %s: %s", path, string(values))

    req, err := c.newRequest("POST", path, bytes.NewBuffer(values))
    if err != nil {
        return err
    }

    req.Header.Set("Content-Type", "application/x-www-form-urlencoded")

    return c.do(req, data)
}

// delete internal method on Client struct for providing the HTTP DELETE call
func (c *Client) delete(path string, values url.Values, data interface{}) error {
    req, err := c.newRequest("DELETE", path, nil)
    if err != nil {
        return err
    }
    return c.do(req, data)
}

// Two functions (newRequest, do) below are used by the http method name functions above
// newRequest internal method on Client struct to be wrapped inside the above http method
// named functions for doing the actual work of the get/post/put/patch/delete methods
func (c *Client) newRequest(method string, path string, body io.Reader) (*http.Request, error) {
    relPath, err := url.Parse(apiKeyPath(path, c.apiKey))

    if err != nil {
        return nil, err

    }

    url := c.endPoint.ResolveReference(relPath)

    req, err := http.NewRequest(method, url.String(), body)
    if err != nil {
        return nil, err

    }

    req.Header.Add("User-Agent", c.userAgent)
    req.Header.Add("Accept", ContentType)

    c.debugLog("making a %s request to %s", method, url)
    return req, nil
}

// apiResponse is a message returned by the API that is used both for successful
// responses and for some error responses.
type apiResponse struct {
    Result  string                 `json:"result"`
    Message string                 `json:"message"`
    Data    interface{}            `json:"data"`
    Code    int                    `json:"code"`
    Fields  map[string]interface{} `json:"fields"`
}

// do internal method on Client struct for making the HTTP calls
func (c *Client) do(req *http.Request, data interface{}) error {
    resp, err := c.client.Do(req)
    if err != nil {
        return err
    }
    defer resp.Body.Close()

    body, err := io.ReadAll(resp.Body)
    if err != nil {
        return err
    }
    c.debugLog("got a response: %s", string(body))

    r := &apiResponse{
        Data: data,
    }
    if err := json.Unmarshal(body, r); err != nil {
        return fmt.Errorf("could not unmarshal response %q: %w", string(body), err)
    }

  // Error Handling - This currently ignores invalid mbpkdgid errors to enable the Terraform Provider
    if (resp.StatusCode == 422 || r.Code == 422) && (r.Fields != nil && r.Fields["mbpkgid"] == nil) {
        fieldStr := ""
        for key, value := range r.Fields {
            fieldStr = fieldStr + fmt.Sprintf("%s: %v, ", key, value)
        }
        return fmt.Errorf("got an ERROR response on %s %s: code %d / %d, response: %s / %s", req.Method, req.URL, resp.StatusCode, r.Code, r.Message, fieldStr)
    }

    if (resp.StatusCode != http.StatusOK && resp.StatusCode != 422) || (r.Code != http.StatusOK && r.Code != 422) {
        return fmt.Errorf("got an error response on %s %s: code %d / %d, response: %s / %+v", req.Method, req.URL, resp.StatusCode, r.Code, r.Message, r.Data)
    }

    return nil
}
*/
