//! CLI for the NetActuate API, built on the `rnaapi` crate.
//!
//! ```bash
//! export API_KEY='<your api key>'
//! # optional; defaults to https://vapi2.netactuate.com/api/
//! export API_ADDRESS='https://vapi2.netactuate.com/api/'
//! ```
//!
//! ```text
//! rnatui get server
//! rnatui get server -m <mbpkgid>
//! ```
//!
// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use anyhow::{Context, Result, bail};
use chrono::NaiveDate;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use rnaapi::config::Settings;
use rnaapi::endpoints::{self, MonthlyBw};
use rnaapi::{EndpointGetAll, EndpointGetArgs, EndpointGetOne, NaClient};
use rnatui::commands::get_pub_key;
use rnatui::commands::{Cli, Commands, GetCommands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Some(Commands::GenerateCompletions { shell }) => {
            let mut app = Cli::command();
            let name = app.get_name().to_string();
            generate(shell, &mut app, name, &mut std::io::stdout());
            Ok(())
        }
        Some(Commands::Get { cmd }) => match cmd {
            GetCommands::Pubkey { file } => {
                let pubkey = get_pub_key(&file)?;
                println!("PubKey for {file} is: {pubkey}");
                Ok(())
            }
            other => dispatch(other).await,
        },
        None => Ok(()),
    }
}

async fn dispatch(cmd: GetCommands) -> Result<()> {
    let settings = Settings::new()?;
    let na_client = NaClient::new(settings.api_key, settings.api_url).await?;
    match cmd {
        GetCommands::Pubkey { .. } => {
            bail!("public key lookup does not use the API");
        }
        GetCommands::Server { mbpkgid } => {
            print_server(&na_client, mbpkgid).await
        }
        GetCommands::Dns { id } => print_dns(&na_client, id).await,
        GetCommands::Ssh { id } => print_ssh(&na_client, id).await,
        GetCommands::Job { mbpkgid, jobid } => {
            print_job(&na_client, mbpkgid, jobid).await
        }
        GetCommands::Ip { mbpkgid } => print_ips(&na_client, mbpkgid).await,
        GetCommands::Bandwidth { mbpkgid } => {
            print_bandwidth(&na_client, mbpkgid).await
        }
        GetCommands::Invoice { count } => {
            print_invoices(&na_client, count).await
        }
        GetCommands::Sizes { id } => print_sizes(&na_client, id).await,
        GetCommands::Location {} => print_locations(&na_client).await,
        GetCommands::Image {} => print_images(&na_client).await,
        GetCommands::Account {} => print_account(&na_client).await,
    }
}

fn require_id(name: &str, id: u32) -> Result<u32> {
    if id == 0 {
        bail!("{name} must be greater than 0");
    }
    Ok(id)
}

fn bandwidth_date(date: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(&format!("{date}-01"), "%Y-%m-%d")
        .with_context(|| format!("bandwidth date `{date}` is not YYYY-MM"))
}

fn sort_bandwidth(bw_usage: &mut [MonthlyBw]) -> Result<()> {
    let mut dated = Vec::with_capacity(bw_usage.len());
    for usage in bw_usage.iter().cloned() {
        dated.push((bandwidth_date(&usage.date)?, usage));
    }
    dated.sort_by_key(|(date, _)| *date);
    for (slot, (_, usage)) in bw_usage.iter_mut().zip(dated) {
        *slot = usage;
    }
    Ok(())
}

