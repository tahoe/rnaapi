// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0

use std::sync::Arc;

use rnaapi::NaClient;
use rnaapi::endpoints::{
    ContractUsage, DedicatedAction, DeleteServer, Details, IPv4, IPv6, Image,
    Invoices, Location, MetalServer, MonthlyBw, Plan, Record, RecordBody,
    RecordUpdate, SSHKeys, Server, ServerBuild, ServerBuildRequest,
    ServerPower, Sizes, SrvJob, SrvStatus, SshKeyBody, Zone, ZoneBody,
};
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

use crate::focus::ServerPane;
use crate::model::LocationRow;
use crate::model::{
    BuildChoices, BuildFields, Choice, DetailQuery, DnsRecord, Field, Listed,
    Pane, PowerCommand, RecordFields, Reply, Request, Section, ServerBatch,
    ServerDetail, ServerKind, ServerRow, SshRow, UsageBilling, WriteOp,
    ZoneRow, ZoneView, force_flag, merge_servers, power_call_text,
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
        Request::Catalog(token) => Reply::Catalog {
            token,
            result: Ok(load_catalog(client).await),
        },
        Request::Write { token, op } => Reply::Write {
            token,
            result: run_write(client, op).await,
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
        state: fallback(
            server.state.as_deref().unwrap_or(""),
            server.status.as_deref().unwrap_or(""),
        ),
        ip: server.ip.clone(),
        location: server.city.clone(),
        status: server.status.clone().unwrap_or_default(),
        kind: ServerKind::Virtual,
        metal_id: None,
        package: plan_name(server.package.as_deref()),
        plan_id: Some(server.plan_id),
        pkg_id: Some(server.pkg_id),
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
        package: None,
        plan_id: None,
        pkg_id: None,
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
        plan_id: None,
        pkg_id: None,
    };
    match server {
        Ok(server) => {
            detail.fqdn = Some(server.fqdn);
            detail.plan_id = Some(server.plan_id);
            detail.pkg_id = Some(server.pkg_id);
            detail.state = server.state.filter(|text| !text.is_empty());
            detail.status = server.status.filter(|text| !text.is_empty());
            detail.ip = Some(server.ip);
            detail.location = Some(server.city);
            if let Some(uptime) = server.uptime.filter(|text| !text.is_empty())
            {
                detail.extras.push(Field::new("uptime", uptime));
            }
            if let Some(ipv6) = server.ipv6.filter(|text| !text.is_empty()) {
                detail.extras.push(Field::new("ipv6", ipv6));
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
            plan_id: None,
            pkg_id: None,
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
            plan_id: None,
            pkg_id: None,
        },
        Err(err) => ServerDetail {
            fqdn: None,
            state: None,
            status: None,
            ip: None,
            location: None,
            extras: Vec::new(),
            error: Some(err.to_string()),
            plan_id: None,
            pkg_id: None,
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

async fn run_write(client: &NaClient, op: WriteOp) -> Result<String, String> {
    match op {
        WriteOp::UpdateKey { id, name, material } => {
            update_ssh(client, id, name, material).await
        }
        WriteOp::DeleteKey { id, name } => {
            SSHKeys::delete(client, id)
                .await
                .map_err(|err| err.to_string())?;
            Ok(format!("ssh key deleted: {name}"))
        }
        WriteOp::CreateServer(fields) => {
            let fqdn = fields.fqdn.clone();
            let created = Server::buy(client, &build_request(&fields))
                .await
                .map_err(|err| err.to_string())?;
            Ok(server_build_line(
                "server created",
                fqdn.as_deref(),
                &created,
            ))
        }
        WriteOp::RebuildServer {
            mbpkgid,
            fqdn,
            fields,
        } => {
            // `Server::rebuild` takes the server id. This screen calls it mbpkgid.
            let created =
                Server::rebuild(client, mbpkgid, &build_request(&fields))
                    .await
                    .map_err(|err| err.to_string())?;
            Ok(server_build_line("server rebuilt", Some(&fqdn), &created))
        }
        WriteOp::DeleteServer { mbpkgid, fqdn } => {
            let body = DeleteServer::default();
            Server::delete(client, mbpkgid, &body)
                .await
                .map_err(|err| err.to_string())?;
            Ok(format!("server deleted: {fqdn} mbpkgid {mbpkgid}"))
        }
        WriteOp::CreateZone {
            name,
            zone_type,
            ip,
        } => {
            let zone = Zone::create(
                client,
                &ZoneBody {
                    name,
                    zone_type,
                    ip,
                },
            )
            .await
            .map_err(|err| err.to_string())?;
            Ok(format!("zone created: {}", zone.name))
        }
        WriteOp::DeleteZone { id, name } => {
            Zone::delete(client, id)
                .await
                .map_err(|err| err.to_string())?;
            Ok(format!("zone deleted: {name}"))
        }
        WriteOp::CreateRecord {
            domain_id,
            zone,
            fields,
        } => {
            let record =
                Record::create(client, &record_body(domain_id, &fields))
                    .await
                    .map_err(|err| err.to_string())?;
            Ok(format!(
                "record created: {} {} in {zone}",
                record.name, record.record_type
            ))
        }
        WriteOp::UpdateRecord {
            domain_id,
            zone,
            fields,
        } => {
            let id = fields.id.ok_or_else(|| "id is required".to_string())?;
            let record =
                Record::update(client, &record_update(id, domain_id, &fields))
                    .await
                    .map_err(|err| err.to_string())?;
            Ok(format!(
                "record updated: {} {} in {zone}",
                record.name, record.record_type
            ))
        }
        WriteOp::DeleteRecord { id, name, zone } => {
            Record::delete(client, id)
                .await
                .map_err(|err| err.to_string())?;
            Ok(format!("record deleted: {name} in {zone}"))
        }
    }
}

async fn update_ssh(
    client: &NaClient, id: u32, name: String, material: String,
) -> Result<String, String> {
    let ssh_key = tokio::task::spawn_blocking(move || {
        crate::ssh::public_key_material(&material)
    })
    .await
    .map_err(|_| "ssh key preparation stopped".to_string())?
    .map_err(|err| format!("{err:#}").replace('\n', ": "))?;
    let updated = SSHKeys::update(client, id, &SshKeyBody { name, ssh_key })
        .await
        .map_err(|err| err.to_string())?;
    Ok(format!("ssh key updated: {}", updated.name))
}

async fn load_catalog(client: &NaClient) -> BuildChoices {
    let (plans, images, locations, keys, contract) = tokio::join!(
        Plan::list(client),
        Image::list(client),
        Location::list(client),
        SSHKeys::list(client),
        ContractUsage::get(client),
    );
    let (billing, billing_error) = billing_from_contract_get(contract);
    BuildChoices {
        plans: listed(plans, plan_choice),
        images: listed(images, image_choice),
        locations: listed(locations, location_choice),
        keys: listed(keys, key_choice),
        billing,
        billing_error,
    }
}

/// `package_billing` is `contract_type`. `package_billing_contract_id` is `id`.
///
/// `GET cloud/contract/usage` returns one `ContractUsage`, so that row is the
/// only contract and is sent even when `is_free` is not 1. The type has no
/// `allow_create` field. Several free rows prefer the one `contract_type` of
/// "usage"; if that is still ambiguous, the first free row is used.
fn billing_from_contract_get(
    result: Result<ContractUsage, rnaapi::errors::NaApiError>,
) -> (Option<UsageBilling>, Option<String>) {
    match result {
        Ok(row) => select_usage_billing(std::slice::from_ref(&row)),
        Err(err) => (None, Some(err.to_string())),
    }
}

fn select_usage_billing(
    contracts: &[ContractUsage],
) -> (Option<UsageBilling>, Option<String>) {
    match contracts {
        [] => omitted("no usage contract was returned"),
        [only] => billing_from(only, None),
        many => select_among(many),
    }
}

fn select_among(
    contracts: &[ContractUsage],
) -> (Option<UsageBilling>, Option<String>) {
    let free = contracts
        .iter()
        .filter(|contract| contract.is_free == Some(1))
        .collect::<Vec<_>>();
    match free.as_slice() {
        [] => {
            omitted("is_free is not 1 and more than one contract was returned")
        }
        [only] => billing_from(only, None),
        [first, ..] => {
            let mut usage = free
                .iter()
                .copied()
                .filter(|contract| contract_type_is_usage(contract));
            match (usage.next(), usage.next()) {
                (Some(only), None) => billing_from(only, None),
                _ => billing_from(
                    first,
                    Some(
                        "several contracts have is_free 1; using the first"
                            .to_string(),
                    ),
                ),
            }
        }
    }
}

fn billing_from(
    contract: &ContractUsage, note: Option<String>,
) -> (Option<UsageBilling>, Option<String>) {
    let contract_type = contract
        .contract_type
        .as_deref()
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_string);
    let id = contract.id.map(|id| id.to_string());
    match (contract_type, id) {
        (Some(contract_type), Some(id)) => {
            (Some(UsageBilling { contract_type, id }), note)
        }
        (missing_type, missing_id) => {
            let message = match (missing_type.is_some(), missing_id.is_some()) {
                (false, false) => "contract_type and id are missing",
                (false, true) => "contract_type is missing",
                (true, false) => "id is missing",
                (true, true) => "contract billing is incomplete",
            };
            (None, Some(message.to_string()))
        }
    }
}

fn contract_type_is_usage(contract: &ContractUsage) -> bool {
    contract
        .contract_type
        .as_deref()
        .is_some_and(|kind| kind.trim() == "usage")
}

fn omitted(message: &str) -> (Option<UsageBilling>, Option<String>) {
    (None, Some(message.to_string()))
}

fn listed<T>(
    result: Result<Vec<T>, rnaapi::errors::NaApiError>,
    choice: impl Fn(&T) -> Choice,
) -> Listed {
    match result {
        Ok(rows) => {
            let mut options = rows.iter().map(choice).collect::<Vec<_>>();
            options.sort_by(|left, right| {
                left.label
                    .cmp(&right.label)
                    .then(left.value.cmp(&right.value))
            });
            Listed {
                options,
                error: None,
            }
        }
        Err(err) => Listed {
            options: Vec::new(),
            error: Some(err.to_string()),
        },
    }
}

/// `Server::buy` sends `plan` as the plan name from `Plan::list` (`cloud/sizes`).
fn plan_choice(plan: &Plan) -> Choice {
    let name = plan.plan.trim();
    let value = if name.is_empty() {
        plan.plan_id.to_string()
    } else {
        name.to_string()
    };
    let label = if plan.ram.trim().is_empty() {
        value.clone()
    } else {
        format!("{value}  ram {}", plan.ram.trim())
    };
    Choice { label, value }
}

fn image_choice(image: &Image) -> Choice {
    let name = image
        .os
        .as_deref()
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .or_else(|| {
            image
                .description
                .as_deref()
                .map(str::trim)
                .filter(|text| !text.is_empty())
        })
        .unwrap_or("image");
    Choice {
        label: name.to_string(),
        value: image.id.to_string(),
    }
}

fn location_choice(row: &Location) -> Choice {
    let name = row.name.trim();
    let code = row.iata_code.trim();
    let label = match (name.is_empty(), code.is_empty()) {
        (false, false) => format!("{name}  {code}"),
        (false, true) => name.to_string(),
        (true, false) => code.to_string(),
        (true, true) => format!("location {}", row.id),
    };
    Choice {
        label,
        value: row.id.to_string(),
    }
}

fn key_choice(key: &SSHKeys) -> Choice {
    let name = key.name.trim();
    let label = if name.is_empty() {
        "ssh key".to_string()
    } else {
        name.to_string()
    };
    Choice {
        label,
        value: key.id.to_string(),
    }
}

fn build_request(fields: &BuildFields) -> ServerBuildRequest {
    ServerBuildRequest {
        plan: fields.plan.clone(),
        location: fields.location,
        image: fields.image,
        fqdn: fields.fqdn.clone(),
        ssh_key_id: fields.ssh_key_id,
        package_billing: fields.package_billing.clone(),
        package_billing_contract_id: fields.package_billing_contract_id.clone(),
        ..ServerBuildRequest::default()
    }
}

fn server_build_line(
    verb: &str, fqdn: Option<&str>, built: &ServerBuild,
) -> String {
    match fqdn {
        Some(name) if !name.is_empty() => {
            format!("{verb}: {name} mbpkgid {}", built.mbpkgid)
        }
        _ => format!("{verb}: mbpkgid {}", built.mbpkgid),
    }
}

fn record_body(domain_id: i64, fields: &RecordFields) -> RecordBody {
    RecordBody {
        domain_id,
        name: fields.name.clone(),
        record_type: fields.record_type.clone(),
        record_content: fields.content.clone(),
        ttl: fields.ttl,
        prio: fields.prio,
    }
}

fn record_update(
    id: i64, domain_id: i64, fields: &RecordFields,
) -> RecordUpdate {
    RecordUpdate {
        id,
        domain_id,
        name: fields.name.clone(),
        record_type: fields.record_type.clone(),
        record_content: fields.content.clone(),
        ttl: fields.ttl,
        prio: fields.prio,
    }
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

async fn load_zone(client: &NaClient, id: u32) -> Result<ZoneView, String> {
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
    let records = zone
        .records
        .unwrap_or_default()
        .into_iter()
        .map(dns_record)
        .collect::<Vec<_>>();
    let mut sections = Vec::new();
    if !records.is_empty() {
        sections.push(section("records", record_lines(&records)));
    }
    if let Some(ns) = zone.ns {
        sections.push(section(
            "ns",
            record_lines(&ns.into_iter().map(dns_record).collect::<Vec<_>>()),
        ));
    }
    Ok(ZoneView {
        pane: Pane {
            title: zone.name,
            subtitle: format!("zone {}", zone.id),
            fields,
            sections,
        },
        records,
    })
}

fn dns_record(record: Record) -> DnsRecord {
    DnsRecord {
        id: record.id,
        name: record.name,
        record_type: record.record_type,
        content: record.content,
        ttl: record.ttl,
        prio: record.prio,
    }
}

fn record_lines(records: &[DnsRecord]) -> Vec<String> {
    cap(
        records
            .iter()
            .map(|record| {
                format!(
                    "{}  {}  {}  {}",
                    record.id, record.name, record.record_type, record.content
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

/// Plan name from the server list. Blank text is omitted.
fn plan_name(value: Option<&str>) -> Option<String> {
    let text = value?.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
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
            ipv6: Some(String::new()),
            plan_id: 1,
            pkg_id: 1,
            state: Some("running".into()),
            status: Some("running".into()),
            uptime: Some("1".into()),
            installed: 1,
            nic1_mac: Some(String::new()),
            nic2_mac: Some(String::new()),
            package: None,
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
        assert!(virtual_row.package.is_none());
        assert_eq!(virtual_row.plan_id, Some(1));
        assert_eq!(virtual_row.pkg_id, Some(1));
        let mut named = server();
        named.package = Some(" VR4x1x25 ".into());
        assert_eq!(
            server_row(&named).unwrap().package.as_deref(),
            Some("VR4x1x25")
        );
        let dedicated = metal_row(&metal("secret-pass")).unwrap();
        assert_eq!(dedicated.kind, ServerKind::Dedicated);
        assert_eq!(dedicated.mbpkgid, 1004);
        assert_eq!(dedicated.metal_id, Some(50));
        assert!(dedicated.package.is_none());
        assert!(dedicated.plan_id.is_none());
        assert!(dedicated.pkg_id.is_none());
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
    fn a_server_delete_body_is_empty() {
        let value = serde_json::to_value(DeleteServer::default()).unwrap();
        assert_eq!(value, serde_json::json!({}));
    }

    #[test]
    fn a_server_build_request_keeps_only_the_filled_fields() {
        let body = super::build_request(&BuildFields {
            fqdn: Some("web.example".into()),
            plan: Some("plan-1".into()),
            location: Some(3),
            image: None,
            ssh_key_id: Some(9),
            package_billing: None,
            package_billing_contract_id: None,
        });
        assert_eq!(body.fqdn.as_deref(), Some("web.example"));
        assert_eq!(body.plan.as_deref(), Some("plan-1"));
        assert_eq!(body.location, Some(3));
        assert!(body.image.is_none());
        assert_eq!(body.ssh_key_id, Some(9));
        assert!(body.password.is_none());
        assert!(body.ssh_key.is_none());
        assert!(body.package_billing.is_none());
        assert!(body.package_billing_contract_id.is_none());

        let billed = super::build_request(&BuildFields {
            fqdn: Some("web.example".into()),
            plan: Some("plan-1".into()),
            location: Some(3),
            image: Some(4),
            ssh_key_id: Some(9),
            package_billing: Some("metered".into()),
            package_billing_contract_id: Some("7".into()),
        });
        assert_eq!(billed.package_billing.as_deref(), Some("metered"));
        assert_eq!(billed.package_billing_contract_id.as_deref(), Some("7"));
        assert!(billed.ssh_key.is_none());
        assert!(billed.cloud_pool_id.is_none());
        assert!(billed.vpc_id.is_none());
        let value = serde_json::to_value(&billed).unwrap();
        assert!(value["package_billing_contract_id"].is_string());
        assert!(value.get("ssh_key").is_none());
        assert!(value.get("cloud_pool_id").is_none());
    }

    #[test]
    fn the_only_usage_contract_copies_contract_type_and_id() {
        let (billing, message) = super::select_usage_billing(&[contract(
            Some(7),
            Some(" metered "),
            Some(1),
            Some(40),
        )]);
        let billing = billing.expect("free contract");
        assert_eq!(billing.contract_type, "metered");
        assert_eq!(billing.id, "7");
        assert_ne!(billing.id, "40");
        assert!(message.is_none());

        let (billing, message) = super::select_usage_billing(&[contract(
            Some(8),
            Some("hourly"),
            Some(0),
            Some(41),
        )]);
        let billing = billing.expect("only contract");
        assert_eq!(billing.contract_type, "hourly");
        assert_eq!(billing.id, "8");
        assert_ne!(billing.id, "41");
        assert!(message.is_none());

        let (billing, message) = super::select_usage_billing(&[contract(
            Some(9),
            Some("hourly"),
            None,
            Some(42),
        )]);
        assert_eq!(billing.expect("unmarked contract").id, "9");
        assert!(message.is_none());

        let (billing, message) = super::select_usage_billing(&[contract(
            None,
            Some("hourly"),
            Some(1),
            Some(40),
        )]);
        assert!(billing.is_none());
        assert_eq!(message.as_deref(), Some("id is missing"));
        assert!(!message.unwrap().contains("40"));

        let (billing, message) = super::select_usage_billing(&[contract(
            Some(7),
            Some("  "),
            Some(1),
            Some(40),
        )]);
        assert!(billing.is_none());
        assert_eq!(message.as_deref(), Some("contract_type is missing"));
    }

    #[test]
    fn several_usage_contracts_follow_the_free_contract_rules() {
        let (billing, message) = super::select_usage_billing(&[
            contract(Some(15), Some("monthly"), Some(0), Some(40)),
            contract(Some(16), Some("hourly"), Some(1), Some(41)),
        ]);
        let billing = billing.expect("one free contract");
        assert_eq!(billing.contract_type, "hourly");
        assert_eq!(billing.id, "16");
        assert_ne!(billing.id, "41");
        assert!(message.is_none());

        let (billing, message) = super::select_usage_billing(&[
            contract(Some(15), Some("monthly"), Some(1), None),
            contract(Some(16), Some(" usage "), Some(1), Some(41)),
            contract(Some(21), Some("usage"), Some(0), None),
        ]);
        let billing = billing.expect("single usage type");
        assert_eq!(billing.contract_type, "usage");
        assert_eq!(billing.id, "16");
        assert!(message.is_none());

        let (billing, message) = super::select_usage_billing(&[
            contract(Some(15), Some("monthly"), Some(1), None),
            contract(Some(16), Some("hourly"), Some(1), None),
        ]);
        assert_eq!(billing.expect("first free").id, "15");
        let message = message.expect("ambiguous");
        assert!(message.contains("first"), "{message}");

        let (billing, message) = super::select_usage_billing(&[
            contract(Some(15), Some("usage"), Some(1), None),
            contract(Some(16), Some("usage"), Some(1), None),
        ]);
        assert_eq!(billing.expect("first usage").id, "15");
        assert!(message.expect("ambiguous").contains("first"));

        let (billing, message) = super::select_usage_billing(&[
            contract(Some(15), Some("usage"), Some(0), Some(40)),
            contract(Some(16), Some("hourly"), None, Some(41)),
        ]);
        assert!(billing.is_none());
        let message = message.expect("omitted");
        assert!(message.contains("is_free"), "{message}");
        assert!(!message.contains("40"));
        assert!(!message.contains("41"));

        let (billing, message) = super::select_usage_billing(&[]);
        assert!(billing.is_none());
        assert!(message.expect("empty").contains("no usage contract"));
    }

    #[test]
    fn a_failed_contract_call_does_not_invent_billing() {
        let (billing, message) = super::billing_from_contract_get(Err(
            rnaapi::errors::NaApiError::Api("connection failed".into()),
        ));
        assert!(billing.is_none());
        assert_eq!(message.as_deref(), Some("connection failed"));
    }

    fn contract(
        id: Option<i64>, contract_type: Option<&str>, is_free: Option<i64>,
        contract_mbpkgid: Option<i64>,
    ) -> ContractUsage {
        ContractUsage {
            id,
            contract_mbpkgid,
            parent_contract_id: None,
            brand: None,
            mb_id: None,
            contract_type: contract_type.map(str::to_string),
            is_free,
            include_bandwidth: None,
            customer_po: None,
            customer_description: None,
            po_monthly_limit: None,
            monthly_discount: None,
            hourly_discount: None,
            max_cpus: None,
            max_ram: None,
            max_disk: None,
            allow_overage: None,
        }
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

    #[test]
    fn build_choices_show_a_name_and_keep_the_id_buy_sends() {
        let plan = super::plan_choice(&Plan {
            plan_id: 7,
            plan: "VR1x2".into(),
            ram: "2048".into(),
            disk: "40".into(),
            transfer: "1".into(),
            price: "2".into(),
            available: 1.0,
        });
        assert_eq!(plan.value, "VR1x2");
        assert!(plan.label.contains("VR1x2"), "{}", plan.label);
        assert!(plan.label.contains("2048"), "{}", plan.label);
        assert!(!plan.value.contains("2048"));

        let image = super::image_choice(&Image {
            id: 55,
            os: Some("Ubuntu 24.04".into()),
            description: Some("linux".into()),
            size: None,
            subtype: None,
            created: None,
            category: None,
            updated: None,
            iso: None,
            bits: None,
            tech: None,
            icon: None,
            private: None,
        });
        assert_eq!(image.label, "Ubuntu 24.04");
        assert_eq!(image.value, "55");

        let location = super::location_choice(&Location {
            id: 42,
            name: "Ashburn".into(),
            iata_code: "IAD".into(),
            continent: "NA".into(),
            flag: "us".into(),
            latitude: "0".into(),
            longitude: "0".into(),
            disabled: 0,
            status: Vec::new(),
        });
        assert_eq!(location.label, "Ashburn  IAD");
        assert_eq!(location.value, "42");

        let key = super::key_choice(&SSHKeys {
            id: 12,
            ssh_key: "ssh-ed25519 AAAA".into(),
            fingerprint: "SHA256:ab".into(),
            name: "laptop".into(),
        });
        assert_eq!(key.label, "laptop");
        assert_eq!(key.value, "12");
    }
}
