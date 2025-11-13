//! Example rust app using the NetActuate API rust library
//!
//! This app is mainly for testing and is just an example
//!
//! # Usage
//! ### Set ENVs
//!
//! ```bash
//! export API_KEY='<your api key>'
//! export API_ADDRESS='https://vapi2.netactuate.com/api/cloud'
//! ```
//!
//! ### Install example client
//! ```rust
//! cargo install rnaapi
//! ```
//!
//! ### There are two forms of output, all server info or a single server's info
//!
//! #### All servers info
//! `rnaapi`
//!
//! ## A single servers info
//! `rnaapi -m <mbpkgid>`
//!
//! That's it.
//!
use anyhow::Result;
use clap::Parser;
use rnaapi::config::{API_ADDRESS, API_KEY};
use rnaapi::NaClient;
use serde::Serialize;
use serde_json::Value;
use std::env;
use std::fmt::format;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    //! Test/Example "main" function, right now it just takes
    //! one argument, `-m <mbpkgid>` if not given, returns all the servers you own

    // Defaults
    let mut mbpkgid: u32 = 0;
    let mut zoneid: u32 = 0;

    // parse our args into args
    let args = SimpleArgs::parse();

    if args.mbpkgid >= 1 {
        mbpkgid = args.mbpkgid;
    }

    if args.zoneid >= 1 {
        zoneid = args.zoneid;
    }

    // playing with new constructor for client
    let na_client = NaClient::new(API_KEY.to_owned(), API_ADDRESS.to_owned()).await;

    if mbpkgid > 0 {
        // print basic server info
        let srv = na_client.get_server(mbpkgid).await?;
        println!(
            "Package: {}, fqdn: {}, mbpkgid: {}",
            srv.domu_package, srv.fqdn, srv.mbpkgid
        );

        println!();
        // print jobs
        let jobs = na_client.get_jobs(mbpkgid).await?;
        for job in jobs {
            println!(
                "Inserted: {}, Status: {}, command: {}",
                job.ts_insert, job.status, job.command
            );
        }

        println!();
        // print IPv4 Addresses
        let ipv4s = na_client.get_ipv4(mbpkgid).await?;
        for ipv4 in ipv4s {
            println!(
                "Reverse: {}, IP: {}, Gateway: {}",
                ipv4.reverse, ipv4.ip, ipv4.gateway
            );
        }

        println!();
        // print IPv6 Addresses
        let ipv6s = na_client.get_ipv6(mbpkgid).await?;
        for ipv6 in ipv6s {
            println!(
                "Reverse: {}, IP: {}, Gateway: {}",
                ipv6.reverse, ipv6.ip, ipv6.gateway
            );
        }

        println!();
        // print server status, very unverbose
        let stat = na_client.get_status(mbpkgid).await?;
        println!("Status: {}", stat.status);

        println!();
        // print some ssh keys
        let ssh_keys = na_client.get_ssh_keys().await?;
        for sshkey in ssh_keys {
            println!("Key: {}, Created At: {}", sshkey.name, sshkey.created_at);
        }

        println!();
        // print some account deets
        let deets = na_client.get_acct_details().await?;
        println!(
            "FullName: {}, Address: {}, {} {} {}",
            deets.fullname, deets.address1, deets.city, deets.state, deets.postcode
        );

        println!();
        // print some account deets
        let voices = na_client.get_acct_invoices().await?;
        for voice in voices {
            println!(
                "ID: {}, Paid On: {}, Status: {}",
                voice.id, voice.datepaid, voice.status
            );
        }
    } else if zoneid > 0 {
        println!();
        // // print out the zone name
        let zone = na_client.get_zone(zoneid).await?;
        println!("Zone: {}", zone.name);

        // print out the SOA for the zone
        let soa = zone.soa.unwrap();
        println!("SOA: {}", soa.primary);

        // print out the first record
        let recs = zone.records.unwrap();
        println!("1st Record: {}", recs[0].name);

        // print out the first NS record
        let nsrecs = zone.ns.unwrap();
        println!("1st NS: {}", nsrecs[0])
    } else {
        let srvrs = na_client.get_servers().await?;
        for srvr in srvrs {
            println!("fqdn: {}, mbpkgid: {}", srvr.fqdn, srvr.mbpkgid);
        }

        println!();
        // list locations
        let locs = na_client.get_locations().await?;
        for loc in locs {
            println!("Name: {}, Continent: {}", loc.name, loc.continent);
        }

        println!();
        // list packages
        let pkgs = na_client.get_packages().await?;
        for pkg in pkgs {
            println!("Name: {}, Continent: {}", pkg.name, pkg.city);
        }

        println!();
        // list images
        let imgs = na_client.get_images().await?;
        for img in imgs {
            println!(
                "ID: {}, Size: {}, Name: {}",
                img.id,
                img.size.unwrap_or("null".to_owned()),
                img.os.unwrap_or("null".to_owned())
            );
        }

        println!();
        // list dns zones
        let zones = na_client.get_zones().await?;
        for zone in zones {
            println!(
                "ID: {}, Size: {}, Name: {}",
                zone.id, zone.name, zone.zone_type
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

    // -z argument for picking a dns zone
    #[arg(short, long, conflicts_with = "mbpkgid", default_value_t = 0)]
    zoneid: u32,
}
