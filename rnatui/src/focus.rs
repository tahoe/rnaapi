// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
//! Focus order. Tab and Shift-Tab only move the outline.

use ratatui::layout::{Position, Rect};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Servers,
    SshKeys,
    Locations,
    Dns,
    Account,
}

pub const TABS: [Tab; 5] = [
    Tab::Servers,
    Tab::SshKeys,
    Tab::Locations,
    Tab::Dns,
    Tab::Account,
];

impl Tab {
    pub fn label(self) -> &'static str {
        match self {
            Tab::Servers => "Servers",
            Tab::SshKeys => "SSH keys",
            Tab::Locations => "Locations",
            Tab::Dns => "DNS",
            Tab::Account => "Account",
        }
    }

    pub fn list_slot(self) -> Slot {
        match self {
            Tab::Servers => Slot::Servers,
            Tab::SshKeys => Slot::Ssh,
            Tab::Locations => Slot::Locations,
            Tab::Dns => Slot::Zones,
            Tab::Account => Slot::Account,
        }
    }
}

/// A button in a pane. Left buttons start something new. Right buttons act
/// on the selected row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    CreateServer,
    RebuildServer,
    DeleteServer,
    NewKey,
    UpdateKey,
    DeleteKey,
    CreateZone,
    DeleteZone,
    CreateRecord,
    UpdateRecord,
    DeleteRecord,
}

impl Action {
    pub fn label(self) -> &'static str {
        match self {
            Action::CreateServer => "Create server",
            Action::RebuildServer => "Rebuild",
            Action::DeleteServer => "Delete",
            Action::NewKey => "New key",
            Action::UpdateKey => "Update",
            Action::DeleteKey => "Delete",
            Action::CreateZone => "New zone",
            Action::DeleteZone => "Delete zone",
            Action::CreateRecord => "New record",
            Action::UpdateRecord => "Update record",
            Action::DeleteRecord => "Delete record",
        }
    }

    /// New-item buttons for this tab. They stay available with no selection.
    pub fn left(tab: Tab) -> &'static [Action] {
        match tab {
            Tab::Servers => &[Action::CreateServer],
            Tab::SshKeys => &[Action::NewKey],
            Tab::Dns => &[Action::CreateZone, Action::CreateRecord],
            Tab::Locations | Tab::Account => &[],
        }
    }

    /// Buttons that change the selected row. Hidden when nothing is selected.
    pub fn right(tab: Tab) -> &'static [Action] {
        match tab {
            Tab::Servers => &[Action::RebuildServer, Action::DeleteServer],
            Tab::SshKeys => &[Action::UpdateKey, Action::DeleteKey],
            Tab::Dns => &[
                Action::DeleteZone,
                Action::UpdateRecord,
                Action::DeleteRecord,
            ],
            Tab::Locations | Tab::Account => &[],
        }
    }

    pub fn is_left(self, tab: Tab) -> bool {
        Self::left(tab).contains(&self)
    }

    pub fn is_right(self, tab: Tab) -> bool {
        Self::right(tab).contains(&self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    Servers,
    ServerDetail,
    Ssh,
    SshCreate,
    Locations,
    LocationDetail,
    Zones,
    ZoneDetail,
    Account,
    Power,
    /// Jobs, addresses, or bandwidth for the selected server.
    ServerList,
    /// A create, update, or delete. A newer write drops the older response.
    Write,
    /// Plans, images, locations, and SSH keys for the create-server form.
    Catalog,
    /// `ContractUsage::get` when a delete confirm does not have a contract yet.
    Usage,
}

/// Subtabs under the server detail. Tab moves the outline.
/// Enter, Space, and a click select one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ServerPane {
    #[default]
    Jobs,
    Address,
    Bandwidth,
}

impl ServerPane {
    pub const ALL: [ServerPane; 3] =
        [ServerPane::Jobs, ServerPane::Address, ServerPane::Bandwidth];

