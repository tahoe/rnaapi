// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0

use crate::focus::{
    Action, DetailControl, Focus, PopupFocus, PromptFocus, ServerPane, Slot,
    Split, Tab, order, starting_focus, step, step_popup, step_prompt,
};

/// List tokens store the selection generation. Detail tokens store the row id.
/// A response is stale when that value, the tab, or the request id no longer matches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub tab: Tab,
    pub selection: u64,
    pub request_id: u64,
    pub slot: Slot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServerKind {
    Virtual,
    Dedicated,
}

impl ServerKind {
    pub fn label(self) -> &'static str {
        match self {
            ServerKind::Virtual => "virtual",
            ServerKind::Dedicated => "dedicated",
        }
    }

    pub fn command(self) -> &'static str {
        match self {
            ServerKind::Virtual => "power-virt",
            ServerKind::Dedicated => "power-ded",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerVerb {
    On,
    Off,
    Reboot,
}

impl PowerVerb {
    pub fn command(self) -> &'static str {
        match self {
            PowerVerb::On => "on",
            PowerVerb::Off => "off",
            PowerVerb::Reboot => "reboot",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            PowerVerb::On => "Power on",
            PowerVerb::Off => "Power off",
            PowerVerb::Reboot => "Reboot server",
        }
    }

    fn control(self) -> DetailControl {
        match self {
            PowerVerb::On => DetailControl::On,
            PowerVerb::Off => DetailControl::Off,
            PowerVerb::Reboot => DetailControl::Reboot,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerRow {
    pub mbpkgid: u32,
    pub fqdn: String,
    pub state: String,
    pub ip: String,
    pub location: String,
    pub status: String,
    pub kind: ServerKind,
    pub metal_id: Option<u32>,
    /// Plan name from the virtual server list, such as `VR4x1x25`.
    pub package: Option<String>,
    /// `Server::plan_id` from the virtual server list.
    /// Delete sends this value as `plan_id` and as `domU_package`.
    pub plan_id: Option<u32>,
    /// `Server::pkg_id` from the virtual server list.
    /// Delete sends this value as the string `pkg_id`.
    pub pkg_id: Option<u32>,
}

impl ServerRow {
    #[cfg(test)]
    pub fn listed(
        mbpkgid: u32, fqdn: &str, state: &str, kind: ServerKind,
    ) -> Self {
        Self {
            mbpkgid,
            fqdn: fqdn.to_string(),
            state: state.to_string(),
            ip: "192.0.2.10".to_string(),
            location: "Example City".to_string(),
            status: state.to_string(),
            kind,
            metal_id: None,
            package: None,
            plan_id: None,
            pkg_id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SshRow {
    pub id: u32,
    pub name: String,
    pub fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocationRow {
    pub id: u32,
    pub name: String,
    pub continent: String,
    pub iata: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneRow {
    pub id: u32,
    pub name: String,
    pub zone_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub label: String,
    pub value: String,
}

impl Field {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub title: String,
    pub lines: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pane {
    pub title: String,
    pub subtitle: String,
    pub fields: Vec<Field>,
    pub sections: Vec<Section>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerBatch {
    pub rows: Vec<ServerRow>,
    pub warning: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerDetail {
    pub fqdn: Option<String>,
    pub state: Option<String>,
    pub status: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub extras: Vec<Field>,
    pub error: Option<String>,
    /// `Server::plan_id` when this detail was already loaded.
    pub plan_id: Option<u32>,
    /// `Server::pkg_id` when this detail was already loaded.
    pub pkg_id: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PowerPopup {
    pub session: u64,
    pub verb: PowerVerb,
    pub mbpkgid: u32,
    pub fqdn: String,
    pub kind: ServerKind,
    pub force: bool,
    pub focus: PopupFocus,
    pub return_focus: Focus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Choice {
    pub label: String,
    pub value: String,
}

/// One list behind a create-server dropdown. An error leaves that dropdown empty.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Listed {
    pub options: Vec<Choice>,
    pub error: Option<String>,
}

/// The usage contract copied onto `Server::buy`.
/// `contract_type` becomes `package_billing`. `id` becomes
/// `package_billing_contract_id`. This is not `contract_mbpkgid`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsageBilling {
    pub contract_type: String,
    pub id: String,
}

/// Plans, images, locations, SSH keys, and the usage contract for `Server::buy`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildChoices {
    pub plans: Listed,
    pub images: Listed,
    pub locations: Listed,
    pub keys: Listed,
    pub billing: Option<UsageBilling>,
    pub billing_error: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoicePhase {
    Loading,
    Ready,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldKind {
    Text,
    Choice {
        options: Vec<Choice>,
        picked: Option<usize>,
        phase: ChoicePhase,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PromptField {
    pub label: String,
    pub value: String,
    pub kind: FieldKind,
}

impl PromptField {
    /// Text the form shows. A dropdown shows its label, not the id it will send.
    pub fn shown(&self) -> String {
        match &self.kind {
            FieldKind::Text => self.value.clone(),
            FieldKind::Choice {
                options,
                picked,
                phase,
            } => match phase {
                ChoicePhase::Loading => "loading".to_string(),
                ChoicePhase::Failed => "unavailable".to_string(),
                ChoicePhase::Ready => {
                    if let Some(index) = *picked {
                        options
                            .get(index)
                            .map(|choice| choice.label.clone())
                            .unwrap_or_else(|| "choose".to_string())
                    } else if options.is_empty() {
                        "none".to_string()
                    } else {
                        "choose".to_string()
                    }
                }
            },
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self.kind, FieldKind::Text)
    }
}

/// A one-shot prompt. A confirm has no fields. A form has a few text fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prompt {
    pub title: String,
    pub hint: String,
    pub fields: Vec<PromptField>,
    pub focus: PromptFocus,
    pub submit_label: String,
    pub kind: PromptKind,
    pub return_focus: Focus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PromptKind {
    NewKey,
    UpdateKey {
        id: u32,
    },
    DeleteKey {
        id: u32,
        name: String,
    },
    CreateServer,
    RebuildServer {
        mbpkgid: u32,
        fqdn: String,
    },
    DeleteServer {
        mbpkgid: u32,
        fqdn: String,
    },
    CreateZone,
    DeleteZone {
        id: u32,
        name: String,
    },
    CreateRecord {
        domain_id: i64,
        zone: String,
    },
    UpdateRecord {
        domain_id: i64,
        zone: String,
    },
    /// The id is collected first, then the prompt becomes a named confirm.
    DeleteRecord {
        zone: String,
    },
    ConfirmDeleteRecord {
        id: u32,
        name: String,
        zone: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildFields {
    pub fqdn: Option<String>,
    pub plan: Option<String>,
    pub location: Option<i64>,
    pub image: Option<i64>,
    pub ssh_key_id: Option<i64>,
    pub package_billing: Option<String>,
    pub package_billing_contract_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordFields {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub record_type: String,
    pub content: String,
    pub ttl: Option<i64>,
    pub prio: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DnsRecord {
    pub id: u32,
    pub name: String,
    pub record_type: String,
    pub content: String,
    pub ttl: Option<u32>,
    pub prio: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneView {
    pub pane: Pane,
    pub records: Vec<DnsRecord>,
}

/// A write that already has the values the library body needs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriteOp {
    UpdateKey {
        id: u32,
        name: String,
        material: String,
    },
    DeleteKey {
        id: u32,
        name: String,
    },
    CreateServer(BuildFields),
    RebuildServer {
        mbpkgid: u32,
        fqdn: String,
        fields: BuildFields,
    },
    DeleteServer {
        mbpkgid: u32,
        fqdn: String,
    },
    CreateZone {
        name: String,
        zone_type: String,
        ip: Option<String>,
    },
    DeleteZone {
        id: u32,
        name: String,
    },
    CreateRecord {
        domain_id: i64,
        zone: String,
        fields: RecordFields,
    },
    UpdateRecord {
        domain_id: i64,
        zone: String,
        fields: RecordFields,
    },
    DeleteRecord {
        id: u32,
        name: String,
        zone: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Ok,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Banner {
    pub level: Level,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PowerCommand {
    pub token: Token,
    pub mbpkgid: u32,
    pub kind: ServerKind,
    pub verb: PowerVerb,
    pub force: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetailQuery {
    pub token: Token,
    pub kind: ServerKind,
    pub mbpkgid: u32,
    pub metal_id: Option<u32>,
}

/// Jobs, addresses, or bandwidth for one server.
/// The token stores that server's id, same as a detail token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerListQuery {
    pub token: Token,
    pub pane: ServerPane,
    pub mbpkgid: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Servers(Token),
    ServerDetail(DetailQuery),
    ServerList(ServerListQuery),
    Power(PowerCommand),
    Ssh(Token),
    SshCreate {
        token: Token,
        file: Option<String>,
    },
    Locations(Token),
    LocationDetail {
        token: Token,
        id: u32,
    },
    Zones(Token),
    ZoneDetail {
        token: Token,
        id: u32,
    },
    Account(Token),
    Write {
        token: Token,
        op: WriteOp,
    },
    /// Plans, images, locations, and SSH keys for the open build form.
    Catalog(Token),
}

impl Request {
    pub fn token(&self) -> Token {
        match self {
            Request::Servers(token)
            | Request::Ssh(token)
            | Request::Locations(token)
            | Request::Zones(token)
            | Request::Account(token) => *token,
            Request::ServerDetail(query) => query.token,
            Request::ServerList(query) => query.token,
            Request::Power(command) => command.token,
            Request::SshCreate { token, .. }
            | Request::LocationDetail { token, .. }
            | Request::ZoneDetail { token, .. }
            | Request::Write { token, .. }
            | Request::Catalog(token) => *token,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Servers {
        token: Token,
        result: Result<ServerBatch, String>,
    },
    ServerDetail {
        token: Token,
        result: Result<ServerDetail, String>,
    },
    ServerList {
        token: Token,
        pane: ServerPane,
        lines: Vec<String>,
        errors: Vec<String>,
    },
    Power {
        token: Token,
        result: Result<String, String>,
    },
    Ssh {
        token: Token,
        result: Result<Vec<SshRow>, String>,
    },
    SshCreated {
        token: Token,
        result: Result<String, String>,
    },
    Locations {
        token: Token,
        result: Result<Vec<LocationRow>, String>,
    },
    LocationDetail {
        token: Token,
        result: Result<Pane, String>,
    },
    Zones {
        token: Token,
        result: Result<Vec<ZoneRow>, String>,
    },
    ZoneDetail {
        token: Token,
        result: Result<ZoneView, String>,
    },
    Write {
        token: Token,
        result: Result<String, String>,
    },
    Account {
        token: Token,
        result: Result<Pane, String>,
    },
    Catalog {
        token: Token,
        result: Result<BuildChoices, String>,
    },
}

#[derive(Clone, Copy, Debug, Default)]
struct Track {
    latest: u64,
    loading: bool,
    /// A background server refresh is in flight and should not write the log.
    silent: bool,
}

pub struct App {
    pub host: String,
    pub tab: Tab,
    pub focus: Focus,
    /// The open tab keeps the focused style while the startup outline is in
    /// place. A key or a click clears it. Refreshes do not.
    startup_outline: bool,
    /// The outline Tab returns to after a divider has been focused.
    split_return: Focus,
    pub force: bool,
    pub servers: Vec<ServerRow>,
    pub keys: Vec<SshRow>,
    pub locations: Vec<LocationRow>,
    pub zones: Vec<ZoneRow>,
    pub account: Option<Pane>,
    pub server_detail_for: Option<u64>,
    pub server_extras: Vec<Field>,
    pub server_pane: ServerPane,
    pub server_lines: Vec<String>,
    pub server_lines_for: Option<u64>,
    pub server_lines_pane: ServerPane,
    pub server_line: usize,
    pub server_line_window: u16,
    pub location_detail: Option<(u64, Pane)>,
    pub zone_detail: Option<(u64, Pane)>,
    /// Records from the open zone, so a delete can name one.
    pub zone_records: Vec<DnsRecord>,
    pub banner: Option<Banner>,
    log: Vec<Banner>,
    pub power: Option<PowerPopup>,
    pub prompt: Option<Prompt>,
    pub list_scroll: usize,
    pub detail_scroll: u16,
    pub list_window: u16,
    accepting: bool,
    next_id: u64,
    selection_gen: u64,
    servers_selected: Option<u64>,
    pub(crate) ssh_selected: Option<u64>,
    location_selected: Option<u64>,
    pub(crate) zone_selected: Option<u64>,
    power_inflight: Option<u64>,
    power_accepted: Option<u64>,
    power_revoked: Option<u64>,
    /// Set from `ContractUsage` when the create-server catalog returns.
    usage_billing: Option<UsageBilling>,
    tracks: [Track; TRACKS],
    pending: Vec<Request>,
}

impl App {
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            tab: Tab::Servers,
            focus: starting_focus(Tab::Servers, 0, None),
            startup_outline: true,
            split_return: Focus::Tab(Tab::Servers),
            force: false,
            servers: Vec::new(),
            keys: Vec::new(),
            locations: Vec::new(),
            zones: Vec::new(),
            account: None,
            server_detail_for: None,
            server_extras: Vec::new(),
            server_pane: ServerPane::Jobs,
            server_lines: Vec::new(),
            server_lines_for: None,
            server_lines_pane: ServerPane::Jobs,
            server_line: 0,
            server_line_window: 8,
            location_detail: None,
            zone_detail: None,
            zone_records: Vec::new(),
            banner: None,
            log: Vec::new(),
            power: None,
            prompt: None,
            list_scroll: 0,
            detail_scroll: 0,
            list_window: 8,
            accepting: true,
            next_id: 0,
            selection_gen: 0,
            servers_selected: None,
            ssh_selected: None,
            location_selected: None,
            zone_selected: None,
            power_inflight: None,
            power_accepted: None,
            power_revoked: None,
            usage_billing: None,
            tracks: [Track::default(); TRACKS],
            pending: Vec::new(),
        }
    }

    /// The open tab is still part of the startup outline, beside a list row.
    pub fn startup_tab_focused(&self, tab: Tab) -> bool {
        self.startup_outline
            && self.tab == tab
            && matches!(self.focus, Focus::Row(_))
    }

    fn settle_focus(&mut self) {
        if !self.startup_outline || self.modal() != crate::focus::Modal::None {
            return;
        }
        self.focus =
            starting_focus(self.tab, self.row_count(), self.selected_index());
    }

    fn leave_startup_outline(&mut self) {
        self.startup_outline = false;
    }

    pub fn boot(&mut self) {
        self.queue_list();
    }

    /// Re-fetch the open server list. Its detail is fetched when the list arrives.
    /// A fetch already running is left alone, and other tabs are not touched.
    /// Success does not add a status line.
    pub fn refresh_servers(&mut self) {
        if !self.accepting || self.tab != Tab::Servers {
            return;
        }
        if self.server_fetch_busy() {
            return;
        }
        let token = self.issue(Slot::Servers, self.selection_gen);
        self.mark_silent(Slot::Servers);
        self.pending.push(Request::Servers(token));
    }

    fn server_fetch_busy(&self) -> bool {
        [Slot::Servers, Slot::ServerDetail].into_iter().any(|slot| {
            let track = self.track(slot);
            track.loading || track.silent
        })
    }

    fn mark_silent(&mut self, slot: Slot) {
        let track = self.track_mut(slot);
        track.silent = true;
        track.loading = false;
    }

    #[cfg(test)]
    pub fn set_servers(&mut self, rows: Vec<ServerRow>, selected: u32) {
        self.servers_selected = Some(u64::from(selected));
        self.servers = rows;
        self.settle_focus();
    }

    #[cfg(test)]
    pub fn set_keys(&mut self, rows: Vec<SshRow>, selected: Option<u32>) {
        self.keys = rows;
        self.ssh_selected = selected.map(u64::from);
    }

    #[cfg(test)]
    pub fn set_zones(&mut self, rows: Vec<ZoneRow>, selected: Option<u32>) {
        self.zones = rows;
        self.zone_selected = selected.map(u64::from);
    }

    pub fn stop_accepting(&mut self) {
        self.accepting = false;
    }

    pub fn drain_requests(&mut self) -> Vec<Request> {
        std::mem::take(&mut self.pending)
    }

    pub fn modal(&self) -> crate::focus::Modal {
        if self.power.is_some() {
            crate::focus::Modal::Power
        } else if self.prompt.is_some() {
            crate::focus::Modal::Prompt
        } else {
            crate::focus::Modal::None
        }
    }

    pub fn typing(&self) -> bool {
        self.prompt.as_ref().is_some_and(|popup| {
            matches!(popup.focus, PromptFocus::Field(index)
                if popup.fields.get(index).is_some_and(|field| field.is_text()))
        })
    }

    pub fn row_count(&self) -> usize {
        match self.tab {
            Tab::Servers => self.servers.len(),
            Tab::SshKeys => self.keys.len(),
            Tab::Locations => self.locations.len(),
            Tab::Dns => self.zones.len(),
            Tab::Account => 1,
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        match self.tab {
            Tab::Account => Some(0),
            Tab::Servers => {
                position(self.servers_selected, &self.servers, |row| {
                    u64::from(row.mbpkgid)
                })
            }
            Tab::SshKeys => {
                position(self.ssh_selected, &self.keys, |row| u64::from(row.id))
            }
            Tab::Locations => {
                position(self.location_selected, &self.locations, |row| {
                    u64::from(row.id)
                })
            }
            Tab::Dns => position(self.zone_selected, &self.zones, |row| {
                u64::from(row.id)
            }),
        }
    }

    pub fn selected_server(&self) -> Option<&ServerRow> {
        let id = self.servers_selected?;
        self.servers.iter().find(|row| u64::from(row.mbpkgid) == id)
    }

    pub fn selected_key(&self) -> Option<&SshRow> {
        let id = self.ssh_selected?;
        self.keys.iter().find(|row| u64::from(row.id) == id)
    }

    pub fn selected_location(&self) -> Option<&LocationRow> {
        let id = self.location_selected?;
        self.locations.iter().find(|row| u64::from(row.id) == id)
    }

    pub fn selected_zone(&self) -> Option<&ZoneRow> {
        let id = self.zone_selected?;
        self.zones.iter().find(|row| u64::from(row.id) == id)
    }

    pub fn account_label(&self) -> &str {
        self.account
            .as_ref()
            .map(|pane| pane.title.as_str())
            .filter(|title| !title.is_empty())
            .unwrap_or("Account")
    }

    pub fn is_loading(&self, slot: Slot) -> bool {
        self.track(slot).loading
    }

    pub fn activity(&self) -> Option<&'static str> {
        if self.power_inflight.is_some() {
            return Some("Power change in progress");
        }
        if self.is_loading(Slot::Write) {
            return Some("Saving");
        }
        if self.is_loading(Slot::Catalog) && self.prompt.is_some() {
            return Some("Loading choices");
        }
        let slot = match self.tab {
            Tab::Servers if self.is_loading(Slot::Servers) => Slot::Servers,
            Tab::Servers if self.is_loading(Slot::ServerDetail) => {
                Slot::ServerDetail
            }
            Tab::Servers if self.is_loading(Slot::ServerList) => {
                return Some(self.server_pane.loading());
            }
            Tab::SshKeys if self.is_loading(Slot::SshCreate) => Slot::SshCreate,
            Tab::SshKeys if self.is_loading(Slot::Ssh) => Slot::Ssh,
            Tab::Locations if self.is_loading(Slot::Locations) => {
                Slot::Locations
            }
            Tab::Locations if self.is_loading(Slot::LocationDetail) => {
                Slot::LocationDetail
            }
            Tab::Dns if self.is_loading(Slot::Zones) => Slot::Zones,
            Tab::Dns if self.is_loading(Slot::ZoneDetail) => Slot::ZoneDetail,
            Tab::Account if self.is_loading(Slot::Account) => Slot::Account,
            _ => return None,
        };
        Some(match slot {
            Slot::Servers => "Loading servers",
            Slot::ServerDetail => "Loading server",
            Slot::Ssh => "Loading SSH keys",
            Slot::SshCreate => "Creating SSH key",
            Slot::Locations => "Loading locations",
            Slot::LocationDetail => "Loading location",
            Slot::Zones => "Loading DNS",
            Slot::ZoneDetail => "Loading zone",
            Slot::Account => "Loading account",
            Slot::Power => "Power change in progress",
            Slot::ServerList => "Loading jobs",
            Slot::Write => "Saving",
            Slot::Catalog => "Loading choices",
        })
    }

    pub fn focus_move(&mut self, backward: bool) {
        self.leave_startup_outline();
        if let Some(popup) = self.power.as_mut() {
            popup.focus = step_popup(popup.focus, backward);
            return;
        }
        if let Some(popup) = self.prompt.as_mut() {
            popup.focus =
                step_prompt(popup.fields.len(), popup.focus, backward);
            return;
        }
        // A divider is outside the Tab cycle. Resume from the control that
        // was outlined before it, then take one step as usual.
        if matches!(self.focus, Focus::Split(_)) {
            self.focus = self.split_return;
        }
        let items =
            order(self.tab, self.row_count(), self.selected_index().is_some());
        self.focus = step(&items, self.focus, backward);
    }

    /// Outline a divider without inserting it into the Tab cycle.
    pub fn focus_split(&mut self, split: Split) {
        if self.modal() != crate::focus::Modal::None {
            return;
        }
        self.leave_startup_outline();
        if !matches!(self.focus, Focus::Split(_)) {
            self.split_return = self.focus;
        }
        self.focus = Focus::Split(split);
    }

    pub fn activate_focused(&mut self) {
        self.leave_startup_outline();
        if let Some(focus) = self.power.as_ref().map(|popup| popup.focus) {
            self.activate_power(focus);
            return;
        }
        if let Some(focus) = self.prompt.as_ref().map(|popup| popup.focus) {
            // Enter on a field moves to the next one, and submits on the last.
            // A click only focuses the field.
            if matches!(focus, PromptFocus::Field(_)) {
                self.advance_prompt_field();
            } else {
                self.activate_prompt(focus);
            }
            return;
        }
        match self.focus {
            Focus::Tab(tab) => self.activate_tab(tab),
            Focus::Action(action) => self.open_action(action),
            Focus::Row(index) => self.select_index(index),
            Focus::Detail(control) => self.activate_detail(control),
            Focus::ServerPane(pane) => self.activate_server_pane(pane),
            Focus::Split(_) => {}
        }
    }

    pub fn activate_target(&mut self, target: crate::focus::Target) {
        use crate::focus::Target;
        self.leave_startup_outline();
        match target {
            Target::Tab(tab) => {
                self.focus = Focus::Tab(tab);
                self.activate_tab(tab);
            }
            Target::Row(index) => {
                self.focus = Focus::Row(index);
                self.select_index(index);
            }
            Target::Action(action) => {
                self.focus = Focus::Action(action);
                self.open_action(action);
            }
            Target::Detail(control) => {
                self.focus = Focus::Detail(control);
                self.activate_detail(control);
            }
            Target::ServerPane(pane) => self.activate_server_pane(pane),
            Target::Power(focus) => self.activate_power(focus),
            Target::Prompt(focus) => self.activate_prompt(focus),
        }
    }

    pub fn move_list(&mut self, delta: i32) {
        if self.modal() != crate::focus::Modal::None {
            return;
        }
        if matches!(self.focus, Focus::ServerPane(_)) {
            self.leave_startup_outline();
            self.move_server_line(delta);
            return;
        }
        let count = self.row_count();
        if count == 0 {
            return;
        }
        self.leave_startup_outline();
        let current = self.selected_index().unwrap_or(0);
        let last = count - 1;
        let next = (current as i32 + delta).clamp(0, last as i32) as usize;
        if matches!(self.focus, Focus::Row(_)) {
            self.focus = Focus::Row(next);
        }
        if self.selected_index() != Some(next) {
            self.select_index(next);
        }
        self.park_list();
    }

    pub fn scroll_list(&mut self, delta: i32) {
        if self.modal() != crate::focus::Modal::None {
            return;
        }
        let window = usize::from(self.list_window.max(1));
        let max = self.row_count().saturating_sub(window);
        let next = self.list_scroll as i32 + delta;
        self.list_scroll = next.clamp(0, max as i32) as usize;
    }

    pub fn scroll_detail(&mut self, delta: i32) {
        if self.modal() != crate::focus::Modal::None {
            return;
        }
        let next = i32::from(self.detail_scroll) + delta;
        self.detail_scroll = next.max(0) as u16;
    }

    pub fn clamp_detail_scroll(&mut self, max: u16) {
        if self.detail_scroll > max {
            self.detail_scroll = max;
        }
    }

    pub fn type_char(&mut self, ch: char) {
        let Some(field) = self.focused_field_mut() else {
            return;
        };
        if !field.is_text() || field.value.chars().count() >= 2048 {
            return;
        }
        field.value.push(ch);
    }

    pub fn backspace(&mut self) {
        let Some(field) = self.focused_field_mut() else {
            return;
        };
        if field.is_text() {
            field.value.pop();
            return;
        }
        field.value.clear();
        if let FieldKind::Choice { picked, .. } = &mut field.kind {
            *picked = None;
        }
    }

    /// Move the open dropdown. `false` means this key is not on a dropdown.
    pub fn nudge_choice(&mut self, delta: i32) -> bool {
        let Some(field) = self.focused_field_mut() else {
            return false;
        };
        let (next, value) = {
            let FieldKind::Choice {
                options,
                picked,
                phase,
            } = &field.kind
            else {
                return false;
            };
            if *phase != ChoicePhase::Ready || options.is_empty() {
                return true;
            }
            let len = options.len();
            let next = match (*picked, delta.signum()) {
                (None, -1) => len - 1,
                (None, _) => 0,
                (Some(index), step) => {
                    let mut next = index as i32 + step;
                    if next < 0 {
                        next = len as i32 - 1;
                    }
                    if next >= len as i32 {
                        next = 0;
                    }
                    next as usize
                }
            };
            (next, options[next].value.clone())
        };
        if let FieldKind::Choice { picked, .. } = &mut field.kind {
            *picked = Some(next);
        }
        field.value = value;
        true
    }

    fn focused_field_mut(&mut self) -> Option<&mut PromptField> {
        let popup = self.prompt.as_mut()?;
        let PromptFocus::Field(index) = popup.focus else {
            return None;
        };
        popup.fields.get_mut(index)
    }

    pub fn open_power(&mut self, verb: PowerVerb) {
        if self.power.is_some() || self.prompt.is_some() {
            return;
        }
        let Some(server) = self.selected_server().cloned() else {
            self.banner_error("Select a server first");
            return;
        };
        self.power = Some(PowerPopup {
            session: self.alloc(),
            verb,
            mbpkgid: server.mbpkgid,
            fqdn: server.fqdn,
            kind: server.kind,
            force: self.force,
            focus: PopupFocus::Confirm,
            return_focus: Focus::Detail(verb.control()),
        });
    }

    /// Confirm is a no-op while a power call is already running.
    pub fn confirm_power(&mut self) {
        let Some(popup) = self.power.clone() else {
            return;
        };
        if self.power_inflight.is_some() {
            self.banner_error("A power change is already running");
            return;
        }
        if self.power_revoked == Some(popup.session) {
            return;
        }
        let token = self.issue_fixed(Slot::Power, popup.session, popup.session);
        self.power_inflight = Some(popup.session);
        self.power_accepted = Some(popup.session);
        self.focus = popup.return_focus;
        self.power = None;
        self.pending.push(Request::Power(PowerCommand {
            token,
            mbpkgid: popup.mbpkgid,
            kind: popup.kind,
            verb: popup.verb,
            force: popup.force,
        }));
    }

    pub fn cancel_popup(&mut self) {
        if let Some(popup) = self.power.take() {
            self.revoke_power_session(popup.session);
            self.focus = popup.return_focus;
            return;
        }
        if let Some(popup) = self.prompt.take() {
            self.focus = popup.return_focus;
        }
    }

    /// A confirm result for this session is dropped, including one already sent.
    pub fn revoke_power_session(&mut self, session: u64) {
        self.power_revoked = Some(session);
        if self.power_accepted == Some(session) {
            self.power_accepted = None;
        }
    }

    pub fn abandon(&mut self, request: Request) {
        let token = request.token();
        if self.power_inflight == Some(token.request_id) {
            self.power_inflight = None;
            self.power_accepted = None;
        }
        let catalog = matches!(request, Request::Catalog(_));
        self.fail_token(token, "API is not configured".to_string());
        if catalog {
            self.fail_open_choices();
        }
    }

    pub fn apply(&mut self, reply: Reply) {
        if !self.accepting {
            return;
        }
        match reply {
            Reply::Servers { token, result } => {
                self.apply_servers(token, result)
            }
            Reply::ServerDetail { token, result } => {
                self.apply_server_detail(token, result)
            }
            Reply::ServerList {
                token,
                pane,
                lines,
                errors,
            } => self.apply_server_list(token, pane, lines, errors),
            Reply::Power { token, result } => self.apply_power(token, result),
            Reply::Ssh { token, result } => self.apply_ssh(token, result),
            Reply::SshCreated { token, result } => {
                self.apply_ssh_created(token, result)
            }
            Reply::Locations { token, result } => {
                self.apply_locations(token, result)
            }
            Reply::LocationDetail { token, result } => {
                self.apply_location_detail(token, result)
            }
            Reply::Zones { token, result } => self.apply_zones(token, result),
            Reply::ZoneDetail { token, result } => {
                self.apply_zone_detail(token, result)
            }
            Reply::Account { token, result } => {
                self.apply_account(token, result)
            }
            Reply::Write { token, result } => self.apply_write(token, result),
            Reply::Catalog { token, result } => {
                self.apply_catalog(token, result)
            }
        }
    }

    fn activate_tab(&mut self, tab: Tab) {
        if self.tab != tab {
            self.invalidate(self.tab.list_slot());
            self.invalidate(detail_slot(self.tab));
            if self.tab == Tab::Servers {
                self.invalidate(Slot::ServerList);
            }
            self.tab = tab;
            self.list_scroll = 0;
            self.detail_scroll = 0;
        }
        self.focus = Focus::Tab(tab);
        self.queue_list();
        if self.tab != Tab::Account && self.selected_id().is_some() {
            self.queue_detail();
        }
    }

    fn activate_detail(&mut self, control: DetailControl) {
        match control {
            DetailControl::On => self.open_power(PowerVerb::On),
            DetailControl::Off => self.open_power(PowerVerb::Off),
            DetailControl::Reboot => self.open_power(PowerVerb::Reboot),
        }
    }

    fn activate_server_pane(&mut self, pane: ServerPane) {
        if self.server_pane != pane {
            self.server_pane = pane;
            self.clear_server_lines();
        }
        self.focus = Focus::ServerPane(pane);
        self.enqueue_server_list(false);
    }

    fn activate_power(&mut self, focus: PopupFocus) {
        if let Some(popup) = self.power.as_mut() {
            popup.focus = focus;
        }
        match focus {
            PopupFocus::Force => {
                if let Some(popup) = self.power.as_mut() {
                    popup.force = !popup.force;
                }
            }
            PopupFocus::Confirm => self.confirm_power(),
            PopupFocus::Cancel => self.cancel_popup(),
        }
    }

    fn activate_prompt(&mut self, focus: PromptFocus) {
        if let Some(popup) = self.prompt.as_mut() {
            popup.focus = focus;
        }
        match focus {
            PromptFocus::Field(_) => {}
            PromptFocus::Submit => self.submit_prompt(),
            PromptFocus::Cancel => self.cancel_popup(),
        }
    }

    /// Enter on a field moves forward. The last field submits.
    fn advance_prompt_field(&mut self) {
        let Some(popup) = self.prompt.as_ref() else {
            return;
        };
        let PromptFocus::Field(index) = popup.focus else {
            return;
        };
        if index + 1 >= popup.fields.len() {
            self.submit_prompt();
            return;
        }
        if let Some(popup) = self.prompt.as_mut() {
            popup.focus = PromptFocus::Field(index + 1);
        }
    }

    fn open_action(&mut self, action: Action) {
        if self.power.is_some() || self.prompt.is_some() {
            return;
        }
        let back = Focus::Action(action);
        let opened = match action {
            Action::CreateServer => {
                self.queue_catalog();
                Some(form(
                    "Create server",
                    "Up and down pick a plan, image, location, or SSH key.",
                    build_form_fields(),
                    "Create",
                    PromptKind::CreateServer,
                    back,
                ))
            }
            Action::RebuildServer => {
                let Some(server) = self.selected_server().cloned() else {
                    self.banner_error("Select a server first");
                    return;
                };
                self.queue_catalog();
                Some(form(
                    format!("Rebuild {}", server.fqdn),
                    format!(
                        "mbpkgid {}. Up and down pick a list.",
                        server.mbpkgid
                    ),
                    build_form_fields(),
                    "Rebuild",
                    PromptKind::RebuildServer {
                        mbpkgid: server.mbpkgid,
                        fqdn: server.fqdn,
                    },
                    back,
                ))
            }
            Action::DeleteServer => {
                let Some(server) = self.selected_server().cloned() else {
                    self.banner_error("Select a server first");
                    return;
                };
                Some(form(
                    format!("Delete {}", server.fqdn),
                    format!("mbpkgid {}", server.mbpkgid),
                    vec![text_field("hostname", "")],
                    "Delete",
                    PromptKind::DeleteServer {
                        mbpkgid: server.mbpkgid,
                        fqdn: server.fqdn,
                    },
                    back,
                ))
            }
            Action::NewKey => Some(form(
                "New SSH key",
                "Public key file, or empty to generate ~/.ssh/id_ed25519",
                vec![text_field("file", "")],
                "Create",
                PromptKind::NewKey,
                back,
            )),
            Action::UpdateKey => {
                let Some(key) = self.selected_key().cloned() else {
                    self.banner_error("Select an SSH key first");
                    return;
                };
                Some(form(
                    format!("Update {}", key.name),
                    "Public key text, or a file. A bare name is read from ~/.ssh.",
                    vec![
                        text_field("name", &key.name),
                        text_field("public key", ""),
                    ],
                    "Update",
                    PromptKind::UpdateKey { id: key.id },
                    back,
                ))
            }
            Action::DeleteKey => {
                let Some(key) = self.selected_key().cloned() else {
                    self.banner_error("Select an SSH key first");
                    return;
                };
                Some(form(
                    delete_title(&key.name, &format!("ssh key {}", key.id)),
                    format!("ssh key {}", key.id),
                    Vec::new(),
                    "Delete",
                    PromptKind::DeleteKey {
                        id: key.id,
                        name: key.name,
                    },
                    back,
                ))
            }
            Action::CreateZone => Some(form(
                "New zone",
                "Leave ip empty unless the zone needs one.",
                vec![
                    text_field("name", ""),
                    text_field("type", "NATIVE"),
                    text_field("ip", ""),
                ],
                "Create",
                PromptKind::CreateZone,
                back,
            )),
            Action::DeleteZone => {
                let Some(zone) = self.selected_zone().cloned() else {
                    self.banner_error("Select a zone first");
                    return;
                };
                Some(form(
                    delete_title(&zone.name, &format!("zone {}", zone.id)),
                    format!("zone {}", zone.id),
                    Vec::new(),
                    "Delete",
                    PromptKind::DeleteZone {
                        id: zone.id,
                        name: zone.name,
                    },
                    back,
                ))
            }
            Action::CreateRecord => {
                let Some(zone) = self.selected_zone().cloned() else {
                    self.banner_error("Select a zone first");
                    return;
                };
                Some(form(
                    format!("New record in {}", zone.name),
                    "type is A, AAAA, CNAME, MX, TXT, or another record type",
                    record_form_fields(false),
                    "Create",
                    PromptKind::CreateRecord {
                        domain_id: i64::from(zone.id),
                        zone: zone.name,
                    },
                    back,
                ))
            }
            Action::UpdateRecord => {
                let Some(zone) = self.selected_zone().cloned() else {
                    self.banner_error("Select a zone first");
                    return;
                };
                Some(form(
                    format!("Update record in {}", zone.name),
                    self.record_hint(&zone.name),
                    record_form_fields(true),
                    "Update",
                    PromptKind::UpdateRecord {
                        domain_id: i64::from(zone.id),
                        zone: zone.name,
                    },
                    back,
                ))
            }
            Action::DeleteRecord => {
                let Some(zone) = self.selected_zone().cloned() else {
                    self.banner_error("Select a zone first");
                    return;
                };
                Some(form(
                    format!("Delete record in {}", zone.name),
                    self.record_hint(&zone.name),
                    vec![text_field("id", "")],
                    "Delete",
                    PromptKind::DeleteRecord { zone: zone.name },
                    back,
                ))
            }
        };
        self.prompt = opened;
    }

    fn record_hint(&self, zone: &str) -> String {
        if self.zone_records.is_empty() {
            return format!("record id in {zone}");
        }
        let listed = self
            .zone_records
            .iter()
            .take(4)
            .map(|record| format!("{} {}", record.id, record.name))
            .collect::<Vec<_>>()
            .join(", ");
        format!("id in {zone}: {listed}")
    }

    fn submit_prompt(&mut self) {
        let Some(prompt) = self.prompt.clone() else {
            return;
        };
        match self.prepare_write(&prompt) {
            Ok(Prepared::Send(op)) => {
                if self.write_busy() {
                    self.reject_prompt("A change is already running");
                    return;
                }
                self.prompt = None;
                self.focus = prompt.return_focus;
                let token = self.issue(Slot::Write, 0);
                self.pending.push(Request::Write { token, op });
            }
            Ok(Prepared::Ssh(file)) => {
                if self.write_busy() {
                    self.reject_prompt("A change is already running");
                    return;
                }
                self.prompt = None;
                self.focus = prompt.return_focus;
                let token = self.issue(Slot::SshCreate, 0);
                self.pending.push(Request::SshCreate { token, file });
            }
            Ok(Prepared::Stay(next)) => self.prompt = Some(next),
            Err(err) => self.reject_prompt(err),
        }
    }

    fn write_busy(&self) -> bool {
        self.is_loading(Slot::Write) || self.is_loading(Slot::SshCreate)
    }

    fn reject_prompt(&mut self, err: impl Into<String>) {
        let err = err.into();
        if let Some(popup) = self.prompt.as_mut() {
            popup.hint = err.clone();
        }
        self.banner_error(err);
    }

    fn prepare_write(&self, prompt: &Prompt) -> Result<Prepared, String> {
        match &prompt.kind {
            PromptKind::NewKey => {
                Ok(Prepared::Ssh(optional_text(field(prompt, "file"))))
            }
            PromptKind::UpdateKey { id } => {
                let name = required_text(field(prompt, "name"), "name")?;
                let material =
                    required_text(field(prompt, "public key"), "public key")?;
                Ok(Prepared::Send(WriteOp::UpdateKey {
                    id: *id,
                    name,
                    material,
                }))
            }
            PromptKind::DeleteKey { id, name } => {
                Ok(Prepared::Send(WriteOp::DeleteKey {
                    id: *id,
                    name: name.clone(),
                }))
            }
            PromptKind::CreateServer => {
                // buy_build needs plan, location, image, fqdn, and an ssh key id.
                create_server_ready(prompt)?;
                let mut fields = build_fields(prompt)?;
                if let Some(billing) = &self.usage_billing {
                    fields.package_billing =
                        Some(billing.contract_type.clone());
                    fields.package_billing_contract_id =
                        Some(billing.id.clone());
                }
                Ok(Prepared::Send(WriteOp::CreateServer(fields)))
            }
            PromptKind::RebuildServer { mbpkgid, fqdn } => {
                Ok(Prepared::Send(WriteOp::RebuildServer {
                    mbpkgid: *mbpkgid,
                    fqdn: fqdn.clone(),
                    fields: build_fields(prompt)?,
                }))
            }
            PromptKind::DeleteServer { mbpkgid, fqdn } => {
                let typed =
                    required_text(field(prompt, "hostname"), "hostname")?;
                if typed != fqdn.trim() {
                    return Err("hostname does not match".into());
                }
                Ok(Prepared::Send(WriteOp::DeleteServer {
                    mbpkgid: *mbpkgid,
                    fqdn: fqdn.clone(),
                }))
            },
            PromptKind::CreateZone => {
                let name = required_text(field(prompt, "name"), "name")?;
                let zone_type = optional_text(field(prompt, "type"))
                    .unwrap_or_else(|| "NATIVE".to_string());
                let ip = optional_text(field(prompt, "ip"));
                Ok(Prepared::Send(WriteOp::CreateZone {
                    name,
                    zone_type,
                    ip,
                }))
            }
            PromptKind::DeleteZone { id, name } => {
                Ok(Prepared::Send(WriteOp::DeleteZone {
                    id: *id,
                    name: name.clone(),
                }))
            }
            PromptKind::CreateRecord { domain_id, zone } => {
                Ok(Prepared::Send(WriteOp::CreateRecord {
                    domain_id: *domain_id,
                    zone: zone.clone(),
                    fields: self.record_fields(prompt, None)?,
                }))
            }
            PromptKind::UpdateRecord { domain_id, zone } => {
                let id = required_u32(field(prompt, "id"), "id")?;
                Ok(Prepared::Send(WriteOp::UpdateRecord {
                    domain_id: *domain_id,
                    zone: zone.clone(),
                    fields: self.record_fields(prompt, Some(id))?,
                }))
            }
            PromptKind::DeleteRecord { zone } => {
                let id = required_u32(field(prompt, "id"), "id")?;
                let name = self.record_name(id);
                Ok(Prepared::Stay(form(
                    delete_title(&name, &format!("record {id}")),
                    format!("record {id} in {zone}"),
                    Vec::new(),
                    "Delete",
                    PromptKind::ConfirmDeleteRecord {
                        id,
                        name,
                        zone: zone.clone(),
                    },
                    prompt.return_focus,
                )))
            }
            PromptKind::ConfirmDeleteRecord { id, name, zone } => {
                Ok(Prepared::Send(WriteOp::DeleteRecord {
                    id: *id,
                    name: name.clone(),
                    zone: zone.clone(),
                }))
            }
        }
    }

    fn record_name(&self, id: u32) -> String {
        self.zone_records
            .iter()
            .find(|record| record.id == id)
            .map(|record| {
                if !record.name.is_empty() {
                    record.name.clone()
                } else if !record.record_type.is_empty() {
                    record.record_type.clone()
                } else {
                    format!("record {id}")
                }
            })
            .unwrap_or_else(|| format!("record {id}"))
    }

    /// Blank type, content, name, ttl, and prio keep the loaded record's values.
    fn record_fields(
        &self, prompt: &Prompt, id: Option<u32>,
    ) -> Result<RecordFields, String> {
        let stored = id.and_then(|id| {
            self.zone_records.iter().find(|record| record.id == id)
        });
        let mut name = optional_text(field(prompt, "name"));
        let mut record_type =
            optional_text(field(prompt, "type")).unwrap_or_default();
        let mut content =
            optional_text(field(prompt, "content")).unwrap_or_default();
        let mut ttl = optional_i64(field(prompt, "ttl"), "ttl")?;
        let mut prio = optional_i64(field(prompt, "prio"), "prio")?;
        if let Some(stored) = stored {
            if name.is_none() && !stored.name.is_empty() {
                name = Some(stored.name.clone());
            }
            if record_type.is_empty() {
                record_type = stored.record_type.clone();
            }
            if content.is_empty() {
                content = stored.content.clone();
            }
            if ttl.is_none() {
                ttl = stored.ttl.map(i64::from);
            }
            if prio.is_none() {
                prio = stored.prio.map(i64::from);
            }
        }
        if record_type.is_empty() {
            return Err("type is required".into());
        }
        if content.is_empty() {
            return Err("content is required".into());
        }
        Ok(RecordFields {
            id: id.map(i64::from),
            name,
            record_type,
            content,
            ttl,
            prio,
        })
    }

    fn select_index(&mut self, index: usize) {
        let id = match self.tab {
            Tab::Servers => {
                self.servers.get(index).map(|row| u64::from(row.mbpkgid))
            }
            Tab::SshKeys => self.keys.get(index).map(|row| u64::from(row.id)),
            Tab::Locations => {
                self.locations.get(index).map(|row| u64::from(row.id))
            }
            Tab::Dns => self.zones.get(index).map(|row| u64::from(row.id)),
            Tab::Account => Some(0),
        };
        let Some(id) = id else {
            return;
        };
        self.select_id(id);
    }

    fn select_id(&mut self, id: u64) {
        if self.selected_id() == Some(id) {
            self.queue_detail();
            return;
        }
        self.write_selected(Some(id));
        self.selection_gen = self.selection_gen.wrapping_add(1);
        self.detail_scroll = 0;
        self.hide_detail();
        self.queue_detail();
        self.park_list();
    }

    fn select_first(&mut self) {
        let id = match self.tab {
            Tab::Servers => {
                self.servers.first().map(|row| u64::from(row.mbpkgid))
            }
            Tab::SshKeys => self.keys.first().map(|row| u64::from(row.id)),
            Tab::Locations => {
                self.locations.first().map(|row| u64::from(row.id))
            }
            Tab::Dns => self.zones.first().map(|row| u64::from(row.id)),
            Tab::Account => Some(0),
        };
        self.write_selected(id);
        if id.is_some() {
            self.selection_gen = self.selection_gen.wrapping_add(1);
            self.hide_detail();
        }
    }

    fn write_selected(&mut self, id: Option<u64>) {
        match self.tab {
            Tab::Servers => self.servers_selected = id,
            Tab::SshKeys => self.ssh_selected = id,
            Tab::Locations => self.location_selected = id,
            Tab::Dns => self.zone_selected = id,
            Tab::Account => {}
        }
    }

    fn selected_id(&self) -> Option<u64> {
        match self.tab {
            Tab::Servers => self.servers_selected,
            Tab::SshKeys => self.ssh_selected,
            Tab::Locations => self.location_selected,
            Tab::Dns => self.zone_selected,
            Tab::Account => Some(0),
        }
    }

    fn hide_detail(&mut self) {
        match self.tab {
            Tab::Servers => {
                self.server_detail_for = None;
                self.clear_server_lines();
            }
            Tab::Locations => self.location_detail = None,
            Tab::Dns => {
                self.zone_detail = None;
                self.zone_records.clear();
            }
            Tab::SshKeys | Tab::Account => {}
        }
    }

    fn queue_catalog(&mut self) {
        // A new form must not reuse a contract from the previous attempt.
        self.usage_billing = None;
        let token = self.issue(Slot::Catalog, 0);
        self.pending.push(Request::Catalog(token));
    }

    fn queue_list(&mut self) {
        let slot = self.tab.list_slot();
        let token = self.issue(slot, self.selection_gen);
        let request = match self.tab {
            Tab::Servers => Request::Servers(token),
            Tab::SshKeys => Request::Ssh(token),
            Tab::Locations => Request::Locations(token),
            Tab::Dns => Request::Zones(token),
            Tab::Account => Request::Account(token),
        };
        self.pending.push(request);
    }

    fn queue_detail(&mut self) {
        match self.tab {
            Tab::Servers => self.queue_server_detail(),
            Tab::Locations => {
                let Some(id) = self.selected_location().map(|row| row.id)
                else {
                    return;
                };
                let token = self.issue(Slot::LocationDetail, u64::from(id));
                self.pending.push(Request::LocationDetail { token, id });
            }
            Tab::Dns => {
                let Some(id) = self.selected_zone().map(|row| row.id) else {
                    return;
                };
                let token = self.issue(Slot::ZoneDetail, u64::from(id));
                self.pending.push(Request::ZoneDetail { token, id });
            }
            Tab::Account => {
                let token = self.issue(Slot::Account, 0);
                self.pending.push(Request::Account(token));
            }
            Tab::SshKeys => {}
        }
    }

    fn queue_server_detail(&mut self) {
        self.enqueue_server_detail(false);
        self.enqueue_server_list(false);
    }

    fn enqueue_server_detail(&mut self, silent: bool) {
        let Some(server) = self.selected_server().cloned() else {
            return;
        };
        let token = self.issue(Slot::ServerDetail, u64::from(server.mbpkgid));
        if silent {
            self.mark_silent(Slot::ServerDetail);
        }
        self.pending.push(Request::ServerDetail(DetailQuery {
            token,
            kind: server.kind,
            mbpkgid: server.mbpkgid,
            metal_id: server.metal_id,
        }));
    }

    fn enqueue_server_list(&mut self, silent: bool) {
        let Some(server) = self.selected_server() else {
            return;
        };
        let pane = self.server_pane;
        let mbpkgid = server.mbpkgid;
        let token = self.issue(Slot::ServerList, u64::from(mbpkgid));
        if silent {
            self.mark_silent(Slot::ServerList);
        }
        self.pending.push(Request::ServerList(ServerListQuery {
            token,
            pane,
            mbpkgid,
        }));
    }

    pub fn server_lines_ready(&self) -> bool {
        let Some(server) = self.selected_server() else {
            return false;
        };
        self.server_lines_for == Some(u64::from(server.mbpkgid))
            && self.server_lines_pane == self.server_pane
    }

    fn clear_server_lines(&mut self) {
        self.server_lines.clear();
        self.server_lines_for = None;
        self.server_line = 0;
        self.detail_scroll = 0;
    }

    fn move_server_line(&mut self, delta: i32) {
        let count = if self.server_lines_ready() {
            self.server_lines.len()
        } else {
            0
        };
        if count == 0 {
            return;
        }
        let last = count - 1;
        let next =
            (self.server_line as i32 + delta).clamp(0, last as i32) as usize;
        self.server_line = next;
        let window = usize::from(self.server_line_window.max(1));
        let scroll = usize::from(self.detail_scroll);
        if next < scroll {
            self.detail_scroll = u16::try_from(next).unwrap_or(u16::MAX);
        } else if next >= scroll.saturating_add(window) {
            let start = next + 1 - window;
            self.detail_scroll = u16::try_from(start).unwrap_or(u16::MAX);
        }
    }

    fn apply_servers(
        &mut self, token: Token, result: Result<ServerBatch, String>,
    ) {
        if !self.accept_page(token) {
            return;
        }
        let silent = self.track(token.slot).silent;
        self.finish(token);
        match result {
            Ok(batch) => {
                let moved = self.selection_gen != token.selection;
                let previous = self.servers_selected;
                self.servers = batch.rows;
                self.reconcile(moved, previous, |app, id| {
                    app.servers.iter().any(|row| u64::from(row.mbpkgid) == id)
                });
                if silent {
                    if let Some(warning) = batch.warning {
                        self.banner_error_once(warning);
                    }
                } else {
                    self.banner_ok(loaded("servers", self.servers.len()));
                    self.note_warning(batch.warning);
                }
                if self.servers_selected.is_some() {
                    self.enqueue_server_detail(silent);
                    // The quiet refresh keeps the open list. A new selection
                    // still needs that server's jobs, addresses, or bandwidth.
                    if !silent || self.servers_selected != previous {
                        self.enqueue_server_list(silent);
                    }
                }
                self.normalize_focus();
                self.settle_focus();
                self.park_list();
            }
            Err(err) => self.report_server_error(silent, err),
        }
    }

    fn apply_ssh(&mut self, token: Token, result: Result<Vec<SshRow>, String>) {
        if !self.accept_page(token) {
            return;
        }
        self.finish(token);
        match result {
            Ok(rows) => {
                let moved = self.selection_gen != token.selection;
                let previous = self.ssh_selected;
                self.keys = rows;
                self.reconcile(moved, previous, |app, id| {
                    app.keys.iter().any(|row| u64::from(row.id) == id)
                });
                self.banner_ok(loaded("ssh keys", self.keys.len()));
                self.normalize_focus();
                self.settle_focus();
                self.park_list();
            }
            Err(err) => self.banner_error(err),
        }
    }

    fn apply_locations(
        &mut self, token: Token, result: Result<Vec<LocationRow>, String>,
    ) {
        if !self.accept_page(token) {
            return;
        }
        self.finish(token);
        match result {
            Ok(rows) => {
                let moved = self.selection_gen != token.selection;
                let previous = self.location_selected;
                self.locations = rows;
                self.reconcile(moved, previous, |app, id| {
                    app.locations.iter().any(|row| u64::from(row.id) == id)
                });
                if self.location_selected.is_some() {
                    self.queue_detail();
                }
                self.banner_ok(loaded("locations", self.locations.len()));
                self.normalize_focus();
                self.settle_focus();
                self.park_list();
            }
            Err(err) => self.banner_error(err),
        }
    }

    fn apply_zones(
        &mut self, token: Token, result: Result<Vec<ZoneRow>, String>,
    ) {
        if !self.accept_page(token) {
            return;
        }
        self.finish(token);
        match result {
            Ok(rows) => {
                let moved = self.selection_gen != token.selection;
                let previous = self.zone_selected;
                self.zones = rows;
                self.reconcile(moved, previous, |app, id| {
                    app.zones.iter().any(|row| u64::from(row.id) == id)
                });
                if self.zone_selected.is_some() {
                    self.queue_detail();
                }
                self.banner_ok(loaded("zones", self.zones.len()));
                self.normalize_focus();
                self.settle_focus();
                self.park_list();
            }
            Err(err) => self.banner_error(err),
        }
    }

    fn reconcile(
        &mut self, moved: bool, previous: Option<u64>,
        present: impl Fn(&App, u64) -> bool,
    ) {
        if let Some(id) = previous
            && present(self, id)
        {
            self.write_selected(Some(id));
            return;
        }
        if moved && previous.is_some() {
            self.select_first();
            return;
        }
        if !moved {
            self.select_first();
        }
    }

    fn apply_server_detail(
        &mut self, token: Token, result: Result<ServerDetail, String>,
    ) {
        if !self.accept_detail(token) {
            return;
        }
        let silent = self.track(token.slot).silent;
        self.finish(token);
        match result {
            Ok(detail) => {
                if let Some(row) = self
                    .servers
                    .iter_mut()
                    .find(|row| u64::from(row.mbpkgid) == token.selection)
                {
                    if let Some(fqdn) = detail.fqdn {
                        row.fqdn = fqdn;
                    }
                    if let Some(state) = detail.state {
                        row.state = state;
                    }
                    if let Some(status) = detail.status.clone() {
                        row.status = status;
                    }
                    if let Some(ip) = detail.ip {
                        row.ip = ip;
                    }
                    if let Some(location) = detail.location {
                        row.location = location;
                    }
                    if let Some(plan_id) = detail.plan_id {
                        row.plan_id = Some(plan_id);
                    }
                    if let Some(pkg_id) = detail.pkg_id {
                        row.pkg_id = Some(pkg_id);
                    }
                }
                self.server_detail_for = Some(token.selection);
                self.server_extras = detail.extras;
                if !silent {
                    self.banner_ok(format!(
                        "server {} loaded",
                        token.selection
                    ));
                }
                if let Some(error) = detail.error {
                    self.report_server_error(silent, error);
                }
            }
            Err(err) => self.report_server_error(silent, err),
        }
    }

    fn apply_server_list(
        &mut self, token: Token, pane: ServerPane, lines: Vec<String>,
        errors: Vec<String>,
    ) {
        if !self.accept_server_list(token, pane) {
            return;
        }
        let silent = self.track(token.slot).silent;
        self.finish(token);
        self.server_lines = lines;
        self.server_lines_for = Some(token.selection);
        self.server_lines_pane = pane;
        self.server_line = 0;
        self.detail_scroll = 0;
        if errors.is_empty() {
            if !silent {
                self.banner_ok(pane.loaded());
            }
        } else {
            for error in errors {
                self.report_server_error(silent, error);
            }
        }
    }

    fn apply_location_detail(
        &mut self, token: Token, result: Result<Pane, String>,
    ) {
        let id = token.selection;
        self.apply_pane(
            token,
            result,
            format!("location {id} loaded"),
            |app, pane| {
                app.location_detail = Some((id, pane));
            },
        );
    }

    fn apply_zone_detail(
        &mut self, token: Token, result: Result<ZoneView, String>,
    ) {
        let id = token.selection;
        if !self.accept_detail(token) {
            return;
        }
        self.finish(token);
        match result {
            Ok(view) => {
                self.zone_records = view.records;
                self.zone_detail = Some((id, view.pane));
                self.banner_ok(format!("zone {id} loaded"));
            }
            Err(err) => self.banner_error(err),
        }
    }

    fn apply_account(&mut self, token: Token, result: Result<Pane, String>) {
        if !self.accept_page(token) {
            return;
        }
        self.finish(token);
        match result {
            Ok(pane) => {
                self.account = Some(pane);
                self.banner_ok("account loaded");
                self.normalize_focus();
                self.settle_focus();
            }
            Err(err) => self.banner_error(err),
        }
    }

    fn apply_pane(
        &mut self, token: Token, result: Result<Pane, String>, ok: String,
        store: impl FnOnce(&mut App, Pane),
    ) {
        if !self.accept_detail(token) {
            return;
        }
        self.finish(token);
        match result {
            Ok(pane) => {
                store(self, pane);
                self.banner_ok(ok);
            }
            Err(err) => self.banner_error(err),
        }
    }

    fn apply_power(&mut self, token: Token, result: Result<String, String>) {
        if self.power_inflight == Some(token.request_id) {
            self.power_inflight = None;
            self.finish(token);
        }
        // Cancel wins over a confirm that already left the popup.
        if self.power_revoked == Some(token.request_id)
            || self.power_accepted != Some(token.request_id)
        {
            return;
        }
        self.power_accepted = None;
        match result {
            Ok(text) => {
                self.banner_ok(text);
                if self.tab == Tab::Servers {
                    self.queue_list();
                }
            }
            Err(err) => self.banner_error(err),
        }
    }

    fn apply_catalog(
        &mut self, token: Token, result: Result<BuildChoices, String>,
    ) {
        if self.track(Slot::Catalog).latest != token.request_id {
            return;
        }
        self.finish(token);
        let choices = match result {
            Ok(choices) => choices,
            Err(err) => {
                self.usage_billing = None;
                self.fail_open_choices();
                if self.prompt_is_build() {
                    self.banner_error(err);
                }
                return;
            }
        };
        let create = self.prompt.as_ref().is_some_and(|prompt| {
            matches!(prompt.kind, PromptKind::CreateServer)
        });
        if create {
            self.usage_billing = choices.billing.clone();
        }
        if !self.prompt_is_build() {
            return;
        }
        let mut notes = Vec::new();
        if create && let Some(err) = &choices.billing_error {
            notes.push(format!("usage contract: {err}"));
        }
        self.fill_choice("plan", choices.plans, &mut notes);
        self.fill_choice("image", choices.images, &mut notes);
        self.fill_choice("location", choices.locations, &mut notes);
        self.fill_choice("ssh key", choices.keys, &mut notes);
        if notes.is_empty() {
            return;
        }
        let note = notes.join("; ");
        if let Some(prompt) = self.prompt.as_mut() {
            prompt.hint = note.clone();
        }
        self.banner_error(note);
    }

    fn prompt_is_build(&self) -> bool {
        self.prompt.as_ref().is_some_and(|prompt| {
            matches!(
                prompt.kind,
                PromptKind::CreateServer | PromptKind::RebuildServer { .. }
            )
        })
    }

    fn fill_choice(
        &mut self, label: &str, listed: Listed, notes: &mut Vec<String>,
    ) {
        if let Some(err) = listed.error.as_ref() {
            notes.push(format!("{label}: {err}"));
        }
        let Some(prompt) = self.prompt.as_mut() else {
            return;
        };
        let Some(field) =
            prompt.fields.iter_mut().find(|field| field.label == label)
        else {
            return;
        };
        if !matches!(field.kind, FieldKind::Choice { .. }) {
            return;
        }
        if listed.error.is_some() {
            field.kind = FieldKind::Choice {
                options: Vec::new(),
                picked: None,
                phase: ChoicePhase::Failed,
            };
            field.value.clear();
            return;
        }
        field.kind = FieldKind::Choice {
            options: listed.options,
            picked: None,
            phase: ChoicePhase::Ready,
        };
        field.value.clear();
    }

    fn fail_open_choices(&mut self) {
        let Some(prompt) = self.prompt.as_mut() else {
            return;
        };
        if !matches!(
            prompt.kind,
            PromptKind::CreateServer | PromptKind::RebuildServer { .. }
        ) {
            return;
        }
        for field in &mut prompt.fields {
            if matches!(field.kind, FieldKind::Choice { .. }) {
                field.kind = FieldKind::Choice {
                    options: Vec::new(),
                    picked: None,
                    phase: ChoicePhase::Failed,
                };
                field.value.clear();
            }
        }
    }

    fn apply_write(&mut self, token: Token, result: Result<String, String>) {
        if self.track(Slot::Write).latest != token.request_id {
            return;
        }
        self.finish(token);
        match result {
            Ok(text) => {
                self.banner_ok(text);
                if self.tab == token.tab {
                    self.queue_list();
                }
            }
            Err(err) => self.banner_error(err),
        }
    }

    fn apply_ssh_created(
        &mut self, token: Token, result: Result<String, String>,
    ) {
        if self.track(Slot::SshCreate).latest != token.request_id {
            return;
        }
        self.finish(token);
        match result {
            Ok(text) => {
                self.banner_ok(ssh_created_line(&text));
                if self.tab == Tab::SshKeys {
                    self.queue_list();
                }
            }
            Err(err) => self.banner_error(err),
        }
    }

    fn note_warning(&mut self, warning: Option<String>) {
        if let Some(warning) = warning {
            self.banner_error(warning);
        } else {
            self.clear_error_banner();
        }
    }

    fn accept_page(&self, token: Token) -> bool {
        self.tab == token.tab
            && self.track(token.slot).latest == token.request_id
    }

    fn accept_detail(&self, token: Token) -> bool {
        self.accept_page(token) && self.selected_id() == Some(token.selection)
    }

    /// A list for another server, another subtab, or an older request is dropped.
    fn accept_server_list(&self, token: Token, pane: ServerPane) -> bool {
        self.accept_detail(token) && self.server_pane == pane
    }

    fn fail_token(&mut self, token: Token, message: String) {
        if self.track(token.slot).latest == token.request_id {
            self.finish(token);
            self.banner_error(message);
        }
    }

    pub(crate) fn status_lines(&self) -> &[Banner] {
        &self.log
    }

    pub(crate) fn banner_error(&mut self, text: impl Into<String>) {
        self.remember(Level::Error, text);
    }

    fn banner_ok(&mut self, text: impl Into<String>) {
        self.remember(Level::Ok, text);
    }

    /// A repeating refresh failure stays one line, so the log does not fill.
    fn banner_error_once(&mut self, text: impl Into<String>) {
        let text = status_text(text.into());
        if self
            .log
            .last()
            .is_some_and(|line| line.level == Level::Error && line.text == text)
        {
            return;
        }
        self.remember(Level::Error, text);
    }

    fn report_server_error(&mut self, silent: bool, text: impl Into<String>) {
        if silent {
            self.banner_error_once(text);
        } else {
            self.banner_error(text);
        }
    }

    fn remember(&mut self, level: Level, text: impl Into<String>) {
        let text = status_text(text.into());
        if text.is_empty() {
            return;
        }
        self.log.push(Banner {
            level,
            text: text.clone(),
        });
        if self.log.len() > LOG_KEEP {
            let extra = self.log.len() - LOG_KEEP;
            self.log.drain(..extra);
        }
        self.banner = Some(Banner { level, text });
    }

    fn clear_error_banner(&mut self) {
        if self
            .banner
            .as_ref()
            .is_some_and(|banner| banner.level == Level::Error)
        {
            self.banner = None;
        }
    }

    fn normalize_focus(&mut self) {
        if self.power.is_some() || self.prompt.is_some() {
            return;
        }
        if matches!(self.focus, Focus::Split(_)) {
            return;
        }
        let items =
            order(self.tab, self.row_count(), self.selected_index().is_some());
        if items.contains(&self.focus) {
            return;
        }
        self.focus = self
            .selected_index()
            .map(Focus::Row)
            .filter(|focus| items.contains(focus))
            .unwrap_or(Focus::Tab(self.tab));
    }

    fn park_list(&mut self) {
        let Some(index) = self.selected_index() else {
            return;
        };
        let window = usize::from(self.list_window.max(1));
        if index < self.list_scroll {
            self.list_scroll = index;
        } else if index >= self.list_scroll.saturating_add(window) {
            self.list_scroll = index + 1 - window;
        }
    }

    fn issue(&mut self, slot: Slot, selection: u64) -> Token {
        let request_id = self.alloc();
        self.issue_fixed(slot, selection, request_id)
    }

    fn issue_fixed(
        &mut self, slot: Slot, selection: u64, request_id: u64,
    ) -> Token {
        let track = self.track_mut(slot);
        track.latest = request_id;
        track.loading = true;
        track.silent = false;
        Token {
            tab: self.tab,
            selection,
            request_id,
            slot,
        }
    }

    fn alloc(&mut self) -> u64 {
        self.next_id = self.next_id.wrapping_add(1).max(1);
        self.next_id
    }

    fn invalidate(&mut self, slot: Slot) {
        let request_id = self.alloc();
        let track = self.track_mut(slot);
        track.latest = request_id;
        track.loading = false;
        track.silent = false;
    }

    fn finish(&mut self, token: Token) {
        let track = self.track_mut(token.slot);
        if track.latest == token.request_id {
            track.loading = false;
            track.silent = false;
        }
    }

    fn track(&self, slot: Slot) -> &Track {
        &self.tracks[slot_index(slot)]
    }

    fn track_mut(&mut self, slot: Slot) -> &mut Track {
        &mut self.tracks[slot_index(slot)]
    }
}

enum Prepared {
    Send(WriteOp),
    Ssh(Option<String>),
    Stay(Prompt),
}

fn form(
    title: impl Into<String>, hint: impl Into<String>,
    fields: Vec<PromptField>, submit: &str, kind: PromptKind,
    return_focus: Focus,
) -> Prompt {
    let focus = if fields.is_empty() {
        PromptFocus::Submit
    } else {
        PromptFocus::Field(0)
    };
    Prompt {
        title: title.into(),
        hint: hint.into(),
        fields,
        focus,
        submit_label: submit.to_string(),
        kind,
        return_focus,
    }
}

fn text_field(label: &str, value: &str) -> PromptField {
    PromptField {
        label: label.to_string(),
        value: value.to_string(),
        kind: FieldKind::Text,
    }
}

fn choice_field(label: &str) -> PromptField {
    PromptField {
        label: label.to_string(),
        value: String::new(),
        kind: FieldKind::Choice {
            options: Vec::new(),
            picked: None,
            phase: ChoicePhase::Loading,
        },
    }
}

fn build_form_fields() -> Vec<PromptField> {
    vec![
        text_field("fqdn", ""),
        choice_field("plan"),
        choice_field("location"),
        choice_field("image"),
        choice_field("ssh key"),
    ]
}

fn record_form_fields(with_id: bool) -> Vec<PromptField> {
    let mut fields = Vec::new();
    if with_id {
        fields.push(text_field("id", ""));
    }
    fields.extend([
        text_field("name", ""),
        text_field("type", ""),
        text_field("content", ""),
        text_field("ttl", ""),
        text_field("prio", ""),
    ]);
    fields
}

fn delete_title(name: &str, fallback: &str) -> String {
    if name.is_empty() {
        format!("Delete {fallback}")
    } else {
        format!("Delete {name}")
    }
}

fn field<'a>(prompt: &'a Prompt, label: &str) -> &'a str {
    prompt
        .fields
        .iter()
        .find(|field| field.label == label)
        .map(|field| field.value.as_str())
        .unwrap_or("")
}

fn build_fields(prompt: &Prompt) -> Result<BuildFields, String> {
    Ok(BuildFields {
        fqdn: optional_text(field(prompt, "fqdn")),
        plan: optional_text(field(prompt, "plan")),
        location: optional_i64(field(prompt, "location"), "location")?,
        image: optional_i64(field(prompt, "image"), "image")?,
        ssh_key_id: optional_i64(field(prompt, "ssh key"), "ssh key")?,
        package_billing: None,
        package_billing_contract_id: None,
    })
}

/// `Server::buy` stays unsent until plan, location, image, fqdn, and an ssh
/// key id are set. A missing usage contract does not drop those fields.
/// A dropdown that is still loading or failed is named on the form.
fn create_server_ready(prompt: &Prompt) -> Result<(), String> {
    let mut loading = Vec::new();
    let mut failed = Vec::new();
    let mut missing = Vec::new();
    if field(prompt, "fqdn").trim().is_empty() {
        missing.push("fqdn");
    }
    for label in ["plan", "location", "image", "ssh key"] {
        match choice_phase(prompt, label) {
            Some(ChoicePhase::Loading) => loading.push(label),
            Some(ChoicePhase::Failed) => failed.push(label),
            _ => {
                if field(prompt, label).trim().is_empty() {
                    missing.push(label);
                }
            }
        }
    }
    if !loading.is_empty() {
        return Err(state_line(
            &loading,
            "is still loading",
            "are still loading",
        ));
    }
    if !failed.is_empty() {
        return Err(state_line(&failed, "is unavailable", "are unavailable"));
    }
    if !missing.is_empty() {
        return Err(state_line(&missing, "is required", "are required"));
    }
    Ok(())
}

fn choice_phase(prompt: &Prompt, label: &str) -> Option<ChoicePhase> {
    let field = prompt.fields.iter().find(|field| field.label == label)?;
    match &field.kind {
        FieldKind::Choice { phase, .. } => Some(*phase),
        FieldKind::Text => None,
    }
}

fn state_line(names: &[&str], one: &str, many: &str) -> String {
    let verb = if names.len() == 1 { one } else { many };
    format!("{} {verb}", name_list(names))
}

fn name_list(names: &[&str]) -> String {
    let Some((last, head)) = names.split_last() else {
        return String::new();
    };
    match head {
        [] => (*last).to_string(),
        [only] => format!("{only} and {last}"),
        _ => format!("{}, and {last}", head.join(", ")),
    }
}

fn required_text(value: &str, label: &str) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(format!("{label} is required"));
    }
    Ok(value.to_string())
}

fn optional_text(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn optional_i64(value: &str, label: &str) -> Result<Option<i64>, String> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }
    value
        .parse::<i64>()
        .map(Some)
        .map_err(|_| format!("{label} must be a number"))
}

fn required_u32(value: &str, label: &str) -> Result<u32, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(format!("{label} is required"));
    }
    value
        .parse::<u32>()
        .map_err(|_| format!("{label} must be a number"))
}

fn position<T>(
    id: Option<u64>, rows: &[T], key: impl Fn(&T) -> u64,
) -> Option<usize> {
    let id = id?;
    rows.iter().position(|row| key(row) == id)
}

const TRACKS: usize = 13;

fn slot_index(slot: Slot) -> usize {
    match slot {
        Slot::Servers => 0,
        Slot::ServerDetail => 1,
        Slot::Ssh => 2,
        Slot::SshCreate => 3,
        Slot::Locations => 4,
        Slot::LocationDetail => 5,
        Slot::Zones => 6,
        Slot::ZoneDetail => 7,
        Slot::Account => 8,
        Slot::Power => 9,
        Slot::ServerList => 10,
        Slot::Write => 11,
        Slot::Catalog => 12,
    }
}

fn detail_slot(tab: Tab) -> Slot {
    match tab {
        Tab::Servers => Slot::ServerDetail,
        Tab::SshKeys => Slot::Ssh,
        Tab::Locations => Slot::LocationDetail,
        Tab::Dns => Slot::ZoneDetail,
        Tab::Account => Slot::Account,
    }
}

/// An id that appears in both lists stays virtual. Dedicated is the metal-only id.
pub fn merge_servers(
    virtuals: Vec<ServerRow>, metals: Vec<ServerRow>,
) -> Vec<ServerRow> {
    let mut seen = std::collections::HashSet::new();
    let mut rows = Vec::new();
    for mut row in virtuals {
        if !seen.insert(row.mbpkgid) {
            continue;
        }
        row.kind = ServerKind::Virtual;
        rows.push(row);
    }
    for mut row in metals {
        if !seen.insert(row.mbpkgid) {
            continue;
        }
        row.kind = ServerKind::Dedicated;
        rows.push(row);
    }
    rows.sort_by_key(|row| row.mbpkgid);
    rows
}

pub fn power_call_text(
    kind: ServerKind, verb: PowerVerb, mbpkgid: u32, force: bool,
) -> String {
    let mut text =
        format!("{} {} -m {mbpkgid}", kind.command(), verb.command());
    if force {
        text.push_str(" --force");
    }
    text
}

pub fn power_confirm_text(
    kind: ServerKind, verb: PowerVerb, mbpkgid: u32, fqdn: &str, force: bool,
) -> String {
    format!(
        "{}\nmbpkgid {mbpkgid}\nfqdn {fqdn}\nkind {}\n{}",
        verb.title(),
        kind.label(),
        power_call_text(kind, verb, mbpkgid, force)
    )
}

/// `true` is sent. `false` is omitted so the API body skips the field.
pub fn force_flag(force: bool) -> Option<bool> {
    force.then_some(true)
}

pub fn api_host(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    let rest = rest.split(['/', '?', '#']).next().unwrap_or(rest);
    let rest = rest.rsplit('@').next().unwrap_or(rest);
    let host = if let Some(inside) = rest.strip_prefix('[') {
        inside.split(']').next().unwrap_or(rest)
    } else {
        rest.split(':').next().unwrap_or(rest)
    };
    if host.is_empty() {
        "unknown".to_string()
    } else {
        host.to_string()
    }
}

const LOG_KEEP: usize = 48;

fn loaded(plural: &str, count: usize) -> String {
    let word = if count == 1 {
        plural.trim_end_matches('s')
    } else {
        plural
    };
    format!("loaded {count} {word}")
}

/// The create result names the key. The line stays that short, so a key body
/// in the payload is not written into the log.
fn ssh_created_line(detail: &str) -> String {
    let name = detail.split("Name:").nth(1).and_then(|rest| {
        let name = rest.trim().split(',').next().unwrap_or("").trim();
        (!name.is_empty()).then(|| name.to_string())
    });
    match name {
        Some(name) => format!("ssh key created: {name}"),
        None => "ssh key created".to_string(),
    }
}

/// One status line. A private key or an API key is not kept.
fn status_text(text: String) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let flat = redact_private_keys(&flat);
    let flat = redact_assignments(&flat);
    let max = 160;
    let count = flat.chars().count();
    if count <= max {
        return flat;
    }
    let mut out: String = flat.chars().take(max - 1).collect();
    out.push('…');
    out
}

fn redact_private_keys(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some((start, header)) = find_private_begin(rest) {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        out.push_str("[redacted]");
        rest = match find_private_end(&after[header..]) {
            Some(end) => &after[header + end..],
            None => "",
        };
    }
    out.push_str(rest);
    out
}

fn find_private_begin(text: &str) -> Option<(usize, usize)> {
    let lower = text.to_ascii_lowercase();
    let markers = [
        "-----begin openssh private key-----",
        "-----begin rsa private key-----",
        "-----begin ec private key-----",
        "-----begin dsa private key-----",
        "-----begin private key-----",
    ];
    markers
        .iter()
        .filter_map(|marker| {
            lower.find(marker).map(|index| (index, marker.len()))
        })
        .min_by_key(|(index, _)| *index)
}

fn find_private_end(text: &str) -> Option<usize> {
    let lower = text.to_ascii_lowercase();
    let start = lower.find("-----end ")?;
    let from = start + "-----end ".len();
    let rel = lower[from..].find("-----")?;
    Some(from + rel + "-----".len())
}

fn redact_assignments(text: &str) -> String {
    const NAMES: [&str; 4] = ["api_key", "apikey", "password", "key"];
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < bytes.len() {
        if let Some(name) = name_at(text, i, &NAMES) {
            let boundary = i == 0 || !is_name_byte(bytes[i - 1]);
            if boundary {
                let mut j = i + name.len();
                while j < bytes.len() && bytes[j] == b' ' {
                    j += 1;
                }
                if j < bytes.len() && bytes[j] == b'=' {
                    j += 1;
                    while j < bytes.len() && bytes[j] == b' ' {
                        j += 1;
                    }
                    while j < bytes.len()
                        && bytes[j] != b'&'
                        && bytes[j] != b' '
                    {
                        j += 1;
                    }
                    out.push_str(name);
                    out.push_str("=[redacted]");
                    i = j;
                    continue;
                }
            }
        }
        let ch = text[i..].chars().next().unwrap_or(' ');
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn name_at<'a>(text: &str, index: usize, names: &[&'a str]) -> Option<&'a str> {
    let rest = text.get(index..)?;
    names.iter().copied().find(|name| {
        rest.len() >= name.len()
            && rest.as_bytes()[..name.len()]
                .eq_ignore_ascii_case(name.as_bytes())
    })
}

fn is_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

pub fn short_fingerprint(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() <= 28 {
        return value.to_string();
    }
    let head: String = chars.iter().take(18).collect();
    let tail: String = chars.iter().skip(chars.len() - 4).collect();
    format!("{head}…{tail}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: u32, kind: ServerKind) -> ServerRow {
        ServerRow::listed(id, &format!("host-{id}.example"), "running", kind)
    }

    fn servers(ids: &[u32]) -> App {
        let mut app = App::new("vapi2.netactuate.com");
        app.servers =
            ids.iter().map(|id| row(*id, ServerKind::Virtual)).collect();
        app.servers_selected = ids.first().copied().map(u64::from);
        app.tab = Tab::Servers;
        app
    }

    #[test]
    fn the_startup_outline_is_the_tab_then_the_first_row() {
        let mut app = App::new("vapi2.netactuate.com");
        assert_eq!(app.focus, Focus::Tab(Tab::Servers));
        assert!(app.startup_outline);
        app.boot();
        let token = one_server_request(&mut app);
        app.apply(Reply::Servers {
            token,
            result: Ok(ServerBatch {
                rows: vec![
                    row(1001, ServerKind::Virtual),
                    row(1002, ServerKind::Virtual),
                ],
                warning: None,
            }),
        });
        assert_eq!(app.focus, Focus::Row(0));
        assert_eq!(app.servers_selected, Some(1001));
        assert!(app.startup_tab_focused(Tab::Servers));
        app.focus_move(false);
        assert!(!app.startup_outline);
        assert_eq!(app.focus, Focus::Row(1));
        assert!(!app.startup_tab_focused(Tab::Servers));
    }

    #[test]
    fn tab_moves_focus_and_does_not_select() {
        let mut app = servers(&[1001, 1002]);
        app.focus = Focus::Row(0);
        app.focus_move(false);
        assert_eq!(app.focus, Focus::Row(1));
        assert_eq!(app.servers_selected, Some(1001));
        app.focus_move(false);
        assert_eq!(app.focus, Focus::Detail(DetailControl::On));
        assert_eq!(app.servers_selected, Some(1001));
        app.focus = Focus::Tab(Tab::Servers);
        app.focus_move(false);
        assert_eq!(app.focus, Focus::Tab(Tab::SshKeys));
        assert_eq!(app.tab, Tab::Servers);
    }

    #[test]
    fn popup_opens_on_confirm_and_tab_stays_inside_it() {
        let mut app = servers(&[1001]);
        app.focus = Focus::Detail(DetailControl::Reboot);
        app.open_power(PowerVerb::Reboot);
        assert_eq!(
            app.power.as_ref().map(|popup| popup.focus),
            Some(PopupFocus::Confirm)
        );
        app.focus_move(false);
        app.focus_move(false);
        app.focus_move(false);
        assert_eq!(
            app.power.as_ref().map(|popup| popup.focus),
            Some(PopupFocus::Confirm)
        );
        assert_eq!(app.focus, Focus::Detail(DetailControl::Reboot));
        app.focus_move(true);
        assert_eq!(
            app.power.as_ref().map(|popup| popup.focus),
            Some(PopupFocus::Force)
        );
        app.cancel_popup();
        assert!(app.power.is_none());
        assert_eq!(app.focus, Focus::Detail(DetailControl::Reboot));
    }

    #[test]
    fn a_stale_response_is_ignored() {
        let mut app = servers(&[1001]);
        let first = app.issue(Slot::Servers, app.selection_gen);
        let second = app.issue(Slot::Servers, app.selection_gen);
        app.apply(Reply::Servers {
            token: first,
            result: Ok(ServerBatch {
                rows: vec![row(9, ServerKind::Virtual)],
                warning: None,
            }),
        });
        assert_eq!(app.servers[0].mbpkgid, 1001);
        app.apply(Reply::Servers {
            token: second,
            result: Ok(ServerBatch {
                rows: vec![
                    row(1001, ServerKind::Virtual),
                    row(1002, ServerKind::Virtual),
                ],
                warning: None,
            }),
        });
        app.select_id(1002);
        let stale_detail = app.issue(Slot::ServerDetail, 1001);
        app.select_id(1002);
        let _fresh = app.issue(Slot::ServerDetail, 1002);
        app.apply(Reply::ServerDetail {
            token: stale_detail,
            result: Ok(ServerDetail {
                fqdn: Some("stale.example".into()),
                state: Some("stopped".into()),
                status: Some("stale".into()),
                ip: None,
                location: None,
                extras: Vec::new(),
                error: None,
                plan_id: None,
                pkg_id: None,
            }),
        });
        assert_eq!(app.servers_selected, Some(1002));
        assert_ne!(
            app.selected_server().map(|row| row.fqdn.as_str()),
            Some("stale.example")
        );
        assert_ne!(app.server_detail_for, Some(1001));

        let late_list = app.issue(Slot::Servers, app.selection_gen);
        app.activate_tab(Tab::Dns);
        app.apply(Reply::Servers {
            token: late_list,
            result: Ok(ServerBatch {
                rows: vec![row(7, ServerKind::Dedicated)],
                warning: None,
            }),
        });
        assert_eq!(app.tab, Tab::Dns);
        assert!(app.servers.iter().all(|row| row.mbpkgid != 7));
        assert_eq!(app.status_lines().len(), 1);
        assert_eq!(app.status_lines()[0].text, "loaded 2 servers");
        assert_eq!(app.status_lines()[0].level, Level::Ok);
    }

    #[test]
    fn server_refresh_updates_the_open_list_without_a_status_line() {
        let mut app = servers(&[1001]);
        app.refresh_servers();
        let token = one_server_request(&mut app);
        app.refresh_servers();
        assert!(app.drain_requests().is_empty());
        assert!(app.activity().is_none());

        let mut updated = row(1001, ServerKind::Virtual);
        updated.state = "stopped".into();
        app.apply(Reply::Servers {
            token,
            result: Ok(ServerBatch {
                rows: vec![updated],
                warning: None,
            }),
        });
        assert_eq!(app.servers[0].state, "stopped");
        assert!(app.status_lines().is_empty());

        let detail = app.drain_requests();
        assert_eq!(detail.len(), 1);
        let Request::ServerDetail(query) = &detail[0] else {
            panic!("expected a server detail request, got {detail:?}");
        };
        let detail_token = query.token;
        assert!(app.activity().is_none());
        app.refresh_servers();
        assert!(app.drain_requests().is_empty());

        app.apply(Reply::ServerDetail {
            token: detail_token,
            result: Ok(ServerDetail {
                fqdn: None,
                state: Some("stopped".into()),
                status: None,
                ip: None,
                location: None,
                extras: Vec::new(),
                error: None,
                plan_id: None,
                pkg_id: None,
            }),
        });
        assert!(app.status_lines().is_empty());

        app.tab = Tab::SshKeys;
        app.refresh_servers();
        assert!(app.drain_requests().is_empty());

        app.tab = Tab::Servers;
        app.boot();
        let token = one_server_request(&mut app);
        app.apply(Reply::Servers {
            token,
            result: Ok(ServerBatch {
                rows: vec![row(1001, ServerKind::Virtual)],
                warning: None,
            }),
        });
        assert_eq!(app.status_lines()[0].text, "loaded 1 server");
    }

    fn one_server_request(app: &mut App) -> Token {
        match app.drain_requests().as_slice() {
            [Request::Servers(token)] => *token,
            other => {
                panic!("expected one server list request, got {other:?}")
            }
        }
    }

    #[test]
    fn a_slow_list_does_not_replace_a_newer_selection() {
        let mut app = servers(&[1001, 1002]);
        app.selection_gen = 4;
        app.servers_selected = Some(1001);
        let token = app.issue(Slot::Servers, 4);
        app.select_id(1002);
        app.pending.clear();
        app.apply(Reply::Servers {
            token,
            result: Ok(ServerBatch {
                rows: vec![
                    row(1001, ServerKind::Virtual),
                    row(1002, ServerKind::Virtual),
                ],
                warning: None,
            }),
        });
        assert_eq!(app.servers_selected, Some(1002));
    }

    #[test]
    fn power_confirm_text_names_virtual_and_dedicated_calls() {
        let virtual_text = power_confirm_text(
            ServerKind::Virtual,
            PowerVerb::Reboot,
            1001,
            "web-01.example",
            false,
        );
        assert!(virtual_text.contains("mbpkgid 1001"), "{virtual_text}");
        assert!(virtual_text.contains("web-01.example"), "{virtual_text}");
        assert!(virtual_text.contains("virtual"), "{virtual_text}");
        assert!(
            virtual_text.contains("power-virt reboot -m 1001"),
            "{virtual_text}"
        );
        assert!(!virtual_text.contains("power-ded"), "{virtual_text}");
        assert!(!virtual_text.contains("--force"), "{virtual_text}");

        let dedicated = power_confirm_text(
            ServerKind::Dedicated,
            PowerVerb::Off,
            1004,
            "metal-01.example",
            true,
        );
        assert!(dedicated.contains("dedicated"), "{dedicated}");
        assert!(dedicated.contains("metal-01.example"), "{dedicated}");
        assert!(
            dedicated.contains("power-ded off -m 1004 --force"),
            "{dedicated}"
        );
        assert!(!dedicated.contains("power-virt"), "{dedicated}");
    }

    #[test]
    fn confirm_is_ignored_while_a_power_call_is_running() {
        let mut app = servers(&[1001]);
        app.open_power(PowerVerb::On);
        app.confirm_power();
        assert_eq!(
            app.pending
                .iter()
                .filter(|request| matches!(request, Request::Power(_)))
                .count(),
            1
        );
        app.open_power(PowerVerb::Reboot);
        app.confirm_power();
        assert_eq!(
            app.pending
                .iter()
                .filter(|request| matches!(request, Request::Power(_)))
                .count(),
            1
        );
        assert_eq!(
            app.banner.as_ref().map(|banner| banner.text.as_str()),
            Some("A power change is already running")
        );
    }

    #[test]
    fn a_cancelled_power_result_is_ignored() {
        let mut app = servers(&[1004]);
        app.servers[0].kind = ServerKind::Dedicated;
        app.open_power(PowerVerb::Off);
        app.confirm_power();
        let token = match app.pending.last() {
            Some(Request::Power(command)) => command.token,
            other => panic!("expected a power request, got {other:?}"),
        };
        app.revoke_power_session(token.request_id);
        app.apply(Reply::Power {
            token,
            result: Ok("power-ded off -m 1004".into()),
        });
        assert!(app.banner.is_none());
        assert!(app.power_inflight.is_none());
        assert!(app.status_lines().is_empty());
    }

    #[test]
    fn applied_responses_are_logged_and_stale_ones_are_not() {
        let mut app = servers(&[1001]);
        let stale = app.issue(Slot::Servers, app.selection_gen);
        let fresh = app.issue(Slot::Servers, app.selection_gen);
        app.apply(Reply::Servers {
            token: stale,
            result: Err("key=super-secret stale".into()),
        });
        assert!(app.status_lines().is_empty());
        app.apply(Reply::Servers {
            token: fresh,
            result: Ok(ServerBatch {
                rows: vec![row(1001, ServerKind::Virtual)],
                warning: Some("virtual: key=super-secret-key".into()),
            }),
        });
        let joined = app
            .status_lines()
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(joined.contains("loaded 1 server"), "{joined}");
        assert!(joined.contains("key=[redacted]"), "{joined}");
        assert!(!joined.contains("super-secret"), "{joined}");
        assert!(!joined.contains("stale"), "{joined}");

        app.pending.clear();
        app.open_power(PowerVerb::Reboot);
        app.confirm_power();
        let token = match app
            .pending
            .iter()
            .find(|request| matches!(request, Request::Power(_)))
        {
            Some(Request::Power(command)) => command.token,
            other => panic!("expected a power request, got {other:?}"),
        };
        let before = app.status_lines().len();
        app.revoke_power_session(token.request_id);
        app.apply(Reply::Power {
            token,
            result: Ok("power-virt reboot -m 1001".into()),
        });
        assert_eq!(app.status_lines().len(), before);

        app.open_power(PowerVerb::Reboot);
        app.confirm_power();
        let token = match app
            .pending
            .iter()
            .rev()
            .find(|request| matches!(request, Request::Power(_)))
        {
            Some(Request::Power(command)) => command.token,
            other => panic!("expected a power request, got {other:?}"),
        };
        app.apply(Reply::Power {
            token,
            result: Ok(
                "power-virt reboot -m 1001  -----BEGIN OPENSSH PRIVATE KEY-----\nsecret\n-----END OPENSSH PRIVATE KEY-----"
                    .into(),
            ),
        });
        let joined = app
            .status_lines()
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(joined.contains("power-virt reboot -m 1001"), "{joined}");
        assert!(joined.contains("[redacted]"), "{joined}");
        assert!(!joined.contains("PRIVATE KEY"), "{joined}");
        assert!(
            app.status_lines()
                .iter()
                .all(|line| !line.text.contains('\n'))
        );

        let dropped = app.issue(Slot::SshCreate, 0);
        let created = app.issue(Slot::SshCreate, 0);
        app.apply(Reply::SshCreated {
            token: dropped,
            result: Ok("ID: 9, Name: dropped-key, Fingerprint: aa".into()),
        });
        assert!(
            app.status_lines()
                .iter()
                .all(|line| !line.text.contains("dropped-key"))
        );
        app.apply(Reply::SshCreated {
            token: created,
            result: Ok("ID: 1001, Name: laptop, Fingerprint: SHA256:ab".into()),
        });
        assert!(
            app.status_lines()
                .iter()
                .any(|line| line.text == "ssh key created: laptop")
        );
        app.apply(Reply::SshCreated {
            token: created,
            result: Ok("-----BEGIN OPENSSH PRIVATE KEY----- secret".into()),
        });
        assert!(
            app.status_lines()
                .iter()
                .all(|line| !line.text.contains("PRIVATE"))
        );
    }

    #[test]
    fn an_id_in_both_lists_is_virtual() {
        let rows = merge_servers(
            vec![row(1001, ServerKind::Virtual)],
            vec![
                row(1001, ServerKind::Dedicated),
                row(1004, ServerKind::Dedicated),
            ],
        );
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].mbpkgid, 1001);
        assert_eq!(rows[0].kind, ServerKind::Virtual);
        assert_eq!(rows[1].mbpkgid, 1004);
        assert_eq!(rows[1].kind, ServerKind::Dedicated);
    }

    #[test]
    fn force_is_omitted_unless_it_is_set() {
        assert_eq!(force_flag(false), None);
        assert_eq!(force_flag(true), Some(true));
    }

    #[test]
    fn the_host_line_drops_userinfo() {
        let host = api_host("https://user:secret@vapi2.netactuate.com/api/");
        assert_eq!(host, "vapi2.netactuate.com");
        assert!(!host.contains("secret"));
    }

    #[test]
    fn server_panes_follow_the_right_actions_and_enter_selects_one() {
        let mut app = servers(&[1001, 1002]);
        app.focus = Focus::Action(Action::DeleteServer);
        app.focus_move(false);
        assert_eq!(app.focus, Focus::ServerPane(ServerPane::Jobs));
        assert_eq!(app.server_pane, ServerPane::Jobs);
        assert!(app.drain_requests().is_empty());
        app.focus_move(false);
        assert_eq!(app.focus, Focus::ServerPane(ServerPane::Address));
        assert_eq!(app.server_pane, ServerPane::Jobs);
        app.activate_focused();
        assert_eq!(app.server_pane, ServerPane::Address);
        assert_eq!(app.focus, Focus::ServerPane(ServerPane::Address));
        match app.drain_requests().as_slice() {
            [Request::ServerList(query)] => {
                assert_eq!(query.mbpkgid, 1001);
                assert_eq!(query.pane, ServerPane::Address);
                assert_eq!(query.token.selection, 1001);
            }
            other => panic!("expected one address list, got {other:?}"),
        }
    }

    #[test]
    fn a_previous_servers_list_does_not_paint() {
        let mut app = servers(&[1001, 1002]);
        let stale = app.issue(Slot::ServerList, 1001);
        app.select_id(1002);
        app.pending.clear();
        app.apply(Reply::ServerList {
            token: stale,
            pane: ServerPane::Jobs,
            lines: vec!["stale job".into()],
            errors: vec!["jobs: stale".into()],
        });
        assert!(app.server_lines.iter().all(|line| line != "stale job"));
        assert!(
            app.status_lines()
                .iter()
                .all(|line| !line.text.contains("stale"))
        );
        let fresh = app.issue(Slot::ServerList, 1002);
        app.apply(Reply::ServerList {
            token: fresh,
            pane: ServerPane::Jobs,
            lines: vec!["9  2024-01-02  1  reboot".into()],
            errors: Vec::new(),
        });
        assert_eq!(app.server_lines.len(), 1);
        assert!(app.server_lines[0].contains("reboot"));
        assert_eq!(app.server_lines_for, Some(1002));
        assert!(
            app.status_lines()
                .iter()
                .any(|line| line.text == "jobs loaded"
                    && line.level == Level::Ok)
        );
    }

    #[test]
    fn a_list_failure_appends_one_status_line() {
        let mut app = servers(&[1001]);
        app.server_pane = ServerPane::Address;
        let token = app.issue(Slot::ServerList, 1001);
        let before = app.status_lines().len();
        app.apply(Reply::ServerList {
            token,
            pane: ServerPane::Address,
            lines: vec!["ipv6  2001:db8::10  64  2001:db8::1".into()],
            errors: vec!["ipv4: down".into()],
        });
        assert_eq!(app.status_lines().len(), before + 1);
        assert_eq!(app.status_lines().last().unwrap().text, "ipv4: down");
        assert_eq!(app.status_lines().last().unwrap().level, Level::Error);
        assert_eq!(app.server_lines.len(), 1);
        assert!(app.server_lines_ready());
    }

    #[test]
    fn arrows_on_a_server_pane_stay_inside_its_list() {
        let mut app = servers(&[1001, 1002]);
        app.focus = Focus::ServerPane(ServerPane::Jobs);
        app.server_lines = vec!["a".into(), "b".into(), "c".into()];
        app.server_lines_for = Some(1001);
        app.server_lines_pane = ServerPane::Jobs;
        app.server_line_window = 2;
        app.move_list(1);
        app.move_list(1);
        assert_eq!(app.server_line, 2);
        assert_eq!(app.detail_scroll, 1);
        assert_eq!(app.servers_selected, Some(1001));
        assert_eq!(app.focus, Focus::ServerPane(ServerPane::Jobs));
        app.move_list(1);
        assert_eq!(app.server_line, 2);
        app.move_list(-1);
        assert_eq!(app.server_line, 1);
    }

    #[test]
    fn tabs_cover_the_bar_in_order() {
        let labels: Vec<_> =
            crate::focus::TABS.iter().map(|tab| tab.label()).collect();
        assert_eq!(
            labels,
            ["Servers", "SSH keys", "Locations", "DNS", "Account"]
        );
    }

    #[test]
    fn delete_server_asks_for_the_hostname_then_sends() {
        let mut app = servers(&[1001]);
        app.servers[0].fqdn = "web-01.example".into();
        app.focus = Focus::Action(Action::DeleteServer);
        app.activate_focused();
        let prompt = app.prompt.as_ref().expect("hostname");
        assert!(prompt.title.contains("web-01.example"), "{}", prompt.title);
        assert!(prompt.hint.contains("1001"), "{}", prompt.hint);
        assert_eq!(prompt.fields.len(), 1);
        assert_eq!(prompt.fields[0].label, "hostname");
        assert!(prompt.fields[0].is_text());
        assert!(app.pending.is_empty());

        app.type_char('x');
        app.activate_focused();
        let prompt = app.prompt.as_ref().expect("stays on the hostname");
        assert!(prompt.hint.contains("does not match"), "{}", prompt.hint);
        assert!(prompt.fields.iter().any(|field| field.label == "hostname"));
        assert!(app.pending.is_empty());

        app.backspace();
        for ch in "web-01.example".chars() {
            app.type_char(ch);
        }
        app.activate_focused();
        match app.pending.as_slice() {
            [Request::Write {
                op: WriteOp::DeleteServer { mbpkgid, fqdn },
                ..
            }] => {
                assert_eq!(*mbpkgid, 1001);
                assert_eq!(fqdn, "web-01.example");
            }
            other => panic!("expected a server delete, got {other:?}"),
        }
        assert!(app.prompt.is_none());
    }

    #[test]
    fn delete_does_not_wait_for_a_usage_contract() {
        let mut app = servers(&[1001]);
        app.servers[0].fqdn = "web-01.example".into();
        app.usage_billing = Some(UsageBilling {
            contract_type: "usage".into(),
            id: "21".into(),
        });
        app.focus = Focus::Action(Action::DeleteServer);
        app.activate_focused();
        for ch in "web-01.example".chars() {
            app.type_char(ch);
        }
        app.activate_focused();
        match app.pending.as_slice() {
            [Request::Write {
                op: WriteOp::DeleteServer { mbpkgid, fqdn },
                ..
            }] => {
                assert_eq!(*mbpkgid, 1001);
                assert_eq!(fqdn, "web-01.example");
            }
            other => panic!("expected a server delete, got {other:?}"),
        }
    }

    #[test]
    fn new_key_still_uses_the_file_field_and_delete_names_the_key() {
        let mut app = App::new("vapi2.netactuate.com");
        app.tab = Tab::SshKeys;
        app.keys = vec![SshRow {
            id: 9,
            name: "laptop".into(),
            fingerprint: "aa".into(),
        }];
        app.ssh_selected = Some(9);
        app.focus = Focus::Action(Action::NewKey);
        app.activate_focused();
        app.type_char('k');
        app.activate_focused();
        let token = match app.drain_requests().as_slice() {
            [
                Request::SshCreate {
                    token,
                    file: Some(file),
                },
            ] => {
                assert_eq!(file, "k");
                *token
            }
            other => panic!("expected a new key, got {other:?}"),
        };
        app.apply(Reply::SshCreated {
            token,
            result: Ok("ID: 1, Name: k, Fingerprint: aa".into()),
        });
        app.drain_requests();

        app.focus = Focus::Action(Action::DeleteKey);
        app.activate_focused();
        let prompt = app.prompt.as_ref().expect("confirm");
        assert!(prompt.title.contains("laptop"), "{}", prompt.title);
        assert!(prompt.hint.contains('9'), "{}", prompt.hint);
        assert!(app.pending.is_empty());
        app.activate_focused();
        match app.pending.as_slice() {
            [
                Request::Write {
                    op: WriteOp::DeleteKey { id, name },
                    ..
                },
            ] => {
                assert_eq!(*id, 9);
                assert_eq!(name, "laptop");
            }
            other => panic!("expected a key delete, got {other:?}"),
        }
    }

    #[test]
    fn delete_record_confirms_the_loaded_name() {
        let mut app = App::new("vapi2.netactuate.com");
        app.tab = Tab::Dns;
        app.zones = vec![ZoneRow {
            id: 4,
            name: "example.com".into(),
            zone_type: "NATIVE".into(),
        }];
        app.zone_selected = Some(4);
        app.zone_records = vec![DnsRecord {
            id: 12,
            name: "www".into(),
            record_type: "A".into(),
            content: "192.0.2.1".into(),
            ttl: Some(300),
            prio: None,
        }];
        app.focus = Focus::Action(Action::DeleteRecord);
        app.activate_focused();
        app.type_char('1');
        app.type_char('2');
        app.activate_focused();
        let prompt = app.prompt.as_ref().expect("confirm");
        assert!(prompt.title.contains("www"), "{}", prompt.title);
        assert!(prompt.hint.contains("12"), "{}", prompt.hint);
        assert!(prompt.hint.contains("example.com"), "{}", prompt.hint);
        assert!(app.pending.is_empty());
        app.activate_focused();
        match app.pending.as_slice() {
            [
                Request::Write {
                    op: WriteOp::DeleteRecord { id, name, zone },
                    ..
                },
            ] => {
                assert_eq!(*id, 12);
                assert_eq!(name, "www");
                assert_eq!(zone, "example.com");
            }
            other => panic!("expected a record delete, got {other:?}"),
        }
    }

    #[test]
    fn an_empty_server_form_is_not_sent_and_a_stale_write_is_dropped() {
        let mut app = servers(&[1001]);
        app.focus = Focus::Action(Action::CreateServer);
        app.activate_focused();
        let fields = app.prompt.as_ref().expect("form").fields.len();
        app.drain_requests();
        for _ in 0..fields {
            app.activate_focused();
        }
        let hint = &app.prompt.as_ref().expect("form").hint;
        assert!(hint.contains("still loading"), "{hint}");
        assert!(
            app.pending
                .iter()
                .all(|request| !matches!(request, Request::Write { .. }))
        );
        assert!(
            app.status_lines()
                .iter()
                .any(|line| line.level == Level::Error
                    && line.text.contains("still loading"))
        );

        let stale = app.issue(Slot::Write, 0);
        let fresh = app.issue(Slot::Write, 0);
        app.apply(Reply::Write {
            token: stale,
            result: Ok("stale write".into()),
        });
        assert!(
            app.status_lines()
                .iter()
                .all(|line| !line.text.contains("stale write"))
        );
        app.apply(Reply::Write {
            token: fresh,
            result: Ok("server deleted: web-01.example".into()),
        });
        assert!(
            app.status_lines()
                .iter()
                .any(|line| line.text == "server deleted: web-01.example")
        );
        assert!(matches!(app.pending.last(), Some(Request::Servers(_))));
    }

    #[test]
    fn create_server_dropdowns_send_the_selected_ids() {
        let mut app = servers(&[1001]);
        app.focus = Focus::Action(Action::CreateServer);
        app.activate_focused();
        let token = match app.drain_requests().as_slice() {
            [Request::Catalog(token)] => *token,
            other => panic!("expected a catalog request, got {other:?}"),
        };
        let plan = app.prompt.as_ref().unwrap().fields[1].shown();
        assert_eq!(plan, "loading");
        app.apply(Reply::Catalog {
            token,
            result: Ok(BuildChoices {
                plans: listed(vec![choice("VR1x2  ram 2048", "VR1x2")]),
                locations: listed(vec![choice("Ashburn  IAD", "42")]),
                images: listed(vec![choice("Ubuntu 24.04", "55")]),
                keys: listed(vec![choice("laptop", "12")]),
                billing: Some(UsageBilling {
                    contract_type: "metered".into(),
                    id: "7".into(),
                }),
                billing_error: None,
            }),
        });
        app.type_char('w');
        assert_eq!(app.prompt.as_ref().unwrap().fields[0].value, "w");
        app.activate_focused();
        assert!(app.nudge_choice(1));
        assert_eq!(
            app.prompt.as_ref().unwrap().fields[1].shown(),
            "VR1x2  ram 2048"
        );
        assert_eq!(app.prompt.as_ref().unwrap().fields[1].value, "VR1x2");
        app.type_char('1');
        assert_eq!(app.prompt.as_ref().unwrap().fields[1].value, "VR1x2");
        app.activate_focused();
        app.nudge_choice(1);
        app.activate_focused();
        app.nudge_choice(1);
        app.activate_focused();
        app.nudge_choice(1);
        assert_eq!(app.prompt.as_ref().unwrap().fields[4].shown(), "laptop");
        assert_eq!(app.prompt.as_ref().unwrap().fields[4].value, "12");
        app.activate_focused();
        match app.pending.as_slice() {
            [
                Request::Write {
                    op: WriteOp::CreateServer(fields),
                    ..
                },
            ] => {
                assert_eq!(fields.fqdn.as_deref(), Some("w"));
                assert_eq!(fields.plan.as_deref(), Some("VR1x2"));
                assert_eq!(fields.location, Some(42));
                assert_eq!(fields.image, Some(55));
                assert_eq!(fields.ssh_key_id, Some(12));
                assert_eq!(fields.package_billing.as_deref(), Some("metered"));
                assert_eq!(
                    fields.package_billing_contract_id.as_deref(),
                    Some("7")
                );
            }
            other => panic!("expected a server create, got {other:?}"),
        }
    }

    #[test]
    fn a_missing_usage_contract_still_sends_the_other_buy_fields() {
        let mut app = servers(&[1001]);
        app.focus = Focus::Action(Action::CreateServer);
        app.activate_focused();
        let token = match app.drain_requests().as_slice() {
            [Request::Catalog(token)] => *token,
            other => panic!("expected a catalog request, got {other:?}"),
        };
        let mut choices = sample_choices();
        choices.billing_error = Some("connection failed".into());
        app.apply(Reply::Catalog {
            token,
            result: Ok(choices),
        });
        assert_eq!(app.prompt.as_ref().unwrap().fields[1].shown(), "choose");
        assert!(app.status_lines().iter().any(|line| {
            line.level == Level::Error
                && line.text.contains("usage contract: connection failed")
        }));
        submit_filled_create(&mut app);
        match app.pending.as_slice() {
            [
                Request::Write {
                    op: WriteOp::CreateServer(fields),
                    ..
                },
            ] => {
                assert_eq!(fields.fqdn.as_deref(), Some("w"));
                assert_eq!(fields.plan.as_deref(), Some("VR1x2"));
                assert_eq!(fields.location, Some(42));
                assert_eq!(fields.image, Some(55));
                assert_eq!(fields.ssh_key_id, Some(12));
                assert!(fields.package_billing.is_none());
                assert!(fields.package_billing_contract_id.is_none());
                let dumped = format!("{fields:?}");
                assert!(!dumped.contains("ssh-ed25519"));
                assert!(!dumped.contains("PRIVATE"));
            }
            other => panic!("expected a server create, got {other:?}"),
        }
    }

    #[test]
    fn an_ambiguous_usage_contract_is_named_and_still_sent() {
        let mut app = servers(&[1001]);
        app.focus = Focus::Action(Action::CreateServer);
        app.activate_focused();
        let token = match app.drain_requests().as_slice() {
            [Request::Catalog(token)] => *token,
            other => panic!("expected a catalog request, got {other:?}"),
        };
        let mut choices = sample_choices();
        choices.billing = Some(UsageBilling {
            contract_type: "hourly".into(),
            id: "21".into(),
        });
        choices.billing_error =
            Some("several contracts have is_free 1; using the first".into());
        app.apply(Reply::Catalog {
            token,
            result: Ok(choices),
        });
        assert!(
            app.status_lines()
                .iter()
                .any(|line| { line.text.contains("using the first") })
        );
        submit_filled_create(&mut app);
        match app.pending.as_slice() {
            [
                Request::Write {
                    op: WriteOp::CreateServer(fields),
                    ..
                },
            ] => {
                assert_eq!(fields.package_billing.as_deref(), Some("hourly"));
                assert_eq!(
                    fields.package_billing_contract_id.as_deref(),
                    Some("21")
                );
                assert_eq!(fields.ssh_key_id, Some(12));
                assert!(fields.plan.is_some());
                assert!(fields.location.is_some());
                assert!(fields.image.is_some());
                assert!(fields.fqdn.is_some());
            }
            other => panic!("expected a server create, got {other:?}"),
        }
    }

    fn submit_filled_create(app: &mut App) {
        app.type_char('w');
        app.activate_focused();
        assert!(app.nudge_choice(1));
        app.activate_focused();
        assert!(app.nudge_choice(1));
        app.activate_focused();
        assert!(app.nudge_choice(1));
        app.activate_focused();
        assert!(app.nudge_choice(1));
        app.activate_focused();
    }

    #[test]
    fn a_partial_create_is_not_sent() {
        let mut app = servers(&[1001]);
        app.focus = Focus::Action(Action::CreateServer);
        app.activate_focused();
        let token = match app.drain_requests().as_slice() {
            [Request::Catalog(token)] => *token,
            other => panic!("expected a catalog request, got {other:?}"),
        };
        app.type_char('w');
        submit_form(&mut app);
        assert_create_held(&app, "still loading");

        app.apply(Reply::Catalog {
            token,
            result: Err("catalog down".into()),
        });
        submit_form(&mut app);
        assert_create_held(&app, "unavailable");

        app.cancel_popup();
        app.focus = Focus::Action(Action::CreateServer);
        app.activate_focused();
        let token = match app.drain_requests().as_slice() {
            [Request::Catalog(token)] => *token,
            other => panic!("expected a catalog request, got {other:?}"),
        };
        let mut choices = sample_choices();
        choices.billing = Some(UsageBilling {
            contract_type: "metered".into(),
            id: "7".into(),
        });
        app.apply(Reply::Catalog {
            token,
            result: Ok(choices),
        });
        app.type_char('w');
        submit_form(&mut app);
        let hint = app.prompt.as_ref().expect("form").hint.clone();
        assert!(hint.contains("required"), "{hint}");
        assert!(hint.contains("plan"), "{hint}");
        assert!(hint.contains("ssh key"), "{hint}");
        assert_create_held(&app, "required");

        pick_choice(&mut app, 1);
        pick_choice(&mut app, 2);
        pick_choice(&mut app, 3);
        submit_form(&mut app);
        let hint = app.prompt.as_ref().expect("form").hint.clone();
        assert_eq!(hint, "ssh key is required");
        assert_create_held(&app, "ssh key is required");

        app.cancel_popup();
        app.focus = Focus::Action(Action::CreateServer);
        app.activate_focused();
        let token = match app.drain_requests().as_slice() {
            [Request::Catalog(token)] => *token,
            other => panic!("expected a catalog request, got {other:?}"),
        };
        let mut choices = sample_choices();
        choices.keys = Listed {
            options: Vec::new(),
            error: Some("keys down".into()),
        };
        app.apply(Reply::Catalog {
            token,
            result: Ok(choices),
        });
        app.type_char('w');
        pick_choice(&mut app, 1);
        pick_choice(&mut app, 2);
        pick_choice(&mut app, 3);
        submit_form(&mut app);
        let hint = app.prompt.as_ref().expect("form").hint.clone();
        assert_eq!(hint, "ssh key is unavailable");
        assert_create_held(&app, "ssh key is unavailable");
    }

    fn submit_form(app: &mut App) {
        app.activate_target(crate::focus::Target::Prompt(PromptFocus::Submit));
    }

    fn pick_choice(app: &mut App, index: usize) {
        app.activate_target(crate::focus::Target::Prompt(PromptFocus::Field(
            index,
        )));
        assert!(app.nudge_choice(1));
    }

    fn assert_create_held(app: &App, needle: &str) {
        let hint = &app.prompt.as_ref().expect("form stays open").hint;
        assert!(hint.contains(needle), "{hint}");
        assert!(
            app.pending
                .iter()
                .all(|request| !matches!(request, Request::Write { .. }))
        );
    }

    fn sample_choices() -> BuildChoices {
        BuildChoices {
            plans: listed(vec![choice("VR1x2  ram 2048", "VR1x2")]),
            locations: listed(vec![choice("Ashburn  IAD", "42")]),
            images: listed(vec![choice("Ubuntu 24.04", "55")]),
            keys: listed(vec![choice("laptop", "12")]),
            billing: None,
            billing_error: None,
        }
    }

    fn choice(label: &str, value: &str) -> Choice {
        Choice {
            label: label.to_string(),
            value: value.to_string(),
        }
    }

    fn listed(options: Vec<Choice>) -> Listed {
        Listed {
            options,
            error: None,
        }
    }
}
