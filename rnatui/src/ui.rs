// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::focus::{
    DetailControl, Focus, KeyFocus, Modal, PopupFocus, Screen, ServerPane,
    Slot, TABS, Tab, Target,
};
use crate::model::{
    App, Field, Level, Pane, Section, ServerKind, ServerRow,
    power_confirm_text, short_fingerprint,
};

mod theme {
    use ratatui::style::Color;

    pub const BG: Color = Color::Rgb(12, 14, 18);
    pub const BAR: Color = Color::Rgb(20, 24, 31);
    pub const TEXT: Color = Color::Rgb(230, 237, 243);
    pub const MUTED: Color = Color::Rgb(139, 149, 163);
    pub const LINE: Color = Color::Rgb(44, 52, 66);
    pub const PANEL: Color = Color::Rgb(16, 20, 26);
    pub const CONTROL: Color = Color::Rgb(22, 27, 34);
    pub const CONTROL_EDGE: Color = Color::Rgb(92, 103, 115);
    pub const ACTIVE: Color = Color::Rgb(26, 49, 72);
    pub const ACTIVE_EDGE: Color = Color::Rgb(109, 142, 174);
    pub const ACCENT: Color = Color::Rgb(185, 215, 245);
    pub const FOCUS: Color = Color::Rgb(230, 180, 34);
    pub const OK: Color = Color::Rgb(143, 212, 164);
    pub const WARN: Color = Color::Rgb(224, 176, 122);
    pub const ERR: Color = Color::Rgb(224, 122, 122);
    pub const BRIGHT: Color = Color::Rgb(244, 248, 252);
}

pub fn render(frame: &mut Frame<'_>, app: &mut App, screen: &mut Screen) {
    screen.reset_hits();
    let area = frame.area();
    fill(frame, area, theme::BG);
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(LOG_ROWS),
        Constraint::Length(1),
    ])
    .split(area);
    render_top(frame, chunks[0], app);
    render_tabs(frame, chunks[1], app, screen);
    render_body(frame, chunks[2], app, screen);
    render_log(frame, chunks[3], app);
    render_hints(frame, chunks[4]);
    match app.modal() {
        Modal::Power => {
            dim(frame, area);
            render_power_popup(frame, area, app, screen);
        }
        Modal::Key => {
            dim(frame, area);
            render_key_popup(frame, area, app, screen);
        }
        Modal::None => {}
    }
}

fn render_top(frame: &mut Frame<'_>, area: Rect, app: &App) {
    fill(frame, area, theme::BAR);
    let brand = " rnatui";
    let host = clip(&app.host, area.width.saturating_sub(16) as usize);
    let right = format!("api  {host}");
    let gap = (area.width as usize)
        .saturating_sub(brand.chars().count() + right.chars().count());
    let line = Line::from(vec![
        Span::styled(
            brand,
            Style::default()
                .fg(theme::TEXT)
                .bg(theme::BAR)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ".repeat(gap), Style::default().bg(theme::BAR)),
        Span::styled("api  ", Style::default().fg(theme::MUTED).bg(theme::BAR)),
        Span::styled(host, Style::default().fg(theme::TEXT).bg(theme::BAR)),
    ]);
    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(theme::BAR)),
        area,
    );
}

fn render_tabs(
    frame: &mut Frame<'_>, area: Rect, app: &App, screen: &mut Screen,
) {
    fill(frame, area, theme::BAR);
    let labels = TABS.map(Tab::label);
    let row = layout_tabs(area, &labels, true);
    paint_tab_shelf(frame, area, theme::BAR);
    for (tab, rect) in TABS.into_iter().zip(row) {
        let open = app.tab == tab;
        let focused = page_focused(app) && app.focus == Focus::Tab(tab);
        paint_tab(frame, rect, area, tab.label(), open, focused);
        screen.hit(Target::Tab(tab), rect);
    }
}

