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
// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use anyhow::Result;
use chrono::NaiveDate;
use clap::CommandFactory;
use clap::{Parser, Subcommand};
use clap_complete::{Shell, generate};
use rnaapi::NaClient;
use rnaapi::config::Settings;
use rnaapi::endpoints;
use rnaapi::{EndpointGetAll, EndpointGetArgs, EndpointGetOne};

#[tokio::main]
async fn main() -> Result<()> {
    //! Test/Example "main" function, right now it just takes
    //! one argument, `-m <mbpkgid>` if not given, returns all the servers you own

    // Get settings from config
    let settings = Settings::new()?;

    // Defaults
    let mut ssh_keyid: u32 = 0;
    let mut display_count: usize = 0;
    let mut loc_mbpkgid: u32 = 0;
    let mut loc_jobid: u32 = 0;
    let mut loc_zoneid: u32 = 0;
    let mut loc_sizes: u32 = 0;
    let mut command: &str = "default";

    // parse our args into args
    let cli = Cli::parse();

    // check cli sub commands
    match &cli.cmd {
        Some(Commands::GenerateCompletions { shell }) => {
            let mut app = Cli::command();
            let appclone = app.clone();
            generate(
                *shell,
                &mut app,
                appclone.get_name().to_string(),
                &mut std::io::stdout(),
            );
        }
        Some(Commands::Get { cmd }) => match cmd {
            GetCommands::Server { mbpkgid } => {
                if *mbpkgid >= 1 {
                    loc_mbpkgid = *mbpkgid;
                    command = "server";
                } else {
                    command = "server";
                }
            }
            GetCommands::Dns { id } => {
                if *id >= 1 {
                    loc_zoneid = *id;
                    command = "dns";
                } else {
                    command = "dns";
                }
            }
            GetCommands::Ssh { id } => {
                if *id >= 1 {
                    ssh_keyid = *id;
                    command = "ssh";
                } else {
                    command = "ssh";
                }
            }
            GetCommands::Job { mbpkgid, jobid } => {
                if *mbpkgid >= 1 {
                    loc_mbpkgid = *mbpkgid;
                    command = "job";
                    if *jobid >= 1 {
                        loc_jobid = *jobid;
                    }
                }
            }
            GetCommands::Ip { mbpkgid } => {
                if *mbpkgid >= 1 {
                    loc_mbpkgid = *mbpkgid;
                    command = "ip";
                }
            }
            GetCommands::Bandwidth { mbpkgid } => {
                if *mbpkgid >= 1 {
                    loc_mbpkgid = *mbpkgid;
                    command = "bandwidth";
                }
            }
            GetCommands::Invoice { count } => {
                display_count = *count;
                command = "invoice";
            }
            GetCommands::Sizes { id } => {
                loc_sizes = *id;
                command = "sizes";
            }
            GetCommands::Location {} => {
                command = "location";
            }
            GetCommands::Image {} => {
                command = "image";
            }
            GetCommands::Account {} => {
                command = "account";
            }
        },
        _ => {}
    }
    // playing with new constructor for client
    // let na_client = NaClient::new(API_KEY.to_owned(), API_ADDRESS.to_owned()).await;
    let na_client = NaClient::new(settings.api_key, settings.api_url).await;

    if command == "server" {
        if loc_mbpkgid > 0 {
            // submit jobs to the tokio async runtime
            // this automatically awaits so no need for .await
            let (server, srvjobs, ipv4s, ipv6s, status, bw_usage) = tokio::join!(
                endpoints::Server::get_one(
                    &na_client,
                    EndpointGetArgs::OneInt(loc_mbpkgid)
                ),
                endpoints::SrvJob::get_all(
                    &na_client,
                    EndpointGetArgs::OneInt(loc_mbpkgid)
                ),
                endpoints::IPv4::get_all(
                    &na_client,
                    EndpointGetArgs::OneInt(loc_mbpkgid)
                ),
                endpoints::IPv6::get_all(
                    &na_client,
                    EndpointGetArgs::OneInt(loc_mbpkgid)
                ),
                endpoints::SrvStatus::get_one(
                    &na_client,
                    EndpointGetArgs::OneInt(loc_mbpkgid)
                ),
                endpoints::MonthlyBw::get_all(
                    &na_client,
                    EndpointGetArgs::OneInt(loc_mbpkgid),
                ),
            );

            // print basic server info
            if let Ok(srv) = server {
                println!(
                    "Package: {}, fqdn: {}, mbpkgid: {}",
                    srv.domu_package, srv.fqdn, srv.mbpkgid
                );
            }

            println!();
            // print the job data
            if let Ok(jobs) = srvjobs {
                for job in jobs {
                    println!(
                        "Inserted: {}, Status: {}, command: {}",
                        job.ts_insert, job.status, job.command
                    );
                }
            }

            println!();
            // print IPv4 Addresses
            if let Ok(ip4s) = ipv4s {
                for ipv4 in ip4s {
                    println!(
                        "Reverse: {}, IP: {}, Gateway: {}",
                        ipv4.reverse, ipv4.ip, ipv4.gateway
                    );
                }
            }

            println!();
            // print IPv6 Addresses
            if let Ok(ip6s) = ipv6s {
                for ipv6 in ip6s {
                    println!(
                        "Reverse: {}, IP: {}, Gateway: {}",
                        ipv6.reverse, ipv6.ip, ipv6.gateway
                    );
                }
            }

            println!();
            // print server status, very unverbose
            if let Ok(stat) = status {
                println!("Status: {}", stat.status);
            }

            // print out bandwidth usage
            println!();
            if let Ok(mut bwusage) = bw_usage {
                bwusage.sort_by_key(|b| {
                    let date_with_day = format!("{}-01", b.date);
                    NaiveDate::parse_from_str(&date_with_day, "%Y-%m-%d")
                        .expect("Failed to parse date")
                });
                for usage in bwusage {
                    println!(
                        "Date: {}, Rx: {}, Tx: {}",
                        usage.date, usage.rx, usage.tx
                    );
                }
            }
        } else {
            let srvrs =
                endpoints::Server::get_all(&na_client, EndpointGetArgs::NoArgs)
                    .await?;

            for srvr in srvrs {
                println!("ID: {}, fqdn: {}", srvr.mbpkgid, srvr.fqdn);
            }
        }
    } else if command == "dns" {
        if loc_zoneid > 0 {
            println!();
            // // print out the zone name
            let zone = endpoints::Zone::get_one(
                &na_client,
                EndpointGetArgs::OneInt(loc_zoneid),
            )
            .await?;
            println!("Zone: {}", zone.name);

            // print out the SOA for the zone
            if let Some(soa) = zone.soa {
                println!("SOA: {}", soa.primary);
            }

            // print out the first record
            if let Some(recs) = zone.records {
                println!("1st Record: {}", recs[0].name);
            }

            // print out the first NS record
            if let Some(nsrecs) = zone.ns {
                println!("1st NS: {}", nsrecs[0].name)
            }
        } else {
            println!();
            // list dns zones
            let zones =
                endpoints::Zone::get_all(&na_client, EndpointGetArgs::NoArgs)
                    .await?;
            for zone in zones {
                println!(
                    "ID: {}, Size: {}, Name: {}",
                    zone.id, zone.name, zone.zone_type
                );
            }
        }
    } else if command == "ssh" {
        if ssh_keyid > 0 {
            let sshkey = endpoints::SSHKeys::get_one(
                &na_client,
                EndpointGetArgs::OneInt(ssh_keyid),
            )
            .await?;
            println!();
            // print some ssh keys
            println!(
                "ID: {}, Key: {}, Fingerprint: {}",
                sshkey.id, sshkey.name, sshkey.fingerprint
            );
        } else {
            let keys = endpoints::SSHKeys::get_all(
                &na_client,
                EndpointGetArgs::NoArgs,
            )
            .await?;
            println!();
            // print some ssh keys
            for sshkey in keys {
                println!(
                    "ID: {}, Key: {}, Fingerprint: {}",
                    sshkey.id, sshkey.name, sshkey.fingerprint
                );
            }
        }
    } else if command == "job" {
        if loc_mbpkgid > 0 && loc_jobid > 0 {
            let job = endpoints::SrvJob::get_one(
                &na_client,
                EndpointGetArgs::TwoInt(loc_mbpkgid, loc_jobid),
            )
            .await?;
            println!();
            // print some ssh keys
            println!(
                "ID: {}, Status: {}, Command: {}",
                job.id, job.status, job.command
            );
        } else if loc_mbpkgid > 0 {
            let jobs = endpoints::SrvJob::get_all(
                &na_client,
                EndpointGetArgs::OneInt(loc_mbpkgid),
            )
            .await?;
            println!();
            // print some ssh keys
            for job in jobs {
                println!(
                    "ID: {}, Status: {}, Command: {}",
                    job.id, job.status, job.command
                );
            }
        }
    } else if command == "ip" {
        if loc_mbpkgid > 0 {
            let (ipv4s, ipv6s) = tokio::join!(
                endpoints::IPv4::get_all(
                    &na_client,
                    EndpointGetArgs::OneInt(loc_mbpkgid)
                ),
                endpoints::IPv6::get_all(
                    &na_client,
                    EndpointGetArgs::OneInt(loc_mbpkgid)
                ),
            );

            // print IPv4 Addresses
            if let Ok(ip4s) = ipv4s {
                for ipv4 in ip4s {
                    println!(
                        "Reverse: {}, IP: {}, Gateway: {}",
                        ipv4.reverse, ipv4.ip, ipv4.gateway
                    );
                }
            }

            // print IPv6 Addresses
            if let Ok(ip6s) = ipv6s {
                for ipv6 in ip6s {
                    println!(
                        "Reverse: {}, IP: {}, Gateway: {}",
                        ipv6.reverse, ipv6.ip, ipv6.gateway
                    );
                }
            }
        } else {
            println!("you need to provide an mbpkgid");
        }
    } else if command == "bandwidth" {
        let mut bw_usage = endpoints::MonthlyBw::get_all(
            &na_client,
            EndpointGetArgs::OneInt(loc_mbpkgid),
        )
        .await?;
        println!();
        bw_usage.sort_by_key(|b| {
            let date_with_day = format!("{}-01", b.date);
            NaiveDate::parse_from_str(&date_with_day, "%Y-%m-%d")
                .expect("Failed to parse date")
        });
        for usage in bw_usage {
            println!(
                "Date: {}, Rx: {}, Tx: {}",
                usage.date, usage.rx, usage.tx
            );
        }
    } else if command == "location" {
        let locs =
            endpoints::Location::get_all(&na_client, EndpointGetArgs::NoArgs)
                .await?;
        println!();
        // list locations
        for loc in locs {
            println!(
                "ID: {}, Name: {}, Continent: {}",
                loc.id, loc.name, loc.continent
            );
        }
    } else if command == "account" {
        let deets =
            endpoints::Details::get_one(&na_client, EndpointGetArgs::NoArgs)
                .await?;
        println!();
        // print acct details
        println!(
            "FullName: {:?}, Address: {:?}, {:?} {:?} {:?}",
            deets.fullname,
            deets.address1,
            deets.city,
            deets.state,
            deets.postcode
        );
    } else if command == "image" {
        let imgs =
            endpoints::Image::get_all(&na_client, EndpointGetArgs::NoArgs)
                .await?;
        println!();
        // list images
        for img in imgs {
            println!(
                "ID: {}, Size: {}, Name: {}",
                img.id,
                img.size.unwrap_or("null".to_owned()),
                img.os.unwrap_or("null".to_owned())
            );
        }
        println!();
    } else if command == "invoice" {
        let invoices =
            endpoints::Invoices::get_all(&na_client, EndpointGetArgs::NoArgs)
                .await?;
        // print some of the invoices, say 3?
        for invoice in invoices.iter().take(display_count) {
            println!("ID: {}, Status: {}", invoice.id, invoice.status);
        }
    } else if command == "sizes" {
        let sizes = endpoints::Sizes::get_all(
            &na_client,
            EndpointGetArgs::OneInt(loc_sizes),
        )
        .await?;
        // print some of the invoices, say 3?
        for size in sizes.iter().take(40) {
            println!("ID: {}, Name: {}", size.plan_id, size.plan);
        }
    }
    Ok(())
}

