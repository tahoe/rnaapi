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
use rnaapi::endpoints::{Server, ServerData, SrvJob, SrvJobsData};
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
        // print basic server info
        let srv_result = na_client.get_server(mbpkgid).await;
        let srv = srv_result.unwrap();
        println!(
            "Package: {}, fqdn: {}, mbpkgid: {}",
            srv.domu_package, srv.fqdn, srv.mbpkgid
        );

        println!();
        // print jobs
        let jobs_result = na_client.get_jobs(mbpkgid).await;
        let jobs = jobs_result.unwrap();
        for job in jobs {
            println!(
                "Inserted: {}, Status: {}, command: {}",
                job.ts_insert, job.status, job.command
            );
        }

        println!();
        // print IPv4 Addresses
        let ipv4_result = na_client.get_ipv4(mbpkgid).await;
        let ipv4_addresses = ipv4_result.unwrap();
        for ipv4 in ipv4_addresses {
            println!(
                "Reverse: {}, IP: {}, Gateway: {}",
                ipv4.reverse, ipv4.ip, ipv4.gateway
            );
        }

        println!();
        // print IPv6 Addresses
        let ipv6_result = na_client.get_ipv6(mbpkgid).await;
        let ipv6_addresses = ipv6_result.unwrap();
        for ipv6 in ipv6_addresses {
            println!(
                "Reverse: {}, IP: {}, Gateway: {}",
                ipv6.reverse, ipv6.ip, ipv6.gateway
            );
        }

        println!();
        // print server status, very unverbose
        let status_result = na_client.get_status(mbpkgid).await;
        let status = status_result.unwrap();
        println!("Status: {}", status.status);
    } else {
        let srvrs_result = na_client.get_servers().await;
        let srvrs = srvrs_result.unwrap();
        for srvr in srvrs {
            println!("fqdn: {}, mbpkgid: {}", srvr.fqdn, srvr.mbpkgid);
        }

        println!();
        // list locations
        let locs_result = na_client.get_locations().await;
        let locs = locs_result.unwrap();
        for loc in locs {
            println!("Name: {}, Continent: {}", loc.name, loc.continent);
        }

        println!();
        // list packages
        let pkgs_result = na_client.get_packages().await;
        let pkgs = pkgs_result.unwrap();
        for pkg in pkgs {
            println!("Name: {}, Continent: {}", pkg.name, pkg.city);
        }

        println!();
        // list images
        let imgs_result = na_client.get_images().await;
        let imgs = imgs_result.unwrap();
        for img in imgs {
            println!(
                "ID: {}, Size: {}, Name: {}",
                img.id,
                img.size.unwrap_or("null".to_owned()),
                img.os.unwrap_or("null".to_owned())
            );
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