fn render_body(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let columns = Layout::horizontal([
        Constraint::Percentage(42),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .split(area);
    fill(frame, columns[0], theme::PANEL);
    fill(frame, columns[1], theme::LINE);
    screen.list = columns[0];
    screen.detail = columns[2];
    render_list(frame, inset(columns[0], 1), app, screen);
    render_detail(frame, inset(columns[2], 1), app, screen);
}

fn render_list(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    if area.height == 0 || area.width == 0 {
        return;
    }
    let width = usize::from(area.width);
    paint_line(frame, row_rect(area, 0), header_line(app.tab, width));
    let body = Rect {
        x: area.x,
        y: area.y.saturating_add(1),
        width: area.width,
        height: area.height.saturating_sub(1),
    };
    let rows = list_rows(app, width);
    let window = usize::from(body.height);
    let max_scroll = rows.len().saturating_sub(window.max(1));
    if app.list_scroll > max_scroll {
        app.list_scroll = max_scroll;
    }
    app.list_window = u16::try_from(window.max(1)).unwrap_or(u16::MAX);
    screen.list_window = app.list_window;
    if rows.is_empty() {
        let message = if app.activity().is_some() {
            "Loading…"
        } else {
            "Nothing here"
        };
        paint_plain(
            frame,
            row_rect(body, 0),
            message,
            Style::default().fg(theme::MUTED).bg(theme::PANEL),
        );
        return;
    }
    for (offset, (index, line, bg)) in rows
        .into_iter()
        .skip(app.list_scroll)
        .take(window)
        .enumerate()
    {
        let rect = row_rect(body, offset as u16);
        frame.render_widget(
            Paragraph::new(line).style(Style::default().bg(bg)),
            rect,
        );
        screen.hit(Target::Row(index), rect);
    }
}

fn list_rows(app: &App, width: usize) -> Vec<(usize, Line<'static>, Color)> {
    let columns = list_columns(app.tab, width);
    match app.tab {
        Tab::Servers => app
            .servers
            .iter()
            .enumerate()
            .map(|(index, row)| {
                text_row(
                    app,
                    index,
                    &[
                        (row.mbpkgid.to_string(), columns[0].1),
                        (row.fqdn.clone(), columns[1].1),
                        (row.kind.label().to_string(), columns[2].1),
                        (row.state.clone(), columns[3].1),
                    ],
                )
            })
            .collect(),
        Tab::SshKeys => app
            .keys
            .iter()
            .enumerate()
            .map(|(index, row)| {
                text_row(
                    app,
                    index,
                    &[
                        (row.name.clone(), columns[0].1),
                        (row.id.to_string(), columns[1].1),
                    ],
                )
            })
            .collect(),
        Tab::Locations => app
            .locations
            .iter()
            .enumerate()
            .map(|(index, row)| {
                text_row(
                    app,
                    index,
                    &[
                        (row.id.to_string(), columns[0].1),
                        (row.name.clone(), columns[1].1),
                        (row.continent.clone(), columns[2].1),
                    ],
                )
            })
            .collect(),
        Tab::Dns => app
            .zones
            .iter()
            .enumerate()
            .map(|(index, row)| {
                text_row(
                    app,
                    index,
                    &[
                        (row.id.to_string(), columns[0].1),
                        (row.name.clone(), columns[1].1),
                        (row.zone_type.clone(), columns[2].1),
                    ],
                )
            })
            .collect(),
        Tab::Account => {
            vec![text_row(
                app,
                0,
                &[(app.account_label().to_string(), columns[0].1)],
            )]
        }
    }
}

fn list_columns(tab: Tab, width: usize) -> Vec<(&'static str, usize)> {
    match tab {
        Tab::Servers => {
            let id_w = 7;
            let kind_w = 10;
            let state_w = 10;
            let fqdn_w = width.saturating_sub(id_w + kind_w + state_w + 4);
            vec![
                ("mbpkgid", id_w),
                ("fqdn", fqdn_w),
                ("kind", kind_w),
                ("state", state_w),
            ]
        }
        Tab::SshKeys => {
            let id_w = 8;
            vec![("name", width.saturating_sub(id_w + 2)), ("id", id_w)]
        }
        Tab::Locations => {
            let id_w = 6;
            let continent_w = 9;
            let name_w = width.saturating_sub(id_w + continent_w + 4);
            vec![("id", id_w), ("name", name_w), ("continent", continent_w)]
        }
        Tab::Dns => {
            let id_w = 6;
            let type_w = 8;
            let name_w = width.saturating_sub(id_w + type_w + 4);
            vec![("id", id_w), ("name", name_w), ("type", type_w)]
        }
        Tab::Account => vec![("account", width.saturating_sub(2))],
    }
}

fn text_row(
    app: &App, index: usize, cells: &[(String, usize)],
) -> (usize, Line<'static>, Color) {
    let selected = app.selected_index() == Some(index);
    let focused = page_focused(app) && app.focus == Focus::Row(index);
    let bg = if selected {
        theme::ACTIVE
    } else {
        theme::CONTROL
    };
    let mut spans = Vec::new();
    let marker = if focused { "▌" } else { " " };
    let marker_fg = if focused { theme::FOCUS } else { theme::TEXT };
    spans.push(Span::styled(marker, Style::default().fg(marker_fg).bg(bg)));
    for (text, width) in cells {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            pad(text, *width),
            Style::default().fg(theme::TEXT).bg(bg),
        ));
    }
    (index, Line::from(spans), bg)
}

fn header_line(tab: Tab, width: usize) -> Line<'static> {
    let style = Style::default().fg(theme::MUTED).bg(theme::PANEL);
    // Same gutter as the row focus marker, then the same cell widths.
    let mut spans = vec![Span::styled(" ", style)];
    for (label, column) in list_columns(tab, width) {
        spans.push(Span::styled(" ", style));
        spans.push(Span::styled(pad(label, column), style));
    }
    Line::from(spans)
}

fn render_detail(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    match app.tab {
        Tab::Servers => render_server(frame, area, app, screen),
        Tab::SshKeys => render_ssh(frame, area, app, screen),
        Tab::Locations => render_record(frame, area, app, location_pane(app)),
        Tab::Dns => render_record(frame, area, app, zone_pane(app)),
        Tab::Account => render_record(frame, area, app, account_pane(app)),
    }
}

fn render_server(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    let mut y = area.y;
    let bottom = area.bottom();
    if let Some(server) = app.selected_server().cloned() {
        paint_title(
            frame,
            area,
            &mut y,
            &server.fqdn,
            &format!("server {}", server.mbpkgid),
        );
        let reserve = 4.min(bottom.saturating_sub(y));
        let budget = bottom.saturating_sub(y).saturating_sub(reserve);
        for line in server_field_lines(app, &server)
            .into_iter()
            .take(budget as usize)
        {
            paint_line(frame, row_at(area, y), line);
            y = y.saturating_add(1);
        }
    } else {
        paint_plain(
            frame,
            row_at(area, y),
            "Select a server",
            Style::default().fg(theme::MUTED),
        );
        y = y.saturating_add(1);
    }
    if bottom.saturating_sub(y) >= 4 {
        paint_plain(
            frame,
            row_at(area, y),
            "power",
            Style::default().fg(theme::MUTED),
        );
        y = y.saturating_add(1);
        paint_power_buttons(frame, area, y, app, screen);
        y = y.saturating_add(3);
    }
    render_server_tail(frame, remaining(area, y), app, screen);
}

fn render_server_tail(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    if area.height == 0 || area.width == 0 {
        return;
    }
    let mut y = area.y;
    let bottom = area.bottom();
    for text in [
        "On, Off, and Reboot open a confirm popup for this server.",
        "Force carries into the popup. Nothing is sent until Confirm.",
    ] {
        if y >= bottom {
            break;
        }
        paint_plain(
            frame,
            row_at(area, y),
            text,
            Style::default().fg(theme::MUTED),
        );
        y = y.saturating_add(1);
    }
    if y >= bottom {
        return;
    }
    let tab_height = 3.min(bottom.saturating_sub(y));
    paint_server_panes(frame, area, y, tab_height, app, screen);
    y = y.saturating_add(tab_height);
    render_server_lines(frame, remaining(area, y), app);
}

