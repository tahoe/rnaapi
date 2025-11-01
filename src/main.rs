//! Example rust app using the NetActuate API rust library
//!
//! This app is mainly for testing and is just an example
//!
//! # Usage
//!
//! There are two forms of output, all server info or a single server's info
//!
//! ## All servers info
//! `rnaapi`
//!
//! ## A single servers info
//! `rnaapi -m <mbpkgid>`
//!
//! That's it.
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

    // Defaults
    let mut mbpkgid: u32 = 0;

    // parse our args into args
    let args = SimpleArgs::parse();

    if args.mbpkgid >= 1 {
        mbpkgid = args.mbpkgid;
    }

    // playing with new constructor for client
    let na_client = NaClient::new(API_KEY.to_owned(), API_ADDRESS.to_owned()).await;

    if mbpkgid > 0 {
        // print basic server info
        let srv_result = na_client.get_server(mbpkgid).await?;
        let srv = srv_result;
        println!(
            "Package: {}, fqdn: {}, mbpkgid: {}",
            srv.domu_package, srv.fqdn, srv.mbpkgid
        );

        println!();
        // print jobs
        let jobs_result = na_client.get_jobs(mbpkgid).await?;
        let jobs = jobs_result;
        for job in jobs {
            println!(
                "Inserted: {}, Status: {}, command: {}",
                job.ts_insert, job.status, job.command
            );
        }

        println!();
        // print IPv4 Addresses
        let ipv4_result = na_client.get_ipv4(mbpkgid).await?;
        let ipv4s = ipv4_result;
        for ipv4 in ipv4s {
            println!(
                "Reverse: {}, IP: {}, Gateway: {}",
                ipv4.reverse, ipv4.ip, ipv4.gateway
            );
        }

        println!();
        // print IPv6 Addresses
        let ipv6_result = na_client.get_ipv6(mbpkgid).await?;
        let ipv6s = ipv6_result;
        for ipv6 in ipv6s {
            println!(
                "Reverse: {}, IP: {}, Gateway: {}",
                ipv6.reverse, ipv6.ip, ipv6.gateway
            );
        }

        println!();
        // print server status, very unverbose
        let status_result = na_client.get_status(mbpkgid).await?;
        let stat = status_result;
        println!("Status: {}", stat.status);
    } else {
        let srvrs_result = na_client.get_servers().await?;
        let srvrs = srvrs_result;
        for srvr in srvrs {
            println!("fqdn: {}, mbpkgid: {}", srvr.fqdn, srvr.mbpkgid);
        }

        println!();
        // list locations
        let locs_result = na_client.get_locations().await?;
        let locs = locs_result;
        for loc in locs {
            println!("Name: {}, Continent: {}", loc.name, loc.continent);
        }

        println!();
        // list packages
        let pkgs_result = na_client.get_packages().await?;
        let pkgs = pkgs_result;
        for pkg in pkgs {
            println!("Name: {}, Continent: {}", pkg.name, pkg.city);
        }

        println!();
        // list images
        let imgs_result = na_client.get_images().await?;
        let imgs = imgs_result;
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