///
/// This is the CLI Args struct
///
#[derive(Parser, Debug)]
#[command(version, about)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        #[command(subcommand)]
        cmd: GetCommands,
    },
    /// generate completions
    #[command(visible_alias = "gen")]
    GenerateCompletions { shell: Shell },
}

#[derive(Subcommand, Debug)]
enum GetCommands {
    /// Server subcommands
    #[command(visible_alias = "srv")]
    Server {
        // -i argument for picking an mbpkgid
        #[arg(short, long, default_value_t = 0)]
        mbpkgid: u32,
    },

    /// DNS subcommands
    Dns {
        // -i argument for picking a dns zone
        #[arg(short, long, default_value_t = 0)]
        id: u32,
    },

    /// SSh subcommands
    Ssh {
        // -i argument for ssh keyid
        #[arg(short, long, default_value_t = 0)]
        id: u32,
    },

    /// Job subcommands
    Job {
        // -i argument for picking a Job
        #[arg(short, long)]
        mbpkgid: u32,
        #[arg(short, long, default_value_t = 0)]
        jobid: u32,
    },

    /// IPs subcommands
    Ip {
        // --proto argument (-p) for 4 or 6
        // default to 4
        #[arg(short, long)]
        mbpkgid: u32,
    },

    /// Monthly Bandwidth subcommands
    #[command(visible_alias = "bw")]
    Bandwidth {
        #[arg(short, long)]
        mbpkgid: u32,
    },

    /// Invoices subcommands
    #[command(visible_alias = "inv")]
    Invoice {
        // -i argument for number to display
        #[arg(short, long, default_value_t = 5)]
        count: usize,
    },

    /// Invoices subcommands
    #[command(visible_alias = "sz")]
    Sizes {
        // -i argument for number to display
        #[arg(short, long)]
        id: u32,
    },

    /// Location subcommands
    #[command(visible_alias = "loc")]
    Location {},

    /// Images subcommands
    #[command(visible_alias = "img")]
    Image {},

    /// Account subcommands
    Account {},
}