fn paint_server_panes(
    frame: &mut Frame<'_>, area: Rect, y: u16, height: u16, app: &App,
    screen: &mut Screen,
) {
    if height == 0 {
        return;
    }
    let strip = Rect {
        x: area.x,
        y,
        width: area.width,
        height,
    };
    if height < 3 {
        let mut x = area.x;
        for pane in ServerPane::ALL {
            let label = pane.label();
            let width = tab_width(label);
            if x.saturating_add(width) > area.right() {
                break;
            }
            let rect = Rect {
                x,
                y,
                width,
                height,
            };
            let open = app.server_pane == pane;
            let focused =
                page_focused(app) && app.focus == Focus::ServerPane(pane);
            let (bg, fg, _) = tab_colors(open, focused);
            paint_plain(frame, rect, label, Style::default().fg(fg).bg(bg));
            screen.hit(Target::ServerPane(pane), rect);
            x = x.saturating_add(width).saturating_add(1);
        }
        return;
    }
    let labels = ServerPane::ALL.map(ServerPane::label);
    let row = layout_tabs(strip, &labels, false);
    paint_tab_shelf(frame, strip, theme::BG);
    for (pane, rect) in ServerPane::ALL.into_iter().zip(row) {
        let open = app.server_pane == pane;
        let focused = page_focused(app) && app.focus == Focus::ServerPane(pane);
        paint_tab(frame, rect, strip, pane.label(), open, focused);
        screen.hit(Target::ServerPane(pane), rect);
    }
}

fn render_server_lines(frame: &mut Frame<'_>, area: Rect, app: &mut App) {
    if area.height == 0 || area.width == 0 {
        return;
    }
    let width = usize::from(area.width);
    paint_plain(
        frame,
        row_rect(area, 0),
        pane_header(app.server_pane),
        Style::default().fg(theme::MUTED),
    );
    if area.height == 1 {
        app.server_line_window = 1;
        return;
    }
    let body = Rect {
        x: area.x,
        y: area.y.saturating_add(1),
        width: area.width,
        height: area.height.saturating_sub(1),
    };
    let ready = app.server_lines_ready();
    let lines = if ready {
        app.server_lines.clone()
    } else {
        Vec::new()
    };
    if !lines.is_empty() && app.server_line >= lines.len() {
        app.server_line = lines.len() - 1;
    }
    let window = usize::from(body.height);
    let max_scroll = lines.len().saturating_sub(window.max(1));
    if usize::from(app.detail_scroll) > max_scroll {
        app.detail_scroll = u16::try_from(max_scroll).unwrap_or(u16::MAX);
    }
    app.server_line_window = u16::try_from(window.max(1)).unwrap_or(u16::MAX);
    if lines.is_empty() {
        let message = if app.is_loading(Slot::ServerList) {
            "Loading…"
        } else {
            "none"
        };
        paint_plain(
            frame,
            row_rect(body, 0),
            message,
            Style::default().fg(theme::MUTED),
        );
        return;
    }
    let start = usize::from(app.detail_scroll);
    let focused =
        page_focused(app) && matches!(app.focus, Focus::ServerPane(_));
    for (offset, line) in lines.iter().skip(start).take(window).enumerate() {
        let index = start + offset;
        let marked = focused && index == app.server_line;
        let (fg, bg) = if marked {
            (theme::TEXT, theme::ACTIVE)
        } else {
            (theme::TEXT, theme::BG)
        };
        paint_plain(
            frame,
            row_rect(body, offset as u16),
            &pad(line, width),
            Style::default().fg(fg).bg(bg),
        );
    }
}

fn pane_header(pane: ServerPane) -> &'static str {
    match pane {
        ServerPane::Jobs => "id  inserted  status  command",
        ServerPane::Address => "kind  ip  netmask  gateway  reverse",
        ServerPane::Bandwidth => "date  rx  tx",
    }
}

fn server_field_lines(app: &App, server: &ServerRow) -> Vec<Line<'static>> {
    let mut lines = vec![
        field_line("fqdn", &server.fqdn, theme::TEXT),
        field_line("mbpkgid", &server.mbpkgid.to_string(), theme::TEXT),
        kind_line(server.kind),
        field_line("state", &server.state, state_color(&server.state)),
        field_line("status", &server.status, state_color(&server.status)),
        field_line("ip", &server.ip, theme::TEXT),
        field_line("location", &server.location, theme::TEXT),
    ];
    if app.server_detail_for == Some(u64::from(server.mbpkgid)) {
        for extra in &app.server_extras {
            lines.push(field_line(&extra.label, &extra.value, theme::TEXT));
        }
    }
    lines
}

fn kind_line(kind: ServerKind) -> Line<'static> {
    Line::from(vec![
        Span::styled(pad("kind", 12), Style::default().fg(theme::MUTED)),
        Span::styled(kind.label(), Style::default().fg(theme::TEXT)),
        Span::styled(
            format!("  {}", kind.command()),
            Style::default().fg(theme::MUTED),
        ),
    ])
}