    pub fn label(self) -> &'static str {
        match self {
            ServerPane::Jobs => "Jobs",
            ServerPane::Address => "Address",
            ServerPane::Bandwidth => "Bandwidth",
        }
    }

    pub fn loaded(self) -> &'static str {
        match self {
            ServerPane::Jobs => "jobs loaded",
            ServerPane::Address => "address loaded",
            ServerPane::Bandwidth => "bandwidth loaded",
        }
    }

    pub fn loading(self) -> &'static str {
        match self {
            ServerPane::Jobs => "Loading jobs",
            ServerPane::Address => "Loading addresses",
            ServerPane::Bandwidth => "Loading bandwidth",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DetailControl {
    On,
    Off,
    Reboot,
}

impl DetailControl {
    pub fn label(self) -> &'static str {
        match self {
            DetailControl::On => "On",
            DetailControl::Off => "Off",
            DetailControl::Reboot => "Reboot",
        }
    }
}

/// A draggable rule. It is not part of the Tab cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Split {
    /// The vertical rule between the list and the detail.
    Columns,
    /// The horizontal rule between the main panes and the log.
    Rows,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Focus {
    Tab(Tab),
    /// A pane button. Left buttons follow the tabs. Right buttons follow the list.
    Action(Action),
    Row(usize),
    Detail(DetailControl),
    ServerPane(ServerPane),
    Split(Split),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopupFocus {
    Force,
    Confirm,
    Cancel,
}

impl PopupFocus {
    pub const ORDER: [PopupFocus; 3] =
        [PopupFocus::Force, PopupFocus::Confirm, PopupFocus::Cancel];
}

/// Focus inside a small prompt. Fields come first, then the two buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromptFocus {
    Field(usize),
    Submit,
    Cancel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modal {
    None,
    Power,
    Prompt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Tab(Tab),
    Action(Action),
    Row(usize),
    Detail(DetailControl),
    ServerPane(ServerPane),
    Power(PopupFocus),
    Prompt(PromptFocus),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hit {
    pub target: Target,
    pub area: Rect,
}

/// List and detail stay wide enough to read. The log keeps a few rows.
pub const PANE_MIN: u16 = 20;
pub const LOG_MIN: u16 = 3;
pub const LOG_DEFAULT: u16 = 4;
/// Top bar, tab row, and key bar. Pane buttons sit inside the body.
/// The body and the log share what remains.
const CHROME: u16 = 1 + 3 + 1;
/// Kept so a caller can still reserve a band under the tabs. The screen
/// does not. Its buttons live at the top of each pane.
pub const ACTION_ROWS: u16 = 1;
/// Body rows kept so the frame still has an interior.
const MAIN_MIN: u16 = 6;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Screen {
    pub hits: Vec<Hit>,
    pub list: Rect,
    pub detail: Rect,
    pub list_window: u16,
    /// Columns of the list pane, not counting the vertical rule. `0` means
    /// the first frame still has to apply the starting 42% width.
    pub list_width: u16,
    pub log_rows: u16,
    pub area: Rect,
    pub top: Rect,
    pub tabs: Rect,
    pub actions: Rect,
    /// Fitted height of the actions row. `0` when this tab has no writes.
    pub action_rows: u16,
    pub body: Rect,
    pub log: Rect,
    pub hints: Rect,
    pub columns_split: Rect,
    pub rows_split: Rect,
    pub drag: Option<Split>,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            hits: Vec::new(),
            list: Rect::default(),
            detail: Rect::default(),
            list_window: 1,
            list_width: 0,
            log_rows: LOG_DEFAULT,
            area: Rect::default(),
            top: Rect::default(),
            tabs: Rect::default(),
            actions: Rect::default(),
            action_rows: 0,
            body: Rect::default(),
            log: Rect::default(),
            hints: Rect::default(),
            columns_split: Rect::default(),
            rows_split: Rect::default(),
            drag: None,
        }
    }
}

impl Screen {
    pub fn reset_hits(&mut self) {
        self.hits.clear();
    }

