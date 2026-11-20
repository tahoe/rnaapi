//! CLI for the NetActuate API, built on the `rnaapi` crate.
//!
//! ```bash
//! export API_KEY='<your api key>'
//! # optional; defaults to https://vapi2.netactuate.com/api/
//! export API_ADDRESS='https://vapi2.netactuate.com/api/'
//! ```
//!
//! ```text
//! rnacli server get
//! rnacli server get -m <mbpkgid>
//! rnacli server off -m <mbpkgid>
//! rnacli dedicated off -m <mbpkgid>
//! rnacli server terminate -m <mbpkgid>
//! rnacli server cancel -m <mbpkgid> --cancel-type Immediate --agree
//! rnacli server build --fqdn <name> --plan <plan> --location <id> --image <id>
//! rnacli server rebuild -m <mbpkgid> --image <id>
//! ```
//!
// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use anyhow::{Context, Result, bail};
use chrono::NaiveDate;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use rnaapi::NaClient;
use rnaapi::config::Settings;
use rnaapi::endpoints::{self, MonthlyBw};
use rnacli::commands::get_pub_key;
use rnacli::commands::new_ssh_key;
use rnacli::commands::{
    AccountCommands, BandwidthCommands, BuildArgs, Cli, Commands,
    DedicatedCommands, DnsCommands, ImageCommands, InvoiceCommands, IpCommands,
    JobCommands, LocationCommands, PowerTarget, PubkeyCommands, RebuildArgs,
    ServerCommands, SizesCommands, SshCommands,
};

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
        Some(Commands::Server { cmd }) => server(cmd).await,
        Some(Commands::Dedicated { cmd }) => dedicated(cmd).await,
        Some(Commands::Dns { cmd }) => dns(cmd).await,
        Some(Commands::Ssh { cmd }) => ssh(cmd).await,
        Some(Commands::Job { cmd }) => job(cmd).await,
        Some(Commands::Ip { cmd }) => ip(cmd).await,
        Some(Commands::Bandwidth { cmd }) => bandwidth(cmd).await,
        Some(Commands::Invoice { cmd }) => invoice(cmd).await,
        Some(Commands::Sizes { cmd }) => sizes(cmd).await,
        Some(Commands::Location { cmd }) => location(cmd).await,
        Some(Commands::Image { cmd }) => image(cmd).await,
        Some(Commands::Account { cmd }) => account(cmd).await,
        Some(Commands::Pubkey { cmd }) => pubkey(cmd),
        None => Ok(()),
    }
}

fn pubkey(cmd: PubkeyCommands) -> Result<()> {
    match cmd {
        PubkeyCommands::Get { file } => {
            let key = get_pub_key(&file)?;
            println!("PubKey for {file} is: {key}");
            Ok(())
        }
    }
}

async fn create_ssh(file: Option<String>) -> Result<()> {
    let key = new_ssh_key(file.as_deref())?;
    if let Some(private) = &key.private_path {
        println!("Saved {}", private.display());
        println!("Saved {}", private.with_extension("pub").display());
    }
    let settings = Settings::new()?;
    let na_client = NaClient::new(settings.api_key, settings.api_url).await?;
    let created = endpoints::SSHKeys::create(
        &na_client,
        &endpoints::SshKeyBody {
            name: key.name,
            ssh_key: key.ssh_key,
        },
    )
    .await?;
    println!(
        "ID: {}, Name: {}, Fingerprint: {}",
        created.id, created.name, created.fingerprint
    );
    Ok(())
}

async fn power_client() -> Result<NaClient> {
    let settings = Settings::new()?;
    let client = NaClient::new(settings.api_key, settings.api_url).await?;
    Ok(client)
}

async fn server(cmd: ServerCommands) -> Result<()> {
    match cmd {
        ServerCommands::Get { mbpkgid } => {
            print_server(&api_client().await?, mbpkgid).await
        }
        ServerCommands::Terminate { mbpkgid } => terminate(mbpkgid).await,
        ServerCommands::Cancel {
            mbpkgid,
            cancel_type,
            agree,
        } => {
            if !agree {
                bail!(
                    "I understand that my server and its data will be destroyed."
                );
            }
            cancel(mbpkgid, cancel_type).await
        }
        ServerCommands::Build(args) => server_build(args).await,
        ServerCommands::Rebuild(args) => server_rebuild(args).await,
        ServerCommands::On(target) => {
            cloud_power(target, CloudPower::On).await
        }
        ServerCommands::Off(target) => {
            cloud_power(target, CloudPower::Off).await
        }
        ServerCommands::Reboot(target) => {
            cloud_power(target, CloudPower::Reboot).await
        }
    }
}

enum CloudPower {
    On,
    Off,
    Reboot,
}