fn render_ssh(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    let mut y = area.y;
    let bottom = area.bottom();
    if let Some(key) = app.selected_key().cloned() {
        paint_title(
            frame,
            area,
            &mut y,
            &key.name,
            &format!("ssh key {}", key.id),
        );
        for line in [
            field_line("name", &key.name, theme::TEXT),
            field_line("id", &key.id.to_string(), theme::TEXT),
            field_line(
                "fingerprint",
                &short_fingerprint(&key.fingerprint),
                theme::TEXT,
            ),
        ] {
            if y >= bottom {
                break;
            }
            paint_line(frame, row_at(area, y), line);
            y = y.saturating_add(1);
        }
    } else {
        paint_title(frame, area, &mut y, "SSH keys", "new ssh");
    }
    if bottom.saturating_sub(y) >= 3 {
        let rect = Rect {
            x: area.x,
            y,
            width: 12.min(area.width),
            height: 3,
        };
        paint_button(
            frame,
            screen,
            rect,
            DetailControl::NewKey.label(),
            detail_focused(app, DetailControl::NewKey),
            Target::Detail(DetailControl::NewKey),
        );
        y = y.saturating_add(3);
    }
    paint_scrolled(
        frame,
        remaining(area, y),
        app,
        vec![
            muted_line(
                "New key reads a public key file. A bare name is read from ~/.ssh.",
            ),
            muted_line(
                "No file generates ~/.ssh/id_ed25519 and refuses to overwrite it.",
            ),
        ],
    );
}

fn render_record(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, pane: Option<Pane>,
) {
    let mut y = area.y;
    let Some(pane) = pane else {
        paint_plain(
            frame,
            row_at(area, y),
            "Select a row",
            Style::default().fg(theme::MUTED),
        );
        return;
    };
    paint_title(frame, area, &mut y, &pane.title, &pane.subtitle);
    let mut lines: Vec<Line<'static>> = pane
        .fields
        .iter()
        .map(|field| field_line(&field.label, &field.value, theme::TEXT))
        .collect();
    extend_sections(&mut lines, &pane.sections);
    paint_scrolled(frame, remaining(area, y), app, lines);
}

fn location_pane(app: &App) -> Option<Pane> {
    if let Some((id, pane)) = app.location_detail.as_ref()
        && app
            .selected_location()
            .is_some_and(|row| u64::from(row.id) == *id)
    {
        return Some(pane.clone());
    }
    let row = app.selected_location()?;
    Some(Pane {
        title: row.name.clone(),
        subtitle: format!("location {}", row.id),
        fields: vec![
            Field::new("id", row.id.to_string()),
            Field::new("name", row.name.clone()),
            Field::new("iata", row.iata.clone()),
            Field::new("continent", row.continent.clone()),
        ],
        sections: Vec::new(),
    })
}

fn zone_pane(app: &App) -> Option<Pane> {
    if let Some((id, pane)) = app.zone_detail.as_ref()
        && app
            .selected_zone()
            .is_some_and(|row| u64::from(row.id) == *id)
    {
        return Some(pane.clone());
    }
    let row = app.selected_zone()?;
    Some(Pane {
        title: row.name.clone(),
        subtitle: format!("zone {}", row.id),
        fields: vec![
            Field::new("id", row.id.to_string()),
            Field::new("name", row.name.clone()),
            Field::new("type", row.zone_type.clone()),
        ],
        sections: Vec::new(),
    })
}

fn account_pane(app: &App) -> Option<Pane> {
    if let Some(pane) = app.account.clone() {
        return Some(pane);
    }
    Some(Pane {
        title: "Account".into(),
        subtitle: "account".into(),
        fields: Vec::new(),
        sections: Vec::new(),
    })
}

fn paint_power_buttons(
    frame: &mut Frame<'_>, area: Rect, y: u16, app: &App, screen: &mut Screen,
) {
    let controls = [
        (DetailControl::On, 6u16),
        (DetailControl::Off, 7),
        (DetailControl::Reboot, 10),
    ];
    let mut x = area.x;
    for (control, width) in controls {
        if x.saturating_add(width) > area.right() {
            break;
        }
        let rect = Rect {
            x,
            y,
            width,
            height: 3,
        };
        paint_button(
            frame,
            screen,
            rect,
            control.label(),
            detail_focused(app, control),
            Target::Detail(control),
        );
        x = x.saturating_add(width).saturating_add(1);
    }
    let check = Rect {
        x,
        y,
        width: 14.min(area.right().saturating_sub(x)),
        height: 3,
    };
    if check.width >= 8 {
        paint_check(
            frame,
            screen,
            check,
            DetailControl::Force.label(),
            app.force,
            detail_focused(app, DetailControl::Force),
            Target::Detail(DetailControl::Force),
        );
    }
}

fn render_power_popup(
    frame: &mut Frame<'_>, area: Rect, app: &App, screen: &mut Screen,
) {
    let Some(popup) = app.power.as_ref() else {
        return;
    };
    let rect = centered(area, 56, 16);
    frame.render_widget(Clear, rect);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACTIVE_EDGE))
        .style(Style::default().bg(theme::CONTROL).fg(theme::TEXT));
    let inner = block.inner(rect);
    frame.render_widget(block, rect);
    if inner.width == 0 || inner.height == 0 {
        return;
    }
    let title = Line::from(vec![
        Span::styled(
            popup.verb.title(),
            Style::default().fg(theme::BRIGHT).bg(theme::ACTIVE),
        ),
        Span::styled(
            pad_left(
                "confirm",
                inner.width.saturating_sub(popup.verb.title().len() as u16)
                    as usize,
            ),
            Style::default().fg(theme::MUTED).bg(theme::ACTIVE),
        ),
    ]);
    paint_line(frame, row_at(inner, inner.y), title);
    let body = power_confirm_text(
        popup.kind,
        popup.verb,
        popup.mbpkgid,
        &popup.fqdn,
        popup.force,
    );
    for (offset, line) in body.lines().skip(1).enumerate() {
        paint_plain(
            frame,
            row_at(inner, inner.y.saturating_add(2 + offset as u16)),
            line,
            Style::default().fg(theme::TEXT).bg(theme::CONTROL),
        );
    }
    let check_y = inner.y.saturating_add(7);
    let check = Rect {
        x: inner.x,
        y: check_y,
        width: 16.min(inner.width),
        height: 3,
    };
    paint_check(
        frame,
        screen,
        check,
        "Force",
        popup.force,
        popup.focus == PopupFocus::Force,
        Target::Power(PopupFocus::Force),
    );
    let hint_x = check.right().saturating_add(1);
    if hint_x < inner.right() {
        paint_plain(
            frame,
            Rect {
                x: hint_x,
                y: check_y.saturating_add(1),
                width: inner.right().saturating_sub(hint_x),
                height: 1,
            },
            "adds --force",
            Style::default().fg(theme::MUTED).bg(theme::CONTROL),
        );
    }
    let button_y = inner.bottom().saturating_sub(3);
    paint_popup_button(
        frame,
        screen,
        inner,
        button_y,
        PopupButton {
            label: "Confirm",
            width: 12,
            primary: true,
            focused: popup.focus == PopupFocus::Confirm,
            target: Target::Power(PopupFocus::Confirm),
        },
    );
    paint_popup_button(
        frame,
        screen,
        inner,
        button_y,
        PopupButton {
            label: "Cancel",
            width: 10,
            primary: false,
            focused: popup.focus == PopupFocus::Cancel,
            target: Target::Power(PopupFocus::Cancel),
        },
    );
}