async fn print_server(na_client: &NaClient, mbpkgid: u32) -> Result<()> {
    if mbpkgid == 0 {
        let servers =
            endpoints::Server::get_all(na_client, EndpointGetArgs::NoArgs)
                .await?;
        for server in servers {
            println!("ID: {}, fqdn: {}", server.mbpkgid, server.fqdn);
        }
        return Ok(());
    }

    let (server, srvjobs, ipv4s, ipv6s, status, bw_usage) = tokio::join!(
        endpoints::Server::get_one(na_client, EndpointGetArgs::OneInt(mbpkgid)),
        endpoints::SrvJob::get_all(na_client, EndpointGetArgs::OneInt(mbpkgid)),
        endpoints::IPv4::get_all(na_client, EndpointGetArgs::OneInt(mbpkgid)),
        endpoints::IPv6::get_all(na_client, EndpointGetArgs::OneInt(mbpkgid)),
        endpoints::SrvStatus::get_one(
            na_client,
            EndpointGetArgs::OneInt(mbpkgid)
        ),
        endpoints::MonthlyBw::get_all(
            na_client,
            EndpointGetArgs::OneInt(mbpkgid)
        ),
    );

    let mut failed = false;
    match server {
        Ok(srv) => println!(
            "Package: {}, fqdn: {}, mbpkgid: {}",
            srv.domu_package, srv.fqdn, srv.mbpkgid
        ),
        Err(err) => {
            eprintln!("server: {err}");
            failed = true;
        }
    }

    println!();
    match srvjobs {
        Ok(jobs) => {
            for job in jobs {
                println!(
                    "Inserted: {}, Status: {}, command: {}",
                    job.ts_insert, job.status, job.command
                );
            }
        }
        Err(err) => {
            eprintln!("jobs: {err}");
            failed = true;
        }
    }

    println!();
    match ipv4s {
        Ok(addresses) => {
            for ipv4 in addresses {
                println!(
                    "Reverse: {}, IP: {}, Gateway: {}",
                    ipv4.reverse, ipv4.ip, ipv4.gateway
                );
            }
        }
        Err(err) => {
            eprintln!("ipv4: {err}");
            failed = true;
        }
    }

    println!();
    match ipv6s {
        Ok(addresses) => {
            for ipv6 in addresses {
                println!(
                    "Reverse: {}, IP: {}, Gateway: {}",
                    ipv6.reverse, ipv6.ip, ipv6.gateway
                );
            }
        }
        Err(err) => {
            eprintln!("ipv6: {err}");
            failed = true;
        }
    }

    println!();
    match status {
        Ok(stat) => println!("Status: {}", stat.status),
        Err(err) => {
            eprintln!("status: {err}");
            failed = true;
        }
    }

    println!();
    match bw_usage {
        Ok(mut usage) => {
            sort_bandwidth(&mut usage)?;
            for sample in usage {
                println!(
                    "Date: {}, Rx: {}, Tx: {}",
                    sample.date, sample.rx, sample.tx
                );
            }
        }
        Err(err) => {
            eprintln!("bandwidth: {err}");
            failed = true;
        }
    }

    if failed {
        bail!("one or more server requests failed");
    }
    Ok(())
}

async fn print_dns(na_client: &NaClient, id: u32) -> Result<()> {
    if id == 0 {
        println!();
        let zones =
            endpoints::Zone::get_all(na_client, EndpointGetArgs::NoArgs)
                .await?;
        for zone in zones {
            println!(
                "ID: {}, Name: {}, Type: {}",
                zone.id, zone.name, zone.zone_type
            );
        }
        return Ok(());
    }

    println!();
    let zone = endpoints::Zone::get_one(na_client, EndpointGetArgs::OneInt(id))
        .await?;
    println!("Zone: {}", zone.name);
    if let Some(soa) = &zone.soa {
        println!("SOA: {}", soa.primary);
    }
    if let Some(record) =
        zone.records.as_ref().and_then(|records| records.first())
    {
        println!("1st Record: {}", record.name);
    }
    if let Some(ns) = zone.ns.as_ref().and_then(|records| records.first()) {
        println!("1st NS: {}", ns.name);
    }
    Ok(())
}

async fn print_ssh(na_client: &NaClient, id: u32) -> Result<()> {
    println!();
    if id > 0 {
        let sshkey =
            endpoints::SSHKeys::get_one(na_client, EndpointGetArgs::OneInt(id))
                .await?;
        println!(
            "ID: {}, Name: {}, Fingerprint: {}",
            sshkey.id, sshkey.name, sshkey.fingerprint
        );
    } else {
        let keys =
            endpoints::SSHKeys::get_all(na_client, EndpointGetArgs::NoArgs)
                .await?;
        for sshkey in keys {
            println!(
                "ID: {}, Name: {}, Fingerprint: {}",
                sshkey.id, sshkey.name, sshkey.fingerprint
            );
        }
    }
    Ok(())
}

async fn print_job(
    na_client: &NaClient, mbpkgid: u32, jobid: u32,
) -> Result<()> {
    require_id("mbpkgid", mbpkgid)?;
    println!();
    if jobid > 0 {
        let job = endpoints::SrvJob::get_one(
            na_client,
            EndpointGetArgs::TwoInt(mbpkgid, jobid),
        )
        .await?;
        println!(
            "ID: {}, Status: {}, Command: {}",
            job.id, job.status, job.command
        );
    } else {
        let jobs = endpoints::SrvJob::get_all(
            na_client,
            EndpointGetArgs::OneInt(mbpkgid),
        )
        .await?;
        for job in jobs {
            println!(
                "ID: {}, Status: {}, Command: {}",
                job.id, job.status, job.command
            );
        }
    }
    Ok(())
}

