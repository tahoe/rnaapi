// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0

use std::sync::Arc;

use rnaapi::NaClient;
use rnaapi::endpoints::{
    DedicatedAction, Details, IPv4, IPv6, Image, Invoices, Location,
    MetalServer, MonthlyBw, SSHKeys, Server, ServerPower, Sizes, SrvJob,
    SrvStatus, SshKeyBody, Zone,
};
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

use crate::focus::ServerPane;
use crate::model::LocationRow;
use crate::model::{
    DetailQuery, Field, Pane, PowerCommand, Reply, Request, Section,
    ServerBatch, ServerDetail, ServerKind, ServerRow, SshRow, ZoneRow,
    force_flag, merge_servers, power_call_text,
};

pub fn spawn(
    client: Arc<NaClient>, request: Request, tx: UnboundedSender<Reply>,
) {
    tokio::spawn(async move {
        let reply = perform(client.as_ref(), request).await;
        let _ = tx.send(reply);
    });
}

async fn perform(client: &NaClient, request: Request) -> Reply {
    match request {
        Request::Servers(token) => Reply::Servers {
            token,
            result: load_servers(client).await,
        },
        Request::ServerDetail(query) => {
            let token = query.token;
            Reply::ServerDetail {
                token,
                result: load_server(client, query).await,
            }
        }
        Request::ServerList(query) => {
            let token = query.token;
            let pane = query.pane;
            let (lines, errors) =
                load_server_pane(client, pane, query.mbpkgid).await;
            Reply::ServerList {
                token,
                pane,
                lines,
                errors,
            }
        }
        Request::Power(command) => {
            let token = command.token;
            Reply::Power {
                token,
                result: run_power(client, &command).await,
            }
        }
        Request::Ssh(token) => Reply::Ssh {
            token,
            result: load_ssh(client).await,
        },
        Request::SshCreate { token, file } => Reply::SshCreated {
            token,
            result: create_ssh(client, file).await,
        },
        Request::Locations(token) => Reply::Locations {
            token,
            result: load_locations(client).await,
        },
        Request::LocationDetail { token, id } => Reply::LocationDetail {
            token,
            result: load_location(client, id).await,
        },
        Request::Zones(token) => Reply::Zones {
            token,
            result: load_zones(client).await,
        },
        Request::ZoneDetail { token, id } => Reply::ZoneDetail {
            token,
            result: load_zone(client, id).await,
        },
        Request::Account(token) => Reply::Account {
            token,
            result: load_account(client).await,
        },
    }
}

async fn load_servers(client: &NaClient) -> Result<ServerBatch, String> {
    let (virtuals, metals) =
        tokio::join!(Server::list(client), MetalServer::list(client));
    let mut warning = Vec::new();
    let virtual_rows = match virtuals {
        Ok(rows) => map_virtuals(rows, &mut warning),
        Err(err) => {
            warning.push(format!("virtual: {err}"));
            Vec::new()
        }
    };
    let metal_rows = match metals {
        Ok(rows) => map_metals(rows, &mut warning),
        Err(err) => {
            warning.push(format!("dedicated: {err}"));
            Vec::new()
        }
    };
    if virtual_rows.is_empty() && metal_rows.is_empty() && !warning.is_empty() {
        return Err(warning.join("; "));
    }
    Ok(ServerBatch {
        rows: merge_servers(virtual_rows, metal_rows),
        warning: (!warning.is_empty()).then(|| warning.join("; ")),
    })
}

fn map_virtuals(
    rows: Vec<Server>, warning: &mut Vec<String>,
) -> Vec<ServerRow> {
    let mut out = Vec::new();
    for server in rows {
        match server_row(&server) {
            Some(row) => out.push(row),
            None => {
                warning.push(format!("skipped virtual id {}", server.mbpkgid))
            }
        }
    }
    out
}

fn map_metals(
    rows: Vec<MetalServer>, warning: &mut Vec<String>,
) -> Vec<ServerRow> {
    let mut out = Vec::new();
    for server in rows {
        match metal_row(&server) {
            Some(row) => out.push(row),
            None => {
                warning.push(format!("skipped dedicated id {}", server.mbpkgid))
            }
        }
    }
    out
}