    pub fn hit(&mut self, target: Target, area: Rect) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        self.hits.push(Hit { target, area });
    }

    /// Place the panes for this frame and keep a resize that is already underway.
    /// `action_rows` is `ACTION_ROWS` when the open tab has writes, else `0`.
    pub fn layout(&mut self, area: Rect, action_rows: u16) {
        self.area = area;
        let action_rows = fitted_action_rows(area.height, action_rows);
        self.action_rows = action_rows;
        if self.log_rows == 0 {
            self.log_rows = LOG_DEFAULT;
        }
        self.log_rows =
            clamp_log_rows_for(self.log_rows, area.height, action_rows);
        let bands = bands(area, self.log_rows, action_rows);
        self.top = bands.top;
        self.tabs = bands.tabs;
        self.actions = bands.actions;
        self.body = bands.body;
        self.log = bands.log;
        self.hints = bands.hints;
        let want = if self.list_width == 0 {
            default_list_width(bands.body.width)
        } else {
            self.list_width
        };
        self.list_width = clamp_list_width(want, bands.body.width);
        let split_x = bands.body.x.saturating_add(self.list_width);
        let rule_height = bands.body.height.saturating_sub(1);
        self.columns_split = Rect {
            x: split_x,
            y: bands.body.y,
            width: u16::from(bands.body.width > 0),
            height: rule_height,
        };
        self.rows_split = Rect {
            x: bands.body.x,
            y: bands.body.bottom().saturating_sub(1),
            width: bands.body.width,
            height: u16::from(bands.body.height > 0),
        };
    }

    pub fn nudge(&mut self, split: Split, delta: i16) {
        match split {
            Split::Columns => {
                let next =
                    i16::try_from(self.list_width).unwrap_or(i16::MAX) + delta;
                self.list_width = clamp_list_width(
                    u16::try_from(next.max(0)).unwrap_or(0),
                    self.body.width,
                );
            }
            Split::Rows => {
                let next =
                    i16::try_from(self.log_rows).unwrap_or(i16::MAX) + delta;
                self.log_rows = clamp_log_rows_for(
                    u16::try_from(next.max(0)).unwrap_or(0),
                    self.area.height,
                    self.action_rows,
                );
            }
        }
    }

    /// Move the vertical rule so it sits on `column`.
    pub fn drag_columns_to(&mut self, column: u16) {
        let width = column.saturating_sub(self.body.x);
        self.list_width = clamp_list_width(width, self.body.width);
    }

    /// Move the horizontal rule so it sits on `row`.
    pub fn drag_rows_to(&mut self, row: u16) {
        let log_rows = self.hints.y.saturating_sub(row.saturating_add(1));
        self.log_rows =
            clamp_log_rows_for(log_rows, self.area.height, self.action_rows);
    }
}

struct Bands {
    top: Rect,
    tabs: Rect,
    actions: Rect,
    body: Rect,
    log: Rect,
    hints: Rect,
}

fn fitted_action_rows(area_height: u16, want: u16) -> u16 {
    if want == 0 {
        return 0;
    }
    let top_h = 1.min(area_height);
    let tabs_h = 3.min(area_height.saturating_sub(top_h));
    let room = area_height.saturating_sub(top_h.saturating_add(tabs_h));
    if room < ACTION_ROWS {
        0
    } else {
        ACTION_ROWS.min(want)
    }
}

fn bands(area: Rect, log_rows: u16, action_rows: u16) -> Bands {
    let top_h = 1.min(area.height);
    let tabs_h = 3.min(area.height.saturating_sub(top_h));
    let actions_h = action_rows
        .min(area.height.saturating_sub(top_h.saturating_add(tabs_h)));
    let used = top_h.saturating_add(tabs_h).saturating_add(actions_h);
    let hints_h = 1.min(area.height.saturating_sub(used));
    let hints_y = area.y.saturating_add(area.height.saturating_sub(hints_h));
    let log_y = hints_y.saturating_sub(log_rows);
    let body_y = area.y.saturating_add(used);
    let body_h = log_y.saturating_sub(body_y);
    Bands {
        top: Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: top_h,
        },
        tabs: Rect {
            x: area.x,
            y: area.y.saturating_add(top_h),
            width: area.width,
            height: tabs_h,
        },
        actions: Rect {
            x: area.x,
            y: area.y.saturating_add(top_h.saturating_add(tabs_h)),
            width: area.width,
            height: actions_h,
        },
        body: Rect {
            x: area.x,
            y: body_y,
            width: area.width,
            height: body_h,
        },
        log: Rect {
            x: area.x,
            y: log_y,
            width: area.width,
            height: log_rows.min(hints_y.saturating_sub(log_y)),
        },
        hints: Rect {
            x: area.x,
            y: hints_y,
            width: area.width,
            height: hints_h,
        },
    }
}

pub fn default_list_width(body_width: u16) -> u16 {
    let want =
        u16::try_from(u32::from(body_width) * 42 / 100).unwrap_or(body_width);
    clamp_list_width(want, body_width)
}