fn render_key_popup(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    let Some(popup) = app.key.clone() else {
        return;
    };
    let rect = centered(area, 60, 14);
    frame.render_widget(Clear, rect);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACTIVE_EDGE))
        .style(Style::default().bg(theme::CONTROL).fg(theme::TEXT));
    let inner = block.inner(rect);
    frame.render_widget(block, rect);
    if inner.height < 8 || inner.width < 8 {
        return;
    }
    paint_line(
        frame,
        row_at(inner, inner.y),
        Line::from(Span::styled(
            "New SSH key",
            Style::default().fg(theme::BRIGHT).bg(theme::ACTIVE),
        )),
    );
    paint_plain(
        frame,
        row_at(inner, inner.y.saturating_add(2)),
        "Public key file, or empty to generate ~/.ssh/id_ed25519",
        Style::default().fg(theme::MUTED).bg(theme::CONTROL),
    );
    let input = Rect {
        x: inner.x,
        y: inner.y.saturating_add(4),
        width: inner.width,
        height: 3,
    };
    let focused = popup.focus == KeyFocus::Input;
    let border = if focused {
        theme::FOCUS
    } else {
        theme::CONTROL_EDGE
    };
    paint_box(frame, input, "", theme::BG, theme::TEXT, border);
    let text_area = Rect {
        x: input.x.saturating_add(1),
        y: input.y.saturating_add(1),
        width: input.width.saturating_sub(2),
        height: 1,
    };
    let shown = tail(&popup.text, text_area.width as usize);
    paint_plain(
        frame,
        text_area,
        &shown,
        Style::default().fg(theme::TEXT).bg(theme::BG),
    );
    if focused {
        let cursor_x = text_area.x.saturating_add(shown.chars().count() as u16);
        if cursor_x < text_area.right() {
            frame.set_cursor_position(Position::new(cursor_x, text_area.y));
        }
    }
    screen.hit(Target::Key(KeyFocus::Input), input);
    let button_y = inner.bottom().saturating_sub(3);
    paint_popup_button(
        frame,
        screen,
        inner,
        button_y,
        PopupButton {
            label: "Create",
            width: 10,
            primary: true,
            focused: popup.focus == KeyFocus::Create,
            target: Target::Key(KeyFocus::Create),
        },
    );
    paint_popup_button(
        frame,
        screen,
        inner,
        button_y,
        PopupButton {
            label: "Cancel",
            width: 10,
            primary: false,
            focused: popup.focus == KeyFocus::Cancel,
            target: Target::Key(KeyFocus::Cancel),
        },
    );
}

struct PopupButton<'a> {
    label: &'a str,
    width: u16,
    primary: bool,
    focused: bool,
    target: Target,
}

fn paint_popup_button(
    frame: &mut Frame<'_>, screen: &mut Screen, inner: Rect, y: u16,
    button: PopupButton<'_>,
) {
    let right = if button.primary {
        inner.right().saturating_sub(button.width + 12)
    } else {
        inner.right().saturating_sub(button.width)
    };
    let rect = Rect {
        x: right,
        y,
        width: button.width,
        height: 3,
    };
    if rect.x < inner.x
        || rect.y.saturating_add(3) > inner.bottom().saturating_add(1)
    {
        return;
    }
    paint_button(
        frame,
        screen,
        rect,
        button.label,
        button.focused,
        button.target,
    );
}

const LOG_ROWS: u16 = 4;

fn render_log(frame: &mut Frame<'_>, area: Rect, app: &App) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let width = usize::from(area.width.saturating_sub(2));
    let mut lines = Vec::new();
    for entry in app.status_lines() {
        let color = match entry.level {
            Level::Error => theme::ERR,
            Level::Ok => theme::OK,
        };
        lines.push((clip(&entry.text, width), color));
    }
    if let Some(activity) = app.activity() {
        lines.push((clip(activity, width), theme::MUTED));
    }
    let height = usize::from(area.height);
    let start = lines.len().saturating_sub(height);
    let visible = lines.len() - start;
    let pad = height - visible;
    for (offset, (text, color)) in lines.into_iter().skip(start).enumerate() {
        paint_plain(
            frame,
            row_rect(area, (pad + offset) as u16),
            &format!(" {text}"),
            Style::default().fg(color).bg(theme::BG),
        );
    }
}

fn render_hints(frame: &mut Frame<'_>, area: Rect) {
    fill(frame, area, theme::BAR);
    let mut spans = vec![Span::styled(" ", Style::default().bg(theme::BAR))];
    spans.extend(hint("Tab", "move"));
    spans.extend(hint("Shift-Tab", "back"));
    spans.extend(hint("Enter or Space", "activate"));
    spans.extend(hint("click", "activates"));
    let line = Line::from(spans);
    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(theme::BAR)),
        area,
    );
}