pub fn server_row(server: &Server) -> Option<ServerRow> {
    Some(ServerRow {
        mbpkgid: server.mbpkgid,
        fqdn: fallback(&server.fqdn, "(no name)"),
        state: fallback(&server.state, &server.status),
        ip: server.ip.clone(),
        location: server.city.clone(),
        status: server.status.clone(),
        kind: ServerKind::Virtual,
        metal_id: None,
    })
}

pub fn metal_row(server: &MetalServer) -> Option<ServerRow> {
    let mbpkgid = u32::try_from(server.mbpkgid).ok().filter(|id| *id > 0)?;
    Some(ServerRow {
        mbpkgid,
        fqdn: fallback(&server.hostname, "(no name)"),
        state: fallback(&server.package_status, "unknown"),
        ip: server.primary_ip.clone(),
        location: server.location.clone(),
        status: server.package_status.clone(),
        kind: ServerKind::Dedicated,
        metal_id: u32::try_from(server.id).ok(),
    })
}

async fn load_server(
    client: &NaClient, query: DetailQuery,
) -> Result<ServerDetail, String> {
    match query.kind {
        ServerKind::Virtual => Ok(load_virtual(client, query.mbpkgid).await),
        ServerKind::Dedicated => Ok(load_dedicated(client, query).await),
    }
}

async fn load_virtual(client: &NaClient, id: u32) -> ServerDetail {
    let (server, status) =
        tokio::join!(Server::get(client, id), SrvStatus::get(client, id),);
    let mut detail = ServerDetail {
        fqdn: None,
        state: None,
        status: None,
        ip: None,
        location: None,
        extras: Vec::new(),
        error: None,
    };
    match server {
        Ok(server) => {
            detail.fqdn = Some(server.fqdn);
            detail.state = Some(server.state);
            detail.status = Some(server.status);
            detail.ip = Some(server.ip);
            detail.location = Some(server.city);
            if !server.uptime.is_empty() {
                detail.extras.push(Field::new("uptime", server.uptime));
            }
            if !server.ipv6.is_empty() {
                detail.extras.push(Field::new("ipv6", server.ipv6));
            }
        }
        Err(err) => detail.error = Some(format!("server: {err}")),
    }
    if let Ok(status) = status {
        detail.status = Some(status.status);
    }
    detail
}

async fn load_server_pane(
    client: &NaClient, pane: ServerPane, id: u32,
) -> (Vec<String>, Vec<String>) {
    match pane {
        ServerPane::Jobs => load_jobs(client, id).await,
        ServerPane::Address => load_addresses(client, id).await,
        ServerPane::Bandwidth => load_bandwidth(client, id).await,
    }
}

async fn load_jobs(client: &NaClient, id: u32) -> (Vec<String>, Vec<String>) {
    match SrvJob::list(client, id).await {
        Ok(jobs) => (jobs.iter().map(job_line).collect(), Vec::new()),
        Err(err) => (Vec::new(), vec![format!("jobs: {err}")]),
    }
}

async fn load_addresses(
    client: &NaClient, id: u32,
) -> (Vec<String>, Vec<String>) {
    let (v4, v6) = tokio::join!(IPv4::list(client, id), IPv6::list(client, id));
    let mut lines = Vec::new();
    let mut errors = Vec::new();
    match v4 {
        Ok(rows) => lines.extend(rows.iter().map(ipv4_line)),
        Err(err) => errors.push(format!("ipv4: {err}")),
    }
    match v6 {
        Ok(rows) => lines.extend(rows.iter().map(ipv6_line)),
        Err(err) => errors.push(format!("ipv6: {err}")),
    }
    (lines, errors)
}

async fn load_bandwidth(
    client: &NaClient, id: u32,
) -> (Vec<String>, Vec<String>) {
    match MonthlyBw::list(client, id).await {
        Ok(mut rows) => {
            rows.sort_by(|left, right| left.date.cmp(&right.date));
            (rows.iter().map(bandwidth_line).collect(), Vec::new())
        }
        Err(err) => (Vec::new(), vec![format!("bandwidth: {err}")]),
    }
}