pub fn clamp_list_width(want: u16, body_width: u16) -> u16 {
    if body_width < 3 {
        return body_width.saturating_sub(1) / 2;
    }
    let floor = if body_width >= PANE_MIN.saturating_mul(2).saturating_add(1) {
        PANE_MIN
    } else {
        body_width.saturating_sub(1) / 2
    };
    let max = body_width.saturating_sub(1).saturating_sub(floor);
    if max < floor {
        return floor.min(body_width.saturating_sub(1));
    }
    want.clamp(floor, max)
}

pub fn clamp_log_rows_for(
    want: u16, area_height: u16, action_rows: u16,
) -> u16 {
    let available =
        area_height.saturating_sub(CHROME.saturating_add(action_rows));
    if available == 0 {
        return 0;
    }
    let max_log = available.saturating_sub(MAIN_MIN);
    if max_log < LOG_MIN {
        return want.min(available).max(1).min(available);
    }
    want.clamp(LOG_MIN, max_log)
}

/// Outline to draw before any key or click.
/// An empty list keeps the open tab. A list with rows outlines the selected
/// row, or the first row when nothing is selected yet.
pub fn starting_focus(tab: Tab, rows: usize, selected: Option<usize>) -> Focus {
    if rows == 0 {
        return Focus::Tab(tab);
    }
    let index = selected.unwrap_or(0).min(rows - 1);
    Focus::Row(index)
}

/// Tabs, then the left-pane buttons, then the list, then the right-pane
/// buttons, then the server subtabs. Right buttons are omitted when
/// `selected` is false, because those controls are not drawn.
pub fn order(tab: Tab, rows: usize, selected: bool) -> Vec<Focus> {
    let mut items = TABS.into_iter().map(Focus::Tab).collect::<Vec<_>>();
    for action in Action::left(tab) {
        items.push(Focus::Action(*action));
    }
    for index in 0..rows {
        items.push(Focus::Row(index));
    }
    if selected {
        if tab == Tab::Servers {
            items.push(Focus::Detail(DetailControl::On));
            items.push(Focus::Detail(DetailControl::Off));
            items.push(Focus::Detail(DetailControl::Reboot));
        }
        for action in Action::right(tab) {
            items.push(Focus::Action(*action));
        }
    }
    if tab == Tab::Servers {
        for pane in ServerPane::ALL {
            items.push(Focus::ServerPane(pane));
        }
    }
    items
}

pub fn step(items: &[Focus], current: Focus, backward: bool) -> Focus {
    cycle(items, current, backward)
}

pub fn step_popup(current: PopupFocus, backward: bool) -> PopupFocus {
    cycle(&PopupFocus::ORDER, current, backward)
}

pub fn step_prompt(
    fields: usize, current: PromptFocus, backward: bool,
) -> PromptFocus {
    let mut items = Vec::with_capacity(fields.saturating_add(2));
    for index in 0..fields {
        items.push(PromptFocus::Field(index));
    }
    items.push(PromptFocus::Submit);
    items.push(PromptFocus::Cancel);
    cycle(&items, current, backward)
}

fn cycle<T>(items: &[T], current: T, backward: bool) -> T
where
    T: Copy + PartialEq,
{
    let Some(index) = items.iter().position(|item| *item == current) else {
        return items.first().copied().unwrap_or(current);
    };
    let next = if backward {
        if index == 0 {
            items.len().saturating_sub(1)
        } else {
            index - 1
        }
    } else {
        (index + 1) % items.len().max(1)
    };
    items.get(next).copied().unwrap_or(current)
}

pub fn target_at(
    screen: &Screen, column: u16, row: u16, modal: Modal,
) -> Option<Target> {
    let point = Position::new(column, row);
    screen.hits.iter().rev().find_map(|hit| {
        if !hit.area.contains(point) || !target_allowed(modal, hit.target) {
            return None;
        }
        Some(hit.target)
    })
}

