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
}

/// Subtabs under the server power buttons. Tab moves the outline.
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
    Force,
    NewKey,
}

impl DetailControl {
    pub fn label(self) -> &'static str {
        match self {
            DetailControl::On => "On",
            DetailControl::Off => "Off",
            DetailControl::Reboot => "Reboot",
            DetailControl::Force => "Force",
            DetailControl::NewKey => "New key",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Focus {
    Tab(Tab),
    Row(usize),
    Detail(DetailControl),
    ServerPane(ServerPane),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyFocus {
    Input,
    Create,
    Cancel,
}

impl KeyFocus {
    pub const ORDER: [KeyFocus; 3] =
        [KeyFocus::Input, KeyFocus::Create, KeyFocus::Cancel];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modal {
    None,
    Power,
    Key,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Tab(Tab),
    Row(usize),
    Detail(DetailControl),
    ServerPane(ServerPane),
    Power(PopupFocus),
    Key(KeyFocus),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hit {
    pub target: Target,
    pub area: Rect,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Screen {
    pub hits: Vec<Hit>,
    pub list: Rect,
    pub detail: Rect,
    pub list_window: u16,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            hits: Vec::new(),
            list: Rect::default(),
            detail: Rect::default(),
            list_window: 1,
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
}

pub fn order(tab: Tab, rows: usize) -> Vec<Focus> {
    let mut items = TABS.into_iter().map(Focus::Tab).collect::<Vec<_>>();
    for index in 0..rows {
        items.push(Focus::Row(index));
    }
    match tab {
        Tab::Servers => {
            items.push(Focus::Detail(DetailControl::On));
            items.push(Focus::Detail(DetailControl::Off));
            items.push(Focus::Detail(DetailControl::Reboot));
            items.push(Focus::Detail(DetailControl::Force));
            for pane in ServerPane::ALL {
                items.push(Focus::ServerPane(pane));
            }
        }
        Tab::SshKeys => items.push(Focus::Detail(DetailControl::NewKey)),
        Tab::Locations | Tab::Dns | Tab::Account => {}
    }
    items
}

pub fn step(items: &[Focus], current: Focus, backward: bool) -> Focus {
    cycle(items, current, backward)
}

pub fn step_popup(current: PopupFocus, backward: bool) -> PopupFocus {
    cycle(&PopupFocus::ORDER, current, backward)
}

pub fn step_key(current: KeyFocus, backward: bool) -> KeyFocus {
    cycle(&KeyFocus::ORDER, current, backward)
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
        Modal::None => !matches!(target, Target::Power(_) | Target::Key(_)),
        Modal::Power => matches!(target, Target::Power(_)),
        Modal::Key => matches!(target, Target::Key(_)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_order_is_tabs_then_rows_then_detail_and_tab_wraps() {
        let order = order(Tab::Servers, 2);
        assert_eq!(
            &order[..7],
            &[
                Focus::Tab(Tab::Servers),
                Focus::Tab(Tab::SshKeys),
                Focus::Tab(Tab::Locations),
                Focus::Tab(Tab::Dns),
                Focus::Tab(Tab::Account),
                Focus::Row(0),
                Focus::Row(1),
            ]
        );
        assert_eq!(
            &order[7..],
            &[
                Focus::Detail(DetailControl::On),
                Focus::Detail(DetailControl::Off),
                Focus::Detail(DetailControl::Reboot),
                Focus::Detail(DetailControl::Force),
                Focus::ServerPane(ServerPane::Jobs),
                Focus::ServerPane(ServerPane::Address),
                Focus::ServerPane(ServerPane::Bandwidth),
            ]
        );
        assert_eq!(
            step(&order, Focus::Detail(DetailControl::Force), false,),
            Focus::ServerPane(ServerPane::Jobs)
        );
        assert_eq!(
            step(&order, Focus::ServerPane(ServerPane::Jobs), true),
            Focus::Detail(DetailControl::Force)
        );
        assert_eq!(
            step(&order, Focus::ServerPane(ServerPane::Bandwidth), false),
            order[0]
        );

        let mut focus = order[0];
        for _ in 0..order.len() {
            focus = step(&order, focus, false);
        }
        assert_eq!(focus, order[0]);
        assert_eq!(step(&order, order[0], true), *order.last().unwrap());

        let ssh = super::order(Tab::SshKeys, 1);
        assert_eq!(*ssh.last().unwrap(), Focus::Detail(DetailControl::NewKey));
        let locations = super::order(Tab::Locations, 3);
        assert_eq!(locations.len(), 8);
        assert!(
            locations
                .iter()
                .all(|item| !matches!(item, Focus::Detail(_)))
        );
    }

    #[test]
    fn popup_focus_cycles_force_confirm_cancel() {
        assert_eq!(step_popup(PopupFocus::Confirm, false), PopupFocus::Cancel);
        assert_eq!(step_popup(PopupFocus::Cancel, false), PopupFocus::Force);
        assert_eq!(step_popup(PopupFocus::Force, false), PopupFocus::Confirm);
        assert_eq!(step_popup(PopupFocus::Confirm, true), PopupFocus::Force);
        assert_eq!(step_popup(PopupFocus::Force, true), PopupFocus::Cancel);
        assert_eq!(step_key(KeyFocus::Cancel, false), KeyFocus::Input);
        assert_eq!(step_key(KeyFocus::Input, true), KeyFocus::Cancel);
    }
}