fn hint(key: &str, word: &str) -> Vec<Span<'static>> {
    vec![
        Span::styled(
            key.to_string(),
            Style::default().fg(theme::TEXT).bg(theme::BAR),
        ),
        Span::styled(
            format!(" {word}  "),
            Style::default().fg(theme::MUTED).bg(theme::BAR),
        ),
    ]
}

fn paint_title(
    frame: &mut Frame<'_>, area: Rect, y: &mut u16, title: &str, subtitle: &str,
) {
    let title = clip(title, area.width as usize);
    let room = (area.width as usize).saturating_sub(title.chars().count() + 2);
    let subtitle = clip(subtitle, room);
    let gap = (area.width as usize)
        .saturating_sub(title.chars().count() + subtitle.chars().count() + 1);
    let line = Line::from(vec![
        Span::styled(
            title,
            Style::default()
                .fg(theme::TEXT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ".repeat(gap.max(1))),
        Span::styled(subtitle, Style::default().fg(theme::MUTED)),
    ]);
    paint_line(frame, row_at(area, *y), line);
    *y = y.saturating_add(2);
}

fn paint_button(
    frame: &mut Frame<'_>, screen: &mut Screen, area: Rect, label: &str,
    focused: bool, target: Target,
) {
    let border = if focused {
        theme::FOCUS
    } else {
        theme::CONTROL_EDGE
    };
    paint_box(frame, area, label, theme::CONTROL, theme::TEXT, border);
    screen.hit(target, area);
}

fn paint_check(
    frame: &mut Frame<'_>, screen: &mut Screen, area: Rect, label: &str,
    checked: bool, focused: bool, target: Target,
) {
    let border = if focused {
        theme::FOCUS
    } else {
        theme::CONTROL_EDGE
    };
    let mark = if checked { "[x]" } else { "[ ]" };
    paint_box(
        frame,
        area,
        &format!("{mark} {label}"),
        theme::CONTROL,
        theme::TEXT,
        border,
    );
    screen.hit(target, area);
}

fn tab_width(label: &str) -> u16 {
    (label.chars().count() as u16).saturating_add(4)
}

fn tab_colors(open: bool, focused: bool) -> (Color, Color, Color) {
    if focused {
        (
            if open { theme::ACTIVE } else { theme::CONTROL },
            theme::BRIGHT,
            theme::FOCUS,
        )
    } else if open {
        (theme::ACTIVE, theme::BRIGHT, theme::ACCENT)
    } else {
        (theme::CONTROL, theme::TEXT, theme::CONTROL_EDGE)
    }
}

/// Place tab rects on one row. `center` is the main tab row; detail tabs stay
/// on the left. A tab that does not fit is dropped, and so are the ones after it.
fn layout_tabs(area: Rect, labels: &[&str], center: bool) -> Vec<Rect> {
    if area.height < 3 || area.width == 0 {
        return Vec::new();
    }
    let widths: Vec<u16> = labels.iter().copied().map(tab_width).collect();
    let gaps = widths.len().saturating_sub(1) as u16;
    let total = widths
        .iter()
        .copied()
        .fold(0u16, u16::saturating_add)
        .saturating_add(gaps);
    let mut x = if center && total < area.width {
        area.x.saturating_add((area.width - total) / 2)
    } else if center {
        area.x.saturating_add(1)
    } else {
        area.x
    };
    let mut placed = Vec::with_capacity(widths.len());
    for width in widths {
        if x.saturating_add(width) > area.right() {
            break;
        }
        placed.push(Rect {
            x,
            y: area.y,
            width,
            height: 3.min(area.height),
        });
        x = x.saturating_add(width).saturating_add(1);
    }
    placed
}

fn paint_tab_shelf(frame: &mut Frame<'_>, area: Rect, canvas: Color) {
    if area.height < 3 || area.width == 0 {
        return;
    }
    let y = area.y.saturating_add(2);
    let style = Style::default().fg(theme::CONTROL_EDGE).bg(canvas);
    let horizontal = ratatui::symbols::line::NORMAL.horizontal;
    for x in area.x..area.right() {
        paint_cell(frame, x, y, horizontal, style);
    }
}

/// A tab on the shelf: outline and fill, open along the bottom when selected.
fn paint_tab(
    frame: &mut Frame<'_>, tab: Rect, row: Rect, label: &str, open: bool,
    focused: bool,
) {
    if tab.width < 2 || tab.height < 3 {
        return;
    }
    let (bg, fg, border) = tab_colors(open, focused);
    let line = ratatui::symbols::line::NORMAL;
    let edge = Style::default().fg(border).bg(bg);
    let text = Style::default().fg(fg).bg(bg);
    let left = tab.x;
    let right = tab.right().saturating_sub(1);
    let top = tab.y;
    let mid = top.saturating_add(1);
    let bottom = top.saturating_add(2);
    paint_cell(frame, left, top, line.top_left, edge);
    for x in left.saturating_add(1)..right {
        paint_cell(frame, x, top, line.horizontal, edge);
    }
    paint_cell(frame, right, top, line.top_right, edge);
    paint_cell(frame, left, mid, line.vertical, edge);
    let inner = usize::from(tab.width.saturating_sub(2));
    for (offset, ch) in centered_label(label, inner).chars().enumerate() {
        let x = left
            .saturating_add(1)
            .saturating_add(u16::try_from(offset).unwrap_or(u16::MAX));
        if x >= right {
            break;
        }
        paint_char(frame, x, mid, ch, text);
    }
    paint_cell(frame, right, mid, line.vertical, edge);
    let shelf_left = left > row.x;
    let shelf_right = right.saturating_add(1) < row.right();
    let (south_west, south_east, foot) = if open {
        (
            if shelf_left { line.bottom_right } else { "╵" },
            if shelf_right { line.bottom_left } else { "╵" },
            " ",
        )
    } else {
        (
            if shelf_left {
                line.horizontal_up
            } else {
                line.bottom_left
            },
            if shelf_right {
                line.horizontal_up
            } else {
                line.bottom_right
            },
            line.horizontal,
        )
    };
    paint_cell(frame, left, bottom, south_west, edge);
    for x in left.saturating_add(1)..right {
        paint_cell(frame, x, bottom, foot, edge);
    }
    paint_cell(frame, right, bottom, south_east, edge);
}

fn centered_label(label: &str, inner: usize) -> String {
    let count = label.chars().count();
    if count >= inner {
        return label.chars().take(inner).collect();
    }
    let pad = inner - count;
    let left = pad / 2;
    format!("{}{label}{}", " ".repeat(left), " ".repeat(pad - left))
}

fn paint_cell(
    frame: &mut Frame<'_>, x: u16, y: u16, symbol: &str, style: Style,
) {
    if let Some(cell) = frame.buffer_mut().cell_mut((x, y)) {
        cell.set_symbol(symbol);
        cell.set_style(style);
    }
}

fn paint_char(frame: &mut Frame<'_>, x: u16, y: u16, ch: char, style: Style) {
    let mut bytes = [0u8; 4];
    paint_cell(frame, x, y, ch.encode_utf8(&mut bytes), style);
}

fn paint_box(
    frame: &mut Frame<'_>, area: Rect, label: &str, bg: Color, fg: Color,
    border: Color,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border))
        .style(Style::default().bg(bg).fg(fg));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    if inner.width > 0 && inner.height > 0 && !label.is_empty() {
        frame.render_widget(
            Paragraph::new(label)
                .alignment(Alignment::Center)
                .style(Style::default().fg(fg).bg(bg)),
            inner,
        );
    }
}