async fn print_ips(na_client: &NaClient, mbpkgid: u32) -> Result<()> {
    require_id("mbpkgid", mbpkgid)?;
    let (ipv4s, ipv6s) = tokio::join!(
        endpoints::IPv4::get_all(na_client, EndpointGetArgs::OneInt(mbpkgid)),
        endpoints::IPv6::get_all(na_client, EndpointGetArgs::OneInt(mbpkgid)),
    );
    let mut failed = false;
    match ipv4s {
        Ok(addresses) => {
            for ipv4 in addresses {
                println!(
                    "Reverse: {}, IP: {}, Gateway: {}",
                    ipv4.reverse, ipv4.ip, ipv4.gateway
                );
            }
        }
        Err(err) => {
            eprintln!("ipv4: {err}");
            failed = true;
        }
    }
    match ipv6s {
        Ok(addresses) => {
            for ipv6 in addresses {
                println!(
                    "Reverse: {}, IP: {}, Gateway: {}",
                    ipv6.reverse, ipv6.ip, ipv6.gateway
                );
            }
        }
        Err(err) => {
            eprintln!("ipv6: {err}");
            failed = true;
        }
    }
    if failed {
        bail!("one or more address requests failed");
    }
    Ok(())
}

async fn print_bandwidth(na_client: &NaClient, mbpkgid: u32) -> Result<()> {
    require_id("mbpkgid", mbpkgid)?;
    let mut bw_usage = endpoints::MonthlyBw::get_all(
        na_client,
        EndpointGetArgs::OneInt(mbpkgid),
    )
    .await?;
    println!();
    sort_bandwidth(&mut bw_usage)?;
    for usage in bw_usage {
        println!("Date: {}, Rx: {}, Tx: {}", usage.date, usage.rx, usage.tx);
    }
    Ok(())
}

async fn print_locations(na_client: &NaClient) -> Result<()> {
    let locations =
        endpoints::Location::get_all(na_client, EndpointGetArgs::NoArgs)
            .await?;
    println!();
    for location in locations {
        println!(
            "ID: {}, Name: {}, Continent: {}",
            location.id, location.name, location.continent
        );
    }
    Ok(())
}

async fn print_account(na_client: &NaClient) -> Result<()> {
    let details =
        endpoints::Details::get_one(na_client, EndpointGetArgs::NoArgs).await?;
    println!();
    println!(
        "FullName: {:?}, Address: {:?}, {:?} {:?} {:?}",
        details.fullname,
        details.address1,
        details.city,
        details.state,
        details.postcode
    );
    Ok(())
}

async fn print_images(na_client: &NaClient) -> Result<()> {
    let images =
        endpoints::Image::get_all(na_client, EndpointGetArgs::NoArgs).await?;
    println!();
    for image in images {
        println!(
            "ID: {}, Size: {}, Name: {}",
            image.id,
            image.size.unwrap_or_else(|| "null".to_owned()),
            image.os.unwrap_or_else(|| "null".to_owned())
        );
    }
    println!();
    Ok(())
}

async fn print_invoices(na_client: &NaClient, count: usize) -> Result<()> {
    let invoices =
        endpoints::Invoices::get_all(na_client, EndpointGetArgs::NoArgs)
            .await?;
    for invoice in invoices.iter().take(count) {
        println!("ID: {}, Status: {}", invoice.id, invoice.status);
    }
    Ok(())
}

async fn print_sizes(na_client: &NaClient, id: u32) -> Result<()> {
    require_id("id", id)?;
    let sizes =
        endpoints::Sizes::get_all(na_client, EndpointGetArgs::OneInt(id))
            .await?;
    for size in sizes {
        println!("ID: {}, Name: {}", size.plan_id, size.plan);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bandwidth_dates_sort_chronologically() {
        let mut usage = vec![
            MonthlyBw {
                date: "2024-12".into(),
                rx: 1,
                tx: 1,
            },
            MonthlyBw {
                date: "2024-02".into(),
                rx: 2,
                tx: 2,
            },
        ];
        sort_bandwidth(&mut usage).unwrap();
        assert_eq!(usage[0].date, "2024-02");
        assert_eq!(usage[1].date, "2024-12");
    }

    #[test]
    fn a_bad_bandwidth_date_is_an_error() {
        let mut usage = vec![MonthlyBw {
            date: "March".into(),
            rx: 1,
            tx: 1,
        }];
        let err = sort_bandwidth(&mut usage).unwrap_err();
        assert!(err.to_string().contains("March"));
    }
}