async fn cloud_power(target: PowerTarget, verb: CloudPower) -> Result<()> {
    let PowerTarget { mbpkgid, force } = target;
    require_id("mbpkgid", mbpkgid)?;
    let na_client = power_client().await?;
    let body = endpoints::ServerPower {
        force: force.then_some(true),
    };
    let result = match verb {
        CloudPower::On => {
            endpoints::Server::start(&na_client, mbpkgid, &body).await?
        }
        CloudPower::Off => {
            endpoints::Server::shutdown(&na_client, mbpkgid, &body).await?
        }
        CloudPower::Reboot => {
            endpoints::Server::reboot(&na_client, mbpkgid, &body).await?
        }
    };
    println!("{result}");
    Ok(())
}

fn build_body(args: &BuildArgs) -> endpoints::ServerBuildRequest {
    endpoints::ServerBuildRequest {
        plan: args.plan.clone(),
        location: args.location,
        image: args.image,
        fqdn: args.fqdn.clone(),
        ssh_key: args.ssh_key.clone(),
        ssh_key_id: args.ssh_key_id,
        password: args.password.clone(),
        package_billing: args.package_billing.clone(),
        package_billing_contract_id: args.package_billing_contract_id.clone(),
        cloud_config: args.cloud_config.clone(),
        script_content: args.script_content.clone(),
        script_type: args.script_type.clone(),
        params: args.params.clone(),
        tag: args.tag.clone(),
        tag_list: (!args.tag_list.is_empty()).then(|| args.tag_list.clone()),
        cloud_pool_id: args.cloud_pool_id,
        vpc_id: args.vpc_id,
    }
}

async fn server_build(args: BuildArgs) -> Result<()> {
    let na_client = power_client().await?;
    let built =
        endpoints::Server::buy(&na_client, &build_body(&args)).await?;
    println!(
        "built: {} mbpkgid {} status {}",
        args.fqdn.unwrap_or_default(),
        built.mbpkgid,
        built.status
    );
    Ok(())
}

async fn server_rebuild(args: RebuildArgs) -> Result<()> {
    require_id("mbpkgid", args.mbpkgid)?;
    let na_client = power_client().await?;
    let built = endpoints::Server::rebuild(
        &na_client,
        args.mbpkgid,
        &build_body(&args.build),
    )
    .await?;
    println!(
        "rebuilt: {} mbpkgid {} status {}",
        args.build.fqdn.unwrap_or_default(),
        built.mbpkgid,
        built.status
    );
    Ok(())
}

async fn terminate(mbpkgid: u32) -> Result<()> {
    require_id("mbpkgid", mbpkgid)?;
    let na_client = power_client().await?;
    let body = endpoints::DeleteServer::default();
    let deleted =
        endpoints::Server::delete(&na_client, mbpkgid, &body).await?;
    println!("terminated {mbpkgid} id {}", deleted.id);
    Ok(())
}

async fn cancel(mbpkgid: u32, cancel_type: String) -> Result<()> {
    require_id("mbpkgid", mbpkgid)?;
    let na_client = power_client().await?;
    let body = endpoints::CancelPackage {
        mbpkgid: i64::from(mbpkgid),
        domu_package: None,
        comments: None,
        cancel_type,
        agree: true,
        password: None,
    };
    let result = endpoints::Package::cancel(&na_client, &body).await?;
    println!("{result}");
    Ok(())
}

async fn dedicated(cmd: DedicatedCommands) -> Result<()> {
    let (verb, target) = match cmd {
        DedicatedCommands::On(target) => ("on", target),
        DedicatedCommands::Off(target) => ("off", target),
        DedicatedCommands::Reboot(target) => ("reboot", target),
    };
    let PowerTarget { mbpkgid, force } = target;
    require_id("mbpkgid", mbpkgid)?;
    let na_client = power_client().await?;
    let body = endpoints::DedicatedAction {
        force: force.then_some(true),
        password: None,
    };
    match verb {
        "on" => {
            endpoints::MetalServer::start(&na_client, mbpkgid, &body).await?;
        }
        "off" => {
            endpoints::MetalServer::shutdown(&na_client, mbpkgid, &body)
                .await?;
        }
        _ => {
            endpoints::MetalServer::reboot(&na_client, mbpkgid, &body).await?;
        }
    }
    println!("{verb} {mbpkgid}");
    Ok(())
}

async fn api_client() -> Result<NaClient> {
    let settings = Settings::new()?;
    let client = NaClient::new(settings.api_key, settings.api_url).await?;
    Ok(client)
}

async fn dns(cmd: DnsCommands) -> Result<()> {
    match cmd {
        DnsCommands::Get { id } => print_dns(&api_client().await?, id).await,
    }
}

async fn ssh(cmd: SshCommands) -> Result<()> {
    match cmd {
        SshCommands::Get { id } => print_ssh(&api_client().await?, id).await,
        SshCommands::New { file } => create_ssh(file).await,
    }
}

async fn job(cmd: JobCommands) -> Result<()> {
    match cmd {
        JobCommands::Get { mbpkgid, jobid } => {
            print_job(&api_client().await?, mbpkgid, jobid).await
        }
    }
}

async fn ip(cmd: IpCommands) -> Result<()> {
    match cmd {
        IpCommands::Get { mbpkgid } => {
            print_ips(&api_client().await?, mbpkgid).await
        }
    }
}