fn paint_scrolled(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, lines: Vec<Line<'static>>,
) {
    if area.height == 0 {
        return;
    }
    let max = u16::try_from(lines.len().saturating_sub(area.height as usize))
        .unwrap_or(u16::MAX);
    app.clamp_detail_scroll(max);
    let start = app.detail_scroll as usize;
    for (offset, line) in lines
        .into_iter()
        .skip(start)
        .take(area.height as usize)
        .enumerate()
    {
        paint_line(frame, row_rect(area, offset as u16), line);
    }
}

fn extend_sections(lines: &mut Vec<Line<'static>>, sections: &[Section]) {
    for section in sections {
        lines.push(Line::from(""));
        lines.push(muted_line(&section.title));
        if section.lines.is_empty() {
            lines.push(muted_line("none"));
            continue;
        }
        for line in &section.lines {
            lines.push(Line::from(line.clone()));
        }
    }
}

fn field_line(label: &str, value: &str, color: Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(pad(label, 12), Style::default().fg(theme::MUTED)),
        Span::styled(value.to_string(), Style::default().fg(color)),
    ])
}

fn muted_line(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default().fg(theme::MUTED),
    ))
}

fn paint_line(frame: &mut Frame<'_>, area: Rect, line: Line<'_>) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    frame.render_widget(Paragraph::new(line), area);
}

fn paint_plain(frame: &mut Frame<'_>, area: Rect, text: &str, style: Style) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    frame.render_widget(Paragraph::new(text).style(style), area);
}

fn fill(frame: &mut Frame<'_>, area: Rect, color: Color) {
    frame.render_widget(
        Block::default().style(Style::default().bg(color)),
        area,
    );
}

fn dim(frame: &mut Frame<'_>, area: Rect) {
    let buffer = frame.buffer_mut();
    for y in area.y..area.bottom() {
        for x in area.x..area.right() {
            if let Some(cell) = buffer.cell_mut((x, y)) {
                cell.set_style(
                    Style::default().fg(theme::MUTED).bg(Color::Rgb(6, 7, 10)),
                );
            }
        }
    }
}

fn page_focused(app: &App) -> bool {
    app.modal() == Modal::None
}

fn detail_focused(app: &App, control: DetailControl) -> bool {
    page_focused(app) && app.focus == Focus::Detail(control)
}

fn state_color(state: &str) -> Color {
    let value = state.to_ascii_lowercase();
    if value.contains("run") || value.contains("active") || value == "ok" {
        theme::OK
    } else if value.contains("stop")
        || value.contains("off")
        || value.contains("shut")
    {
        theme::WARN
    } else {
        theme::TEXT
    }
}

fn remaining(area: Rect, y: u16) -> Rect {
    Rect {
        x: area.x,
        y,
        width: area.width,
        height: area.bottom().saturating_sub(y),
    }
}

fn row_at(area: Rect, y: u16) -> Rect {
    Rect {
        x: area.x,
        y,
        width: area.width,
        height: 1,
    }
}

fn row_rect(area: Rect, offset: u16) -> Rect {
    Rect {
        x: area.x,
        y: area.y.saturating_add(offset),
        width: area.width,
        height: 1,
    }
}

fn inset(area: Rect, by: u16) -> Rect {
    Rect {
        x: area.x.saturating_add(by),
        y: area.y,
        width: area.width.saturating_sub(by.saturating_mul(2)),
        height: area.height,
    }
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    }
}

fn pad(text: &str, width: usize) -> String {
    let clipped = clip(text, width);
    let extra = width.saturating_sub(clipped.chars().count());
    let mut out = clipped;
    out.extend(std::iter::repeat_n(' ', extra));
    out
}

fn pad_left(text: &str, width: usize) -> String {
    let clipped = clip(text, width);
    let extra = width.saturating_sub(clipped.chars().count());
    format!("{}{clipped}", " ".repeat(extra))
}

fn clip(text: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let count = text.chars().count();
    if count <= width {
        return text.to_string();
    }
    if width == 1 {
        return "…".to_string();
    }
    let mut out: String = text.chars().take(width - 1).collect();
    out.push('…');
    out
}