fn job_line(job: &SrvJob) -> String {
    format!(
        "{}  {}  {}  {}",
        job.id, job.ts_insert, job.status, job.command
    )
}

fn ipv4_line(row: &IPv4) -> String {
    address_line("ipv4", &row.ip, &row.netmask, &row.gateway, &row.reverse)
}

fn ipv6_line(row: &IPv6) -> String {
    address_line("ipv6", &row.ip, &row.netmask, &row.gateway, &row.reverse)
}

fn address_line(
    kind: &str, ip: &str, netmask: &str, gateway: &str, reverse: &str,
) -> String {
    let mut line = format!("{kind}  {ip}  {netmask}  {gateway}");
    if !reverse.is_empty() {
        line.push_str("  ");
        line.push_str(reverse);
    }
    line
}

fn bandwidth_line(row: &MonthlyBw) -> String {
    format!("{}  rx {}  tx {}", row.date, row.rx, row.tx)
}

async fn load_dedicated(client: &NaClient, query: DetailQuery) -> ServerDetail {
    let Some(id) = query.metal_id else {
        return ServerDetail {
            fqdn: None,
            state: None,
            status: None,
            ip: None,
            location: None,
            extras: Vec::new(),
            error: Some("dedicated server has no device id".into()),
        };
    };
    match MetalServer::get(client, id).await {
        Ok(server) => ServerDetail {
            fqdn: Some(fallback(&server.hostname, "(no name)")),
            state: Some(fallback(&server.package_status, "unknown")),
            status: Some(server.package_status.clone()),
            ip: Some(server.primary_ip.clone()),
            location: Some(server.location.clone()),
            extras: metal_extras(&server),
            error: None,
        },
        Err(err) => ServerDetail {
            fqdn: None,
            state: None,
            status: None,
            ip: None,
            location: None,
            extras: Vec::new(),
            error: Some(err.to_string()),
        },
    }
}

fn metal_extras(server: &MetalServer) -> Vec<Field> {
    let mut extras = Vec::new();
    if !server.cpu0_model.is_empty() {
        extras.push(Field::new("cpu", server.cpu0_model.clone()));
    }
    if server.total_ram > 0 {
        extras.push(Field::new("ram", server.total_ram.to_string()));
    }
    if let Some(ipv6) = server.primary_ipv6.as_ref().filter(|ip| !ip.is_empty())
    {
        extras.push(Field::new("ipv6", ipv6.clone()));
    }
    extras
}

async fn run_power(
    client: &NaClient, command: &PowerCommand,
) -> Result<String, String> {
    let force = force_flag(command.force);
    let call = power_call_text(
        command.kind,
        command.verb,
        command.mbpkgid,
        command.force,
    );
    match command.kind {
        ServerKind::Virtual => {
            let body = ServerPower { force };
            let value = match command.verb {
                crate::model::PowerVerb::On => {
                    Server::start(client, command.mbpkgid, &body).await
                }
                crate::model::PowerVerb::Off => {
                    Server::shutdown(client, command.mbpkgid, &body).await
                }
                crate::model::PowerVerb::Reboot => {
                    Server::reboot(client, command.mbpkgid, &body).await
                }
            }
            .map_err(|err| err.to_string())?;
            Ok(format!("{call}  {}", compact_value(&value)))
        }
        ServerKind::Dedicated => {
            let body = DedicatedAction {
                force,
                password: None,
            };
            let result = match command.verb {
                crate::model::PowerVerb::On => {
                    MetalServer::start(client, command.mbpkgid, &body).await
                }
                crate::model::PowerVerb::Off => {
                    MetalServer::shutdown(client, command.mbpkgid, &body).await
                }
                crate::model::PowerVerb::Reboot => {
                    MetalServer::reboot(client, command.mbpkgid, &body).await
                }
            };
            result.map_err(|err| err.to_string())?;
            Ok(call)
        }
    }
}