fn target_allowed(modal: Modal, target: Target) -> bool {
    match modal {
        Modal::None => !matches!(target, Target::Power(_) | Target::Prompt(_)),
        Modal::Power => matches!(target, Target::Power(_)),
        Modal::Prompt => matches!(target, Target::Prompt(_)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starting_focus_is_the_open_tab_then_the_first_row() {
        assert_eq!(
            starting_focus(Tab::Servers, 0, None),
            Focus::Tab(Tab::Servers)
        );
        assert_eq!(starting_focus(Tab::Servers, 3, None), Focus::Row(0));
        assert_eq!(starting_focus(Tab::Servers, 3, Some(2)), Focus::Row(2));
        assert_eq!(
            starting_focus(Tab::SshKeys, 0, Some(0)),
            Focus::Tab(Tab::SshKeys)
        );
    }

    #[test]
    fn focus_order_is_tabs_then_left_actions_then_rows_then_right() {
        let order = order(Tab::Servers, 2, true);
        assert_eq!(
            &order[..6],
            &[
                Focus::Tab(Tab::Servers),
                Focus::Tab(Tab::SshKeys),
                Focus::Tab(Tab::Locations),
                Focus::Tab(Tab::Dns),
                Focus::Tab(Tab::Account),
                Focus::Action(Action::CreateServer),
            ]
        );
        assert_eq!(
            &order[6..],
            &[
                Focus::Row(0),
                Focus::Row(1),
                Focus::Detail(DetailControl::On),
                Focus::Detail(DetailControl::Off),
                Focus::Detail(DetailControl::Reboot),
                Focus::Action(Action::RebuildServer),
                Focus::Action(Action::DeleteServer),
                Focus::ServerPane(ServerPane::Jobs),
                Focus::ServerPane(ServerPane::Address),
                Focus::ServerPane(ServerPane::Bandwidth),
            ]
        );
        assert_eq!(
            step(&order, Focus::Action(Action::DeleteServer), false),
            Focus::ServerPane(ServerPane::Jobs)
        );
        assert_eq!(
            step(&order, Focus::ServerPane(ServerPane::Jobs), true),
            Focus::Action(Action::DeleteServer)
        );
        assert_eq!(
            step(&order, Focus::ServerPane(ServerPane::Bandwidth), false),
            order[0]
        );
        assert_eq!(
            step(&order, Focus::Tab(Tab::Account), false),
            Focus::Action(Action::CreateServer)
        );
        assert_eq!(
            step(&order, Focus::Action(Action::CreateServer), false),
            Focus::Row(0)
        );

        let mut focus = order[0];
        for _ in 0..order.len() {
            focus = step(&order, focus, false);
        }
        assert_eq!(focus, order[0]);
        assert_eq!(step(&order, order[0], true), *order.last().unwrap());

        let hidden = super::order(Tab::Servers, 0, false);
        assert!(hidden.contains(&Focus::Action(Action::CreateServer)));
        assert!(hidden.iter().all(|item| {
            !matches!(
                item,
                Focus::Detail(_)
                    | Focus::Action(Action::RebuildServer)
                    | Focus::Action(Action::DeleteServer)
            )
        }));
        assert_eq!(
            *hidden.last().unwrap(),
            Focus::ServerPane(ServerPane::Bandwidth)
        );

        let ssh = super::order(Tab::SshKeys, 1, true);
        assert_eq!(ssh[5], Focus::Action(Action::NewKey));
        assert_eq!(ssh[6], Focus::Row(0));
        assert_eq!(ssh[7], Focus::Action(Action::UpdateKey));
        assert_eq!(*ssh.last().unwrap(), Focus::Action(Action::DeleteKey));
        let ssh_empty = super::order(Tab::SshKeys, 0, false);
        assert_eq!(
            ssh_empty.as_slice(),
            &[
                Focus::Tab(Tab::Servers),
                Focus::Tab(Tab::SshKeys),
                Focus::Tab(Tab::Locations),
                Focus::Tab(Tab::Dns),
                Focus::Tab(Tab::Account),
                Focus::Action(Action::NewKey),
            ]
        );
        let locations = super::order(Tab::Locations, 3, true);
        assert_eq!(locations.len(), 8);
        assert!(locations.iter().all(|item| {
            !matches!(item, Focus::Detail(_) | Focus::Action(_))
        }));
        let account = super::order(Tab::Account, 1, true);
        assert!(account.iter().all(|item| !matches!(item, Focus::Action(_))));
        let dns = super::order(Tab::Dns, 0, false);
        assert_eq!(dns.len(), 7);
        assert_eq!(dns[5], Focus::Action(Action::CreateZone));
        assert_eq!(*dns.last().unwrap(), Focus::Action(Action::CreateRecord));
        let dns_open = super::order(Tab::Dns, 1, true);
        assert_eq!(dns_open[5], Focus::Action(Action::CreateZone));
        assert_eq!(dns_open[6], Focus::Action(Action::CreateRecord));
        assert_eq!(dns_open[7], Focus::Row(0));
        assert_eq!(dns_open[8], Focus::Action(Action::DeleteZone));
        assert_eq!(
            *dns_open.last().unwrap(),
            Focus::Action(Action::DeleteRecord)
        );
    }

    #[test]
    fn popup_focus_cycles_force_confirm_cancel() {
        assert_eq!(step_popup(PopupFocus::Confirm, false), PopupFocus::Cancel);
        assert_eq!(step_popup(PopupFocus::Cancel, false), PopupFocus::Force);
        assert_eq!(step_popup(PopupFocus::Force, false), PopupFocus::Confirm);
        assert_eq!(step_popup(PopupFocus::Confirm, true), PopupFocus::Force);
        assert_eq!(step_popup(PopupFocus::Force, true), PopupFocus::Cancel);
        assert_eq!(
            step_prompt(1, PromptFocus::Cancel, false),
            PromptFocus::Field(0)
        );
        assert_eq!(
            step_prompt(1, PromptFocus::Field(0), true),
            PromptFocus::Cancel
        );
        assert_eq!(
            step_prompt(0, PromptFocus::Submit, false),
            PromptFocus::Cancel
        );
        assert_eq!(
            step_prompt(2, PromptFocus::Field(0), false),
            PromptFocus::Field(1)
        );
        assert_eq!(
            step_prompt(2, PromptFocus::Field(1), false),
            PromptFocus::Submit
        );
    }

    #[test]
    fn a_split_stays_out_of_the_tab_order_and_cannot_collapse_a_pane() {
        let items = order(Tab::Servers, 2, true);
        assert!(items.iter().all(|item| !matches!(item, Focus::Split(_))));
        assert_eq!(clamp_list_width(1, 100), PANE_MIN);
        assert_eq!(clamp_list_width(99, 100), 100 - 1 - PANE_MIN);
        assert_eq!(default_list_width(100), 42);
        assert_eq!(clamp_log_rows_for(1, 32, 0), LOG_MIN);
        assert_eq!(clamp_log_rows_for(4, 32, 0), LOG_DEFAULT);
        // 1 + 3 + 1 chrome, and 6 rows stay with the main panes.
        assert_eq!(clamp_log_rows_for(100, 32, 0), 32 - 5 - 6);

        let mut screen = Screen::default();
        screen.layout(
            Rect {
                x: 0,
                y: 0,
                width: 100,
                height: 32,
            },
            0,
        );
        assert_eq!(screen.list_width, 42);
        assert_eq!(screen.columns_split.width, 1);
        assert!(screen.columns_split.height >= 1);
        assert_eq!(screen.rows_split.height, 1);
        assert_eq!(screen.log.height, LOG_DEFAULT);
        assert_eq!(screen.log.y + screen.log.height, screen.hints.y);
        let columns = screen.columns_split.x;
        screen.drag_columns_to(columns.saturating_add(8));
        assert_eq!(screen.list_width, 50);
        screen.drag_columns_to(0);
        assert_eq!(screen.list_width, PANE_MIN);
        let row = screen.rows_split.y;
        screen.drag_rows_to(row.saturating_sub(2));
        assert_eq!(screen.log_rows, LOG_DEFAULT + 2);
        screen.drag_rows_to(0);
        assert_eq!(screen.log_rows, 32 - 5 - 6);

        let mut with_actions = Screen::default();
        with_actions.layout(
            Rect {
                x: 0,
                y: 0,
                width: 100,
                height: 32,
            },
            ACTION_ROWS,
        );
        assert_eq!(with_actions.actions.y, with_actions.tabs.bottom());
        assert_eq!(with_actions.actions.height, ACTION_ROWS);
        assert_eq!(with_actions.body.y, with_actions.actions.bottom());
        assert!(with_actions.body.height >= 6);
        assert_eq!(
            with_actions.log.y + with_actions.log.height,
            with_actions.hints.y
        );
        with_actions.log_rows = 100;
        with_actions.layout(
            Rect {
                x: 0,
                y: 0,
                width: 100,
                height: 32,
            },
            ACTION_ROWS,
        );
        // The actions row is one more row of chrome.
        assert_eq!(with_actions.log_rows, 32 - 6 - 6);
    }
}