async fn bandwidth(cmd: BandwidthCommands) -> Result<()> {
    match cmd {
        BandwidthCommands::Get { mbpkgid } => {
            print_bandwidth(&api_client().await?, mbpkgid).await
        }
    }
}

async fn invoice(cmd: InvoiceCommands) -> Result<()> {
    match cmd {
        InvoiceCommands::Get { count } => {
            print_invoices(&api_client().await?, count).await
        }
    }
}

async fn sizes(cmd: SizesCommands) -> Result<()> {
    match cmd {
        SizesCommands::Get { id } => print_sizes(&api_client().await?, id).await,
    }
}

async fn location(cmd: LocationCommands) -> Result<()> {
    match cmd {
        LocationCommands::Get { id } => {
            print_locations(&api_client().await?, id).await
        }
    }
}

async fn image(cmd: ImageCommands) -> Result<()> {
    match cmd {
        ImageCommands::Get => print_images(&api_client().await?).await,
    }
}

async fn account(cmd: AccountCommands) -> Result<()> {
    match cmd {
        AccountCommands::Get => print_account(&api_client().await?).await,
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

async fn print_server(
    na_client: &NaClient, mbpkgid: Option<u32>,
) -> Result<()> {
    let Some(mbpkgid) = mbpkgid else {
        let servers = endpoints::Server::list(na_client).await?;
        for server in servers {
            println!("ID: {}, fqdn: {}", server.mbpkgid, server.fqdn);
        }
        return Ok(());
    };

    let (server, srvjobs, ipv4s, ipv6s, status, bw_usage) = tokio::join!(
        endpoints::Server::get(na_client, mbpkgid),
        endpoints::SrvJob::list(na_client, mbpkgid),
        endpoints::IPv4::list(na_client, mbpkgid),
        endpoints::IPv6::list(na_client, mbpkgid),
        endpoints::SrvStatus::get(na_client, mbpkgid),
        endpoints::MonthlyBw::list(na_client, mbpkgid),
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

async fn print_dns(na_client: &NaClient, id: Option<u32>) -> Result<()> {
    let Some(id) = id else {
        println!();
        let zones = endpoints::Zone::list(na_client).await?;
        for zone in zones {
            println!(
                "ID: {}, Name: {}, Type: {}",
                zone.id, zone.name, zone.zone_type
            );
        }
        return Ok(());
    };

    println!();
    let zone = endpoints::Zone::get(na_client, id).await?;
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

async fn print_ssh(na_client: &NaClient, id: Option<u32>) -> Result<()> {
    println!();
    if let Some(id) = id {
        let sshkey = endpoints::SSHKeys::get(na_client, id).await?;
        println!(
            "ID: {}, Name: {}, Fingerprint: {}",
            sshkey.id, sshkey.name, sshkey.fingerprint
        );
    } else {
        let keys = endpoints::SSHKeys::list(na_client).await?;
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
        let job = endpoints::SrvJob::get(na_client, mbpkgid, jobid).await?;
        println!(
            "ID: {}, Status: {}, Command: {}",
            job.id, job.status, job.command
        );
    } else {
        let jobs = endpoints::SrvJob::list(na_client, mbpkgid).await?;
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
        endpoints::IPv4::list(na_client, mbpkgid),
        endpoints::IPv6::list(na_client, mbpkgid),
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
    let mut bw_usage = endpoints::MonthlyBw::list(na_client, mbpkgid).await?;
    println!();
    sort_bandwidth(&mut bw_usage)?;
    for usage in bw_usage {
        println!("Date: {}, Rx: {}, Tx: {}", usage.date, usage.rx, usage.tx);
    }
    Ok(())
}

async fn print_locations(na_client: &NaClient, id: Option<u32>) -> Result<()> {
    println!();
    let Some(id) = id else {
        let locations = endpoints::Location::list(na_client).await?;
        for location in locations {
            println!(
                "ID: {}, Name: {}, Continent: {}",
                location.id, location.name, location.continent
            );
        }
        return Ok(());
    };

    let location = endpoints::Location::get(na_client, id).await?;
    println!(
        "ID: {}, Name: {}, IATA: {}, Continent: {}",
        location.id, location.name, location.iata_code, location.continent
    );
    for status in location.status {
        println!(
            "Status: {}, {}",
            status.status_name, status.container_status
        );
    }
    Ok(())
}

async fn print_account(na_client: &NaClient) -> Result<()> {
    let details = endpoints::Details::get(na_client).await?;
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
    let images = endpoints::Image::list(na_client).await?;
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
    let invoices = endpoints::Invoices::list(na_client).await?;
    for invoice in invoices.iter().take(count) {
        println!("ID: {}, Status: {}", invoice.id, invoice.status);
    }
    Ok(())
}

async fn print_sizes(na_client: &NaClient, id: u32) -> Result<()> {
    require_id("id", id)?;
    let sizes = endpoints::Sizes::list(na_client, id).await?;
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