async fn load_ssh(client: &NaClient) -> Result<Vec<SshRow>, String> {
    let mut keys = SSHKeys::list(client)
        .await
        .map_err(|err| err.to_string())?
        .into_iter()
        .map(|key| SshRow {
            id: key.id,
            name: key.name,
            fingerprint: key.fingerprint,
        })
        .collect::<Vec<_>>();
    keys.sort_by_key(|key| key.id);
    Ok(keys)
}

async fn create_ssh(
    client: &NaClient, file: Option<String>,
) -> Result<String, String> {
    let prepared = tokio::task::spawn_blocking(move || {
        crate::ssh::new_ssh_key(file.as_deref())
    })
    .await
    .map_err(|_| "ssh key preparation stopped".to_string())?
    .map_err(|err| format!("{err:#}").replace('\n', ": "))?;
    let saved = prepared
        .private_path
        .as_ref()
        .map(|path| format!("Saved {}. ", path.display()));
    let created = SSHKeys::create(
        client,
        &SshKeyBody {
            name: prepared.name,
            ssh_key: prepared.ssh_key,
        },
    )
    .await
    .map_err(|err| match &saved {
        Some(prefix) => format!("{prefix}upload failed: {err}"),
        None => err.to_string(),
    })?;
    let prefix = saved.unwrap_or_default();
    Ok(format!(
        "{prefix}ID: {}, Name: {}, Fingerprint: {}",
        created.id, created.name, created.fingerprint
    ))
}

