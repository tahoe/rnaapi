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
use rnaapi::endpoints::{Server, ServerData, ServersData};
use rnaapi::NaClient;
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
    let mut mbpkgid: u32 = 0;

    // parse our args into args
    let args = SimpleArgs::parse();

    if args.mbpkgid >= 1 {
        mbpkgid = args.mbpkgid;
    }

    // playing with new constructor for client
    let na_client = NaClient::new(API_KEY.to_owned(), API_ADDRESS.to_owned()).await;

    // TODO: Create more types and forgoe creating the new functions
    // since we are only worrying about readonly mode...
    // TODO: Star re-working this main.rs as an example TUI app using ratatui
    // At that point we won't take any options except maybe like a starting view
    // for instance -l for starting with listing locations or servers or whatever...
    if mbpkgid > 0 {
        let api_result = na_client.get_server(mbpkgid).await;
        let api_result = api_result.unwrap();
        println!(
            "fqdn: {}, mbpkgid: {}",
            api_result.data.fqdn, api_result.data.mbpkgid
        );
    } else {
        let api_result = na_client.get_servers().await;
        let api_result = api_result.unwrap();
        for srvr in api_result.data {
            println!("fqdn: {}, mbpkgid: {}", srvr.fqdn, srvr.mbpkgid);
        }
    }
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
    mbpkgid: u32,
}