fn tail(text: &str, width: usize) -> String {
    let count = text.chars().count();
    if count <= width {
        return text.to_string();
    }
    text.chars().skip(count - width).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{PowerVerb, ServerKind, ServerRow};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn column(line: &str, needle: &str) -> usize {
        let byte = line.find(needle).unwrap_or_else(|| {
            panic!("{needle} missing from {line}");
        });
        line[..byte].chars().count()
    }

    fn view(app: &mut App) -> String {
        let backend = TestBackend::new(100, 32);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut screen = Screen::default();
        terminal
            .draw(|frame| render(frame, app, &mut screen))
            .unwrap();
        let buffer = terminal.backend().buffer().clone();
        let mut lines = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                if let Some(cell) = buffer.cell((x, y)) {
                    lines.push_str(cell.symbol());
                }
            }
            lines.push('\n');
        }
        lines
    }

    fn sample() -> App {
        let mut app = App::new("vapi2.netactuate.com");
        app.set_servers(
            vec![ServerRow::listed(
                1001,
                "web-01.example",
                "running",
                ServerKind::Virtual,
            )],
            1001,
        );
        app
    }

    #[test]
    fn the_servers_screen_shows_the_host_and_power_controls() {
        let text = view(&mut sample());
        assert!(text.contains("rnatui"), "{text}");
        assert!(text.contains("vapi2.netactuate.com"), "{text}");
        assert!(text.contains("Servers"), "{text}");
        assert!(text.contains("web-01.example"), "{text}");
        assert!(text.contains("virtual"), "{text}");
        assert!(text.contains("On"), "{text}");
        assert!(text.contains("Reboot"), "{text}");
        assert!(text.contains("Force"), "{text}");
        assert!(text.contains("Tab"), "{text}");
        assert!(!text.contains("secret"));
        assert!(!text.contains("get server"), "{text}");
    }

    #[test]
    fn server_subtabs_sit_under_the_power_notes() {
        let mut app = sample();
        app.server_lines = vec!["7  2024-01-02 03:04:05  2  reboot".into()];
        app.server_lines_for = Some(1001);
        app.server_lines_pane = ServerPane::Jobs;
        let text = view(&mut app);
        let lines: Vec<&str> = text.lines().collect();
        let notes = lines
            .iter()
            .position(|line| line.contains("Nothing is sent"))
            .expect("power notes");
        let tabs = lines
            .iter()
            .position(|line| {
                line.contains("Jobs")
                    && line.contains("Address")
                    && line.contains("Bandwidth")
            })
            .expect("subtabs");
        let job = lines
            .iter()
            .position(|line| line.contains("reboot"))
            .expect("job row");
        let reboot = lines
            .iter()
            .position(|line| line.contains("Reboot"))
            .expect("reboot");
        assert!(reboot < notes, "{text}");
        assert!(notes < tabs, "{text}");
        assert!(tabs < job, "{text}");
        assert!(lines[job].contains("2024-01-02"), "{text}");
        assert!(text.contains("id  inserted  status  command"), "{text}");
    }

    #[test]
    fn the_list_header_starts_in_the_same_columns_as_the_rows() {
        let mut app = sample();
        let text = view(&mut app);
        let lines: Vec<&str> = text.lines().collect();
        let header = lines
            .iter()
            .copied()
            .find(|line| line.contains("mbpkgid") && line.contains("fqdn"))
            .expect("header");
        let row = lines
            .iter()
            .copied()
            .find(|line| {
                line.contains("1001")
                    && line.contains("virtual")
                    && line.contains("running")
            })
            .expect("row");
        assert_eq!(
            column(header, "mbpkgid"),
            column(row, "1001"),
            "{header}\n{row}"
        );
        assert_eq!(
            column(header, "fqdn"),
            column(row, "web-01"),
            "{header}\n{row}"
        );
        assert_eq!(
            column(header, "kind"),
            column(row, "virtual"),
            "{header}\n{row}"
        );
        assert_eq!(
            column(header, "state"),
            column(row, "running"),
            "{header}\n{row}"
        );

        for tab in TABS {
            app.tab = tab;
            let text = view(&mut app);
            for label in ["get server", "get ssh", "get location", "get dns"] {
                assert!(!text.contains(label), "{text}");
            }
        }
    }

    #[test]
    fn the_power_popup_names_the_call_for_each_kind() {
        let mut app = sample();
        app.open_power(PowerVerb::Reboot);
        let text = view(&mut app);
        assert!(text.contains("power-virt reboot -m 1001"), "{text}");
        assert!(text.contains("Confirm"), "{text}");
        assert!(text.contains("web-01.example"), "{text}");

        let mut app = sample();
        app.servers[0].kind = ServerKind::Dedicated;
        app.servers[0].fqdn = "metal-01.example".into();
        app.open_power(PowerVerb::Off);
        let text = view(&mut app);
        assert!(text.contains("power-ded off -m 1001"), "{text}");
        assert!(text.contains("dedicated"), "{text}");
        assert!(text.contains("metal-01.example"), "{text}");
    }

    #[test]
    fn the_log_strip_sits_above_the_key_bar() {
        let mut app = sample();
        for index in 0..6 {
            app.banner_error(format!("status {index}"));
        }
        let text = view(&mut app);
        let lines: Vec<&str> = text.lines().collect();
        let key = lines
            .iter()
            .rposition(|line| line.contains("Shift-Tab"))
            .expect("key bar");
        assert_eq!(key, lines.len() - 1, "{text}");
        assert!(lines[key].contains("Tab"), "{text}");
        assert!(lines[key].contains("click"), "{text}");
        assert!(!lines[key].contains("status"), "{text}");
        assert!(lines[key - 1].contains("status 5"), "{text}");
        assert!(lines[key - 2].contains("status 4"), "{text}");
        assert!(lines[key - 3].contains("status 3"), "{text}");
        assert!(lines[key - 4].contains("status 2"), "{text}");
        assert!(!text.contains("status 0"), "{text}");
        assert!(!text.contains("status 1"), "{text}");
    }
}