async fn load_locations(client: &NaClient) -> Result<Vec<LocationRow>, String> {
    let mut rows = Location::list(client)
        .await
        .map_err(|err| err.to_string())?
        .into_iter()
        .map(|row| LocationRow {
            id: row.id,
            name: row.name,
            continent: row.continent,
            iata: row.iata_code,
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| row.id);
    Ok(rows)
}

async fn load_location(client: &NaClient, id: u32) -> Result<Pane, String> {
    let (location, sizes) =
        tokio::join!(Location::get(client, id), Sizes::list(client, id));
    let location = location.map_err(|err| err.to_string())?;
    let mut fields = vec![
        Field::new("id", location.id.to_string()),
        Field::new("name", location.name.clone()),
        Field::new("iata", location.iata_code),
        Field::new("continent", location.continent),
        Field::new("disabled", location.disabled.to_string()),
    ];
    if !location.latitude.is_empty() || !location.longitude.is_empty() {
        fields.push(Field::new(
            "coordinates",
            format!("{}, {}", location.latitude, location.longitude),
        ));
    }
    let mut sections = Vec::new();
    if !location.status.is_empty() {
        sections.push(section(
            "status",
            cap(
                location
                    .status
                    .into_iter()
                    .map(|status| {
                        format!(
                            "{}  {}",
                            status.status_name, status.container_status
                        )
                    })
                    .collect(),
                8,
            ),
        ));
    }
    sections.push(match sizes {
        Ok(rows) => section(
            "sizes",
            cap(
                rows.into_iter()
                    .map(|size| {
                        format!(
                            "{}  ram {}  disk {}",
                            size.plan, size.ram, size.disk
                        )
                    })
                    .collect(),
                8,
            ),
        ),
        Err(err) => section("sizes", vec![err.to_string()]),
    });
    Ok(Pane {
        title: location.name,
        subtitle: format!("location {}", location.id),
        fields,
        sections,
    })
}

async fn load_zones(client: &NaClient) -> Result<Vec<ZoneRow>, String> {
    let mut rows = Zone::list(client)
        .await
        .map_err(|err| err.to_string())?
        .into_iter()
        .map(|zone| ZoneRow {
            id: zone.id,
            name: zone.name,
            zone_type: zone.zone_type,
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|zone| zone.id);
    Ok(rows)
}

async fn load_zone(client: &NaClient, id: u32) -> Result<Pane, String> {
    let zone = Zone::get(client, id).await.map_err(|err| err.to_string())?;
    let mut fields = vec![
        Field::new("id", zone.id.to_string()),
        Field::new("name", zone.name.clone()),
        Field::new("type", zone.zone_type),
    ];
    if let Some(ttl) = zone.ttl {
        fields.push(Field::new("ttl", ttl_text(&ttl)));
    }
    if let Some(soa) = zone.soa.as_ref() {
        fields.push(Field::new("soa", soa.primary.clone()));
        fields.push(Field::new("hostmaster", soa.hostmaster.clone()));
        fields.push(Field::new("serial", soa.serial.clone()));
    }
    let mut sections = Vec::new();
    if let Some(records) = zone.records {
        sections.push(section("records", record_lines(records)));
    }
    if let Some(ns) = zone.ns {
        sections.push(section("ns", record_lines(ns)));
    }
    Ok(Pane {
        title: zone.name,
        subtitle: format!("zone {}", zone.id),
        fields,
        sections,
    })
}

fn record_lines(records: Vec<rnaapi::endpoints::Record>) -> Vec<String> {
    cap(
        records
            .into_iter()
            .map(|record| {
                format!(
                    "{}  {}  {}",
                    record.name, record.record_type, record.content
                )
            })
            .collect(),
        12,
    )
}

fn ttl_text(ttl: &rnaapi::endpoints::TtlType) -> String {
    match ttl {
        rnaapi::endpoints::TtlType::StringKey(value) => value.clone(),
        rnaapi::endpoints::TtlType::IntegerKey(value) => value.to_string(),
    }
}

async fn load_account(client: &NaClient) -> Result<Pane, String> {
    let (details, invoices, images) = tokio::join!(
        Details::get(client),
        Invoices::list(client),
        Image::list(client),
    );
    let details = details.map_err(|err| err.to_string())?;
    let title = if details.fullname.is_empty() {
        "Account".to_string()
    } else {
        details.fullname.clone()
    };
    let fields = vec![
        Field::new("name", details.fullname),
        Field::new("email", details.email),
        Field::new("company", details.companyname),
        Field::new("address", details.address1),
        Field::new("city", details.city),
        Field::new("state", details.state),
        Field::new("postcode", details.postcode),
        Field::new("country", details.country),
        Field::new("phone", details.phonenumber),
        Field::new("status", details.status),
    ];
    let sections = vec![
        match invoices {
            Ok(rows) => section(
                "invoices",
                cap(
                    rows.into_iter()
                        .map(|invoice| {
                            format!("{}  {}", invoice.id, invoice.status)
                        })
                        .collect(),
                    8,
                ),
            ),
            Err(err) => section("invoices", vec![err.to_string()]),
        },
        match images {
            Ok(rows) => section(
                "images",
                cap(
                    rows.into_iter()
                        .map(|image| {
                            format!(
                                "{}  {}",
                                image.id,
                                image.os.unwrap_or_else(|| "image".to_string())
                            )
                        })
                        .collect(),
                    8,
                ),
            ),
            Err(err) => section("images", vec![err.to_string()]),
        },
    ];
    Ok(Pane {
        title,
        subtitle: "account".to_string(),
        fields,
        sections,
    })
}

fn section(title: &str, lines: Vec<String>) -> Section {
    Section {
        title: title.to_string(),
        lines,
    }
}

fn cap(lines: Vec<String>, max: usize) -> Vec<String> {
    let extra = lines.len().saturating_sub(max);
    let mut lines: Vec<_> = lines.into_iter().take(max).collect();
    if extra > 0 {
        lines.push(format!("and {extra} more"));
    }
    lines
}

fn fallback(value: &str, default: &str) -> String {
    if value.is_empty() {
        default.to_string()
    } else {
        value.to_string()
    }
}

fn compact_value(value: &Value) -> String {
    let text = match value {
        Value::Null => "ok".to_string(),
        Value::String(text) => text.clone(),
        other => other.to_string(),
    };
    let text = text.replace('\n', " ");
    text.chars().take(120).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Token;
    use serde_json::Value;

    fn server() -> Server {
        Server {
            city: "Example City".into(),
            fqdn: "web-01.example".into(),
            domu_package: 1,
            mbpkgid: 1001,
            os_id: 1,
            location_id: 3,
            ip: "192.0.2.10".into(),
            ipv6: String::new(),
            plan_id: 1,
            pkg_id: 1,
            state: "running".into(),
            status: "running".into(),
            uptime: "1".into(),
            installed: 1,
            nic1_mac: String::new(),
            nic2_mac: String::new(),
        }
    }

    fn metal(password: &str) -> MetalServer {
        MetalServer {
            id: 50,
            datacenter_id: 1,
            canceling: 0,
            mbpkgid: 1004,
            price: "1".into(),
            hostname: "metal-01.example".into(),
            eth0_mac: String::new(),
            eth1_mac: None,
            ipmi_mac: String::new(),
            primary_ip: "192.0.2.20".into(),
            primary_ipv6: None,
            nps_installed: 0,
            nps_os: String::new(),
            mb_model: None,
            cpu0_model: "cpu".into(),
            cpu1_model: None,
            total_ram: 1024,
            ipmi_pubip: String::new(),
            ipmi_cxuser: Some("root".into()),
            ipmi_cxpass: Some(password.into()),
            ipmi_status: 0,
            locked: 0,
            locked_msg: None,
            ipmi_status_time: String::new(),
            ob_id: Value::Null,
            info: String::new(),
            title: String::new(),
            ipmi_refresh_enabled: 0,
            location: "Example City".into(),
            ip_subnet_id: 1,
            ip_subnet_name: String::new(),
            package_status: "running".into(),
            building: Value::Null,
        }
    }

    #[test]
    fn kind_comes_from_which_list_returned_the_id() {
        let virtual_row = server_row(&server()).unwrap();
        assert_eq!(virtual_row.kind, ServerKind::Virtual);
        assert_eq!(virtual_row.mbpkgid, 1001);
        assert_eq!(virtual_row.location, "Example City");
        let dedicated = metal_row(&metal("secret-pass")).unwrap();
        assert_eq!(dedicated.kind, ServerKind::Dedicated);
        assert_eq!(dedicated.mbpkgid, 1004);
        assert_eq!(dedicated.metal_id, Some(50));
        let dumped = format!("{dedicated:?}");
        assert!(!dumped.contains("secret-pass"));
        assert!(
            metal_row(&{
                let mut row = metal("x");
                row.mbpkgid = -1;
                row
            })
            .is_none()
        );
    }

    #[test]
    fn server_pane_lines_use_the_fields_on_the_type() {
        let job = SrvJob {
            id: 7,
            ts_insert: "2024-01-02 03:04:05".into(),
            command: "reboot".into(),
            status: 2,
        };
        assert_eq!(job_line(&job), "7  2024-01-02 03:04:05  2  reboot");
        let v4 = IPv4 {
            id: 1,
            primary: 1,
            reverse: "web.example".into(),
            ip: "192.0.2.10".into(),
            netmask: "255.255.255.0".into(),
            gateway: "192.0.2.1".into(),
            broadcast: "192.0.2.255".into(),
        };
        assert_eq!(
            ipv4_line(&v4),
            "ipv4  192.0.2.10  255.255.255.0  192.0.2.1  web.example"
        );
        let v6 = IPv6 {
            id: 2,
            primary: 0,
            reverse: String::new(),
            ip: "2001:db8::10".into(),
            netmask: "64".into(),
            gateway: "2001:db8::1".into(),
            broadcast: String::new(),
        };
        assert_eq!(ipv6_line(&v6), "ipv6  2001:db8::10  64  2001:db8::1");
        let sample = MonthlyBw {
            date: "2024-03".into(),
            rx: 10,
            tx: 4,
        };
        assert_eq!(bandwidth_line(&sample), "2024-03  rx 10  tx 4");
    }

    #[test]
    fn a_token_round_trip_is_the_request_token() {
        let token = Token {
            tab: crate::focus::Tab::Servers,
            selection: 4,
            request_id: 9,
            slot: crate::focus::Slot::Power,
        };
        let request = Request::Power(PowerCommand {
            token,
            mbpkgid: 1001,
            kind: ServerKind::Virtual,
            verb: crate::model::PowerVerb::On,
            force: false,
        });
        assert_eq!(request.token(), token);
        assert_eq!(force_flag(false), None);
    }
}
