//! Rust library for talking to the NetActuate API
//!
//! This library provides the methods for establishing a connection
//! and for retrieving data from as many endpoints as I feel like
//! writing support for.
//!
//! It also will include an example app written in some CLI framework
//! that will be interactive to some extent, maybe...
//!
//! # Usage
//!
//! First, let me finish this thang, but you'll need to do a `cargo add rnaapi`
//! to get started. Right now, all you can do is `cargo install rnaapi`
//! and use the example application with it's very limited functionality...
//!
//! Help output:
//! ```
//! No clue yet
//! ```
//!
use clap::Parser;
use rnaapi::config::{API_ADDRESS, API_KEY};
use rnaapi::Application;
use serde::Serialize;
use serde_json::{Result, Value};
use std::env;
use std::fmt::format;
use std::sync::Arc;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    //! Test/Example "main" function, right now it just takes
    //! one argument, `-m <mbpkgid>` if not given, returns all the servers you own
    //!
    //! What makes this whole thing really annoying is that the "list" of servers,
    //! retrieved at the endpoint /servers, returns a list of servers that are not
    //! quite the same as the individual servers returned by /server/&mbpkgid=id
    //!
    //! So it's going to be fun figuring out how to represent them in Rust Structs

    // Defaults
    let mut mbpkgid: i32 = 0;
    let mut servers: &str = "servers";

    // parse our args into args
    let args = SimpleArgs::parse();

    if args.mbpkgid >= 1 {
        mbpkgid = args.mbpkgid;
        servers = "server";
    }

    // playing with new constructor for client
    let test_client = Application::new(API_ADDRESS.to_owned()).await;

    // let test_client = Arc::new(Client::builder().dns_resolver(resolver).build());
    let api_result = test_client
        .http_client
        .get(format!(
            "{}/{servers}/?key={}&mbpkgid={mbpkgid}",
            test_client.address,
            API_KEY.to_owned()
        ))
        .send()
        .await?;
    let res_str = api_result.text().await?;
    let res_json: Value = serde_json::from_str(&res_str).expect("blah");
    println!("{res_json:#}");
    Ok(())
}

///
/// This is the SimpleArgs struct
///
#[derive(Parser, Debug)]
#[command(version, about)]
struct SimpleArgs {
    // -m argument for picking an mbpkgid
    #[arg(short, long, default_value_t = 0)]
    mbpkgid: i32,
}
