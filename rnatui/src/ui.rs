// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0

use ratatui::Frame;
use ratatui::layout::{Alignment, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::focus::{
    Action, DetailControl, Focus, Modal, PopupFocus, PromptFocus, Screen,
    ServerPane, Slot, Split, TABS, Tab, Target,
};
use crate::model::{
    App, Field, Level, Pane, Section, ServerKind, ServerRow,
    power_confirm_text, short_fingerprint,
};

/// Rainfrog's frame and palette: plain box lines, green on the focused pane,
/// light gray on the others, and named ANSI colors rather than a custom fill.
/// Unfocused text stays bright enough to read. It is not dimmed, and it is
/// not the green bold style used for focus.
mod theme {
    use ratatui::style::{Color, Modifier, Style};

    pub const TEXT: Color = Color::Reset;
    pub const GREEN: Color = Color::Green;
    pub const YELLOW: Color = Color::Yellow;
    pub const BLUE: Color = Color::Blue;
    pub const RED: Color = Color::Red;
    /// ANSI white. A visible light gray, not bright-black.
    pub const GRAY: Color = Color::Gray;
    /// Bright white for words that must be read while a pane is unfocused.
    pub const WHITE: Color = Color::White;
    pub const LIGHT_BLUE: Color = Color::LightBlue;

    pub fn focus_edge() -> Style {
        Style::default().fg(GREEN)
    }

    /// Unfocused rules and inactive tab outlines. Light gray, with no dim.
    pub fn quiet_edge() -> Style {
        Style::default().fg(GRAY)
    }

    pub fn rule(focused: bool) -> Style {
        if focused {
            focus_edge().add_modifier(Modifier::BOLD)
        } else {
            quiet_edge()
        }
    }

    /// Labels and other secondary words. Lighter than the old dark gray.
    pub fn label() -> Style {
        Style::default().fg(GRAY)
    }

    /// Body text that is not the focused green row.
    pub fn reading() -> Style {
        Style::default().fg(WHITE)
    }
}

pub fn render(frame: &mut Frame<'_>, app: &mut App, screen: &mut Screen) {
    screen.reset_hits();
    screen.layout(frame.area(), 0);
    render_top(frame, screen.top, app);
    render_tabs(frame, screen.tabs, app, screen);
    render_body(frame, screen.body, app, screen);
    render_log(frame, screen.log, app);
    render_hints(frame, screen.hints);
    let area = screen.area;
    match app.modal() {
        Modal::Power => {
            dim(frame, area);
            render_power_popup(frame, area, app, screen);
        }
        Modal::Prompt => {
            dim(frame, area);
            render_prompt(frame, area, app, screen);
        }
        Modal::None => {}
    }
}

fn render_top(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let brand = " rnatui";
    let host = clip(&app.host, area.width.saturating_sub(16) as usize);
    let right = format!("api  {host}");
    let gap = (area.width as usize)
        .saturating_sub(brand.chars().count() + right.chars().count());
    let line = Line::from(vec![
        Span::styled(
            brand,
            Style::default()
                .fg(theme::GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ".repeat(gap)),
        Span::styled("api  ", theme::label()),
        Span::styled(host, Style::default().fg(theme::TEXT)),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

fn render_tabs(
    frame: &mut Frame<'_>, area: Rect, app: &App, screen: &mut Screen,
) {
    let labels = TABS.map(Tab::label);
    let row = layout_tabs(area, &labels, true);
    paint_tab_shelf(frame, area);
    for (tab, rect) in TABS.into_iter().zip(row) {
        let open = app.tab == tab;
        let focused = page_focused(app)
            && (app.focus == Focus::Tab(tab) || app.startup_tab_focused(tab));
        paint_tab(frame, rect, area, tab.label(), open, focused);
        screen.hit(Target::Tab(tab), rect);
    }
}

struct Command {
    label: &'static str,
    focused: bool,
    target: Target,
}

fn left_commands(app: &App) -> Vec<Command> {
    Action::left(app.tab)
        .iter()
        .copied()
        .map(|action| command(app, action.label(), Target::Action(action)))
        .collect()
}

fn right_commands(app: &App) -> Vec<Command> {
    if app.selected_index().is_none() {
        return Vec::new();
    }
    let mut commands = Vec::new();
    if app.tab == Tab::Servers {
        for control in
            [DetailControl::On, DetailControl::Off, DetailControl::Reboot]
        {
            commands.push(command(
                app,
                control.label(),
                Target::Detail(control),
            ));
        }
    }
    for action in Action::right(app.tab) {
        commands.push(command(app, action.label(), Target::Action(*action)));
    }
    commands
}

fn command(app: &App, label: &'static str, target: Target) -> Command {
    let focused = page_focused(app)
        && match target {
            Target::Action(action) => app.focus == Focus::Action(action),
            Target::Detail(control) => app.focus == Focus::Detail(control),
            _ => false,
        };
    Command {
        label,
        focused,
        target,
    }
}

/// Buttons inside a pane. One blank row separates them from the frame above
/// and from the list or detail below. Returns the rows those pieces use.
fn paint_command_row(
    frame: &mut Frame<'_>, area: Rect, screen: &mut Screen,
    commands: &[Command],
) -> u16 {
    const GAP: u16 = 1;
    if commands.is_empty() || area.height < GAP + 1 || area.width == 0 {
        return 0;
    }
    let mut x = area.x;
    let mut y = area.y.saturating_add(GAP);
    let floor = area.bottom().saturating_sub(GAP);
    if y >= floor {
        return 0;
    }
    let mut rows = 1u16;
    for command in commands {
        let width = button_width(command.label);
        if x != area.x && x.saturating_add(width) > area.right() {
            let next = y.saturating_add(1);
            if next >= floor {
                break;
            }
            y = next;
            x = area.x;
            rows = rows.saturating_add(1);
        }
        if x.saturating_add(width) > area.right() {
            break;
        }
        let rect = Rect {
            x,
            y,
            width,
            height: 1,
        };
        paint_command(frame, screen, rect, command);
        // Two empty columns so a click on the edge of one button misses the next.
        x = x.saturating_add(width).saturating_add(2);
    }
    rows.saturating_add(GAP).saturating_add(GAP)
}

/// A one-line control. Green and bold when it is the outline.
fn paint_command(
    frame: &mut Frame<'_>, screen: &mut Screen, area: Rect, command: &Command,
) {
    let focused = command.focused;
    let label = command.label;
    let text = if focused {
        theme::focus_edge().add_modifier(Modifier::BOLD)
    } else {
        theme::reading()
    };
    let edge = if focused {
        theme::focus_edge().add_modifier(Modifier::BOLD)
    } else {
        theme::quiet_edge()
    };
    let inner = usize::from(area.width.saturating_sub(2));
    let word = clip(label, inner);
    paint_char(frame, area.x, area.y, '[', edge);
    let mut x = area.x.saturating_add(1);
    for ch in word.chars() {
        if x.saturating_add(1) >= area.right() {
            break;
        }
        paint_char(frame, x, area.y, ch, text);
        x = x.saturating_add(1);
    }
    while x.saturating_add(1) < area.right() {
        paint_char(frame, x, area.y, ' ', text);
        x = x.saturating_add(1);
    }
    if area.width > 1 {
        paint_char(frame, area.right().saturating_sub(1), area.y, ']', edge);
    }
    screen.hit(command.target, area);
}

fn button_width(label: &str) -> u16 {
    (label.chars().count() as u16).saturating_add(4).max(6)
}

fn render_body(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    if area.width < 3 || area.height < 2 {
        return;
    }
    let list_width = screen.list_width;
    let split_x = area.x.saturating_add(list_width);
    let list_inner = Rect {
        x: area.x.saturating_add(1),
        y: area.y.saturating_add(1),
        width: list_width.saturating_sub(1),
        height: area.height.saturating_sub(2),
    };
    // One column of air on each side so the text does not sit on the rule.
    // The list already keeps a gutter for its focus marker.
    let detail_inner = Rect {
        x: split_x.saturating_add(2),
        y: area.y.saturating_add(1),
        width: area.width.saturating_sub(list_width).saturating_sub(4),
        height: area.height.saturating_sub(2),
    };
    // Scroll covers the pane interior, including the detail's side gutter.
    // The rules themselves are the drag targets.
    screen.list = list_inner;
    screen.detail = Rect {
        x: split_x.saturating_add(1),
        y: detail_inner.y,
        width: area.width.saturating_sub(list_width).saturating_sub(2),
        height: detail_inner.height,
    };
    paint_split_frame(frame, area, list_width, app);
    render_list(frame, list_inner, app, screen);
    render_detail(frame, detail_inner, app, screen);
}

/// One plain box around the list and the detail, with a vertical rule between
/// them. The bottom rule is the log split. Green marks the focused pane, the
/// same way rainfrog greens the focused block. The other edges and titles
/// stay a readable light gray or white.
fn paint_split_frame(
    frame: &mut Frame<'_>, area: Rect, list_width: u16, app: &App,
) {
    if area.width < 3 || area.height < 2 || list_width == 0 {
        return;
    }
    let line = ratatui::symbols::line::ROUNDED;
    let page = page_focused(app);
    // A row outlines the list. An empty list does too, while its open tab
    // is the outline, so the pane is green before the first row arrives.
    let list_on = page
        && (matches!(app.focus, Focus::Row(_))
            || matches!(app.focus, Focus::Action(action) if action.is_left(app.tab))
            || (app.row_count() == 0 && app.focus == Focus::Tab(app.tab)));
    let detail_on = page
        && (matches!(app.focus, Focus::Detail(_) | Focus::ServerPane(_))
            || matches!(
                app.focus,
                Focus::Action(action) if action.is_right(app.tab)
            ));
    let columns_on = page && matches!(app.focus, Focus::Split(Split::Columns));
    let rows_on = page && matches!(app.focus, Focus::Split(Split::Rows));
    let list_edge = if list_on {
        theme::focus_edge()
    } else {
        theme::quiet_edge()
    };
    let detail_edge = if detail_on {
        theme::focus_edge()
    } else {
        theme::quiet_edge()
    };
    let column_edge = if columns_on {
        theme::rule(true)
    } else if list_on || detail_on {
        theme::focus_edge()
    } else {
        theme::quiet_edge()
    };
    let row_edge = theme::rule(rows_on);
    let left = area.x;
    let right = area.right().saturating_sub(1);
    let top = area.y;
    let bottom = area.bottom().saturating_sub(1);
    let split = left
        .saturating_add(list_width)
        .clamp(left.saturating_add(1), right.saturating_sub(1));
    let list_title = if list_on {
        theme::focus_edge().add_modifier(Modifier::BOLD)
    } else {
        theme::reading()
    };
    let detail_title = if detail_on {
        theme::focus_edge().add_modifier(Modifier::BOLD)
    } else {
        theme::reading()
    };

    paint_cell(frame, left, top, line.top_left, list_edge);
    paint_cell(frame, split, top, line.horizontal_down, column_edge);
    paint_cell(frame, right, top, line.top_right, detail_edge);
    let foot = |side: Style| if rows_on { row_edge } else { side };
    paint_cell(frame, left, bottom, line.bottom_left, foot(list_edge));
    paint_cell(frame, split, bottom, line.horizontal_up, foot(column_edge));
    paint_cell(frame, right, bottom, line.bottom_right, foot(detail_edge));
    for x in left.saturating_add(1)..split {
        paint_cell(frame, x, top, line.horizontal, list_edge);
        paint_cell(frame, x, bottom, line.horizontal, foot(list_edge));
    }
    for x in split.saturating_add(1)..right {
        paint_cell(frame, x, top, line.horizontal, detail_edge);
        paint_cell(frame, x, bottom, line.horizontal, foot(detail_edge));
    }
    for y in top.saturating_add(1)..bottom {
        paint_cell(frame, left, y, line.vertical, list_edge);
        paint_cell(frame, split, y, line.vertical, column_edge);
        paint_cell(frame, right, y, line.vertical, detail_edge);
    }
    paint_rule_label(
        frame,
        left,
        split,
        top,
        &pane_label(app.tab.label()),
        list_title,
    );
    paint_rule_label(
        frame,
        split,
        right,
        top,
        &pane_label(detail_label(app.tab)),
        detail_title,
    );
}

fn pane_label(text: &str) -> String {
    format!(" {text} ")
}

fn detail_label(tab: Tab) -> &'static str {
    match tab {
        Tab::Servers => "server",
        Tab::SshKeys => "ssh key",
        Tab::Locations => "location",
        Tab::Dns => "zone",
        Tab::Account => "account",
    }
}

fn paint_rule_label(
    frame: &mut Frame<'_>, start: u16, end: u16, y: u16, label: &str,
    style: Style,
) {
    let room = usize::from(end.saturating_sub(start).saturating_sub(1));
    if room == 0 {
        return;
    }
    let text = clip(label, room);
    let origin = start.saturating_add(1);
    for (offset, ch) in text.chars().enumerate() {
        let x =
            origin.saturating_add(u16::try_from(offset).unwrap_or(u16::MAX));
        if x >= end {
            break;
        }
        paint_char(frame, x, y, ch, style);
    }
}

fn render_list(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    if area.height == 0 || area.width == 0 {
        return;
    }
    let commands = left_commands(app);
    let used = paint_command_row(frame, area, screen, &commands);
    let area = below(area, used);
    screen.list = area;
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
        let style = if app.activity().is_some() {
            Style::default().fg(theme::GREEN)
        } else {
            theme::reading()
        };
        paint_plain(frame, row_rect(body, 0), message, style);
        return;
    }
    for (offset, (index, line)) in rows
        .into_iter()
        .skip(app.list_scroll)
        .take(window)
        .enumerate()
    {
        let rect = row_rect(body, offset as u16);
        frame.render_widget(Paragraph::new(line), rect);
        screen.hit(Target::Row(index), rect);
    }
}

fn list_rows(app: &App, width: usize) -> Vec<(usize, Line<'static>)> {
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
) -> (usize, Line<'static>) {
    let selected = app.selected_index() == Some(index);
    let focused = page_focused(app) && app.focus == Focus::Row(index);
    // Green and bold on the focused row. A selected row whose pane is not
    // focused stays reversed, on white, so the letters stay readable.
    // Every other row is white, not dim and not green.
    let style = if focused {
        Style::default()
            .fg(theme::GREEN)
            .add_modifier(Modifier::BOLD)
    } else if selected {
        // Reversed white on black: the bar is light, and the letters stay
        // readable whether the terminal swaps the colors or not.
        Style::default()
            .fg(theme::WHITE)
            .bg(Color::Black)
            .add_modifier(Modifier::REVERSED)
    } else {
        theme::reading()
    };
    let mut spans = Vec::new();
    let marker = if focused { "▌" } else { " " };
    spans.push(Span::styled(marker, style));
    for (text, width) in cells {
        spans.push(Span::styled(" ", style));
        spans.push(Span::styled(pad(text, *width), style));
    }
    (index, Line::from(spans))
}

fn header_line(tab: Tab, width: usize) -> Line<'static> {
    let style = theme::label();
    // Same gutter as the row focus marker, then the same cell widths.
    let mut spans = vec![Span::styled(" ", style)];
    for (label, column) in list_columns(tab, width) {
        spans.push(Span::styled(" ", style));
        spans.push(Span::styled(pad(label, column), style));
    }
    Line::from(spans)
}

fn below(area: Rect, rows: u16) -> Rect {
    Rect {
        x: area.x,
        y: area.y.saturating_add(rows),
        width: area.width,
        height: area.height.saturating_sub(rows),
    }
}

fn render_detail(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let commands = right_commands(app);
    let used = paint_command_row(frame, area, screen, &commands);
    let area = below(area, used);
    screen.detail.y = screen.detail.y.saturating_add(used);
    screen.detail.height = screen.detail.height.saturating_sub(used);
    if area.width == 0 || area.height == 0 {
        return;
    }
    match app.tab {
        Tab::Servers => render_server(frame, area, app, screen),
        Tab::SshKeys => render_ssh(frame, area, app),
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
        paint_plain(frame, row_at(area, y), "Select a server", theme::label());
        y = y.saturating_add(1);
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
        paint_plain(frame, row_at(area, y), text, theme::label());
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
    paint_tab_shelf(frame, strip);
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
        theme::label(),
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
        paint_plain(frame, row_rect(body, 0), message, theme::label());
        return;
    }
    let start = usize::from(app.detail_scroll);
    let focused =
        page_focused(app) && matches!(app.focus, Focus::ServerPane(_));
    for (offset, line) in lines.iter().skip(start).take(window).enumerate() {
        let index = start + offset;
        let marked = focused && index == app.server_line;
        // Rainfrog's results cursor is a light-blue reversed row.
        let style = if marked {
            Style::default()
                .fg(theme::LIGHT_BLUE)
                .add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else {
            Style::default().fg(theme::TEXT)
        };
        paint_plain(
            frame,
            row_rect(body, offset as u16),
            &pad(line, width),
            style,
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
        Span::styled(pad("kind", 12), theme::label()),
        Span::styled(kind.label(), Style::default().fg(theme::TEXT)),
        Span::styled(format!("  {}", kind.command()), theme::label()),
    ])
}

fn render_ssh(frame: &mut Frame<'_>, area: Rect, app: &mut App) {
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
        paint_plain(frame, row_at(area, y), "Select a row", theme::label());
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
        .border_style(Style::default().fg(theme::YELLOW))
        .style(Style::default().fg(theme::TEXT));
    let inner = block.inner(rect);
    frame.render_widget(block, rect);
    if inner.width == 0 || inner.height == 0 {
        return;
    }
    let title = Line::from(vec![
        Span::styled(
            popup.verb.title(),
            Style::default()
                .fg(theme::TEXT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            pad_left(
                "confirm",
                inner.width.saturating_sub(popup.verb.title().len() as u16)
                    as usize,
            ),
            theme::label(),
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
            Style::default().fg(theme::TEXT),
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
            theme::label(),
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

fn render_prompt(
    frame: &mut Frame<'_>, area: Rect, app: &mut App, screen: &mut Screen,
) {
    let Some(popup) = app.prompt.clone() else {
        return;
    };
    let height = if popup.fields.is_empty() {
        12
    } else {
        (popup.fields.len() as u16 + 9).max(12)
    };
    let rect = centered(area, 68, height);
    frame.render_widget(Clear, rect);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::YELLOW))
        .style(Style::default().fg(theme::TEXT));
    let inner = block.inner(rect);
    frame.render_widget(block, rect);
    if inner.height < 6 || inner.width < 8 {
        return;
    }
    paint_line(
        frame,
        row_at(inner, inner.y),
        Line::from(Span::styled(
            clip(&popup.title, inner.width as usize),
            Style::default()
                .fg(theme::TEXT)
                .add_modifier(Modifier::BOLD),
        )),
    );
    paint_plain(
        frame,
        row_at(inner, inner.y.saturating_add(1)),
        &clip(&popup.hint, inner.width as usize),
        theme::label(),
    );
    let button_y = inner.bottom().saturating_sub(3);
    let mut y = inner.y.saturating_add(3);
    for (index, field) in popup.fields.iter().enumerate() {
        if y >= button_y {
            break;
        }
        let focused = popup.focus == PromptFocus::Field(index);
        let row = row_at(inner, y);
        let style = if focused {
            Style::default()
                .fg(theme::GREEN)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::TEXT)
        };
        let room = usize::from(row.width.saturating_sub(13));
        let shown = tail(&field.shown(), room);
        let line = format!("{} {}", pad(&field.label, 12), shown);
        paint_plain(frame, row, &clip(&line, row.width as usize), style);
        screen.hit(Target::Prompt(PromptFocus::Field(index)), row);
        if focused && field.is_text() {
            let cursor_x = row
                .x
                .saturating_add(13)
                .saturating_add(shown.chars().count() as u16);
            if cursor_x < row.right() {
                frame.set_cursor_position(Position::new(cursor_x, row.y));
            }
        }
        y = y.saturating_add(1);
    }
    let submit_width = button_width(&popup.submit_label).max(10);
    paint_popup_button(
        frame,
        screen,
        inner,
        button_y,
        PopupButton {
            label: &popup.submit_label,
            width: submit_width,
            primary: true,
            focused: popup.focus == PromptFocus::Submit,
            target: Target::Prompt(PromptFocus::Submit),
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
            focused: popup.focus == PromptFocus::Cancel,
            target: Target::Prompt(PromptFocus::Cancel),
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

fn render_log(frame: &mut Frame<'_>, area: Rect, app: &App) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let width = usize::from(area.width.saturating_sub(2));
    let mut lines = Vec::new();
    for entry in app.status_lines() {
        let color = match entry.level {
            Level::Error => theme::RED,
            Level::Ok => theme::GREEN,
        };
        for piece in wrap_status(&entry.text, width) {
            lines.push((piece, color));
        }
    }
    if let Some(activity) = app.activity() {
        lines.push((clip(activity, width), theme::GRAY));
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
            Style::default().fg(color),
        );
    }
}

fn render_hints(frame: &mut Frame<'_>, area: Rect) {
    let mut spans = Vec::new();
    spans.extend(hint("Tab", "move"));
    spans.extend(hint("Shift-Tab", "back"));
    spans.extend(hint("Enter or Space", "activate"));
    spans.extend(hint("click", "activates"));
    spans.extend(hint("-/=", "resize"));
    let line = Line::from(spans);
    frame.render_widget(
        Paragraph::new(line)
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme::BLUE)),
        area,
    );
}

fn hint(key: &str, word: &str) -> Vec<Span<'static>> {
    vec![
        Span::styled(key.to_string(), Style::default().fg(theme::BLUE)),
        Span::styled(format!(" {word}  "), theme::label()),
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
        Span::styled(subtitle, theme::label()),
    ]);
    paint_line(frame, row_at(area, *y), line);
    *y = y.saturating_add(2);
}

fn paint_button(
    frame: &mut Frame<'_>, screen: &mut Screen, area: Rect, label: &str,
    focused: bool, target: Target,
) {
    let border = if focused {
        theme::focus_edge()
    } else {
        theme::quiet_edge()
    };
    let text = if focused {
        Style::default()
            .fg(theme::TEXT)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::TEXT)
    };
    paint_box(frame, area, label, text, border);
    screen.hit(target, area);
}

fn paint_check(
    frame: &mut Frame<'_>, screen: &mut Screen, area: Rect, label: &str,
    checked: bool, focused: bool, target: Target,
) {
    let border = if focused {
        theme::focus_edge()
    } else {
        theme::quiet_edge()
    };
    let text = if focused {
        Style::default()
            .fg(theme::TEXT)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::TEXT)
    };
    let mark = if checked { "[x]" } else { "[ ]" };
    paint_box(frame, area, &format!("{mark} {label}"), text, border);
    screen.hit(target, area);
}

fn tab_width(label: &str) -> u16 {
    (label.chars().count() as u16).saturating_add(4)
}

fn tab_colors(open: bool, focused: bool) -> (Color, Color, Color) {
    if focused {
        (Color::Reset, theme::GREEN, theme::GREEN)
    } else if open {
        (Color::Reset, theme::WHITE, theme::WHITE)
    } else {
        // Inactive letters stay white. The outline is a visible light gray.
        (Color::Reset, theme::WHITE, theme::GRAY)
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

fn paint_tab_shelf(frame: &mut Frame<'_>, area: Rect) {
    if area.height < 3 || area.width == 0 {
        return;
    }
    let y = area.y.saturating_add(2);
    let style = theme::quiet_edge();
    let horizontal = ratatui::symbols::line::ROUNDED.horizontal;
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
    let line = ratatui::symbols::line::ROUNDED;
    let edge = Style::default().fg(border).bg(bg);
    let text = if focused {
        Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(fg).bg(bg)
    };
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
    frame: &mut Frame<'_>, area: Rect, label: &str, text: Style, border: Style,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border)
        .style(text);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    if inner.width > 0 && inner.height > 0 && !label.is_empty() {
        frame.render_widget(
            Paragraph::new(label)
                .alignment(Alignment::Center)
                .style(text),
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
        Span::styled(pad(label, 12), theme::label()),
        Span::styled(value.to_string(), Style::default().fg(color)),
    ])
}

fn muted_line(text: &str) -> Line<'static> {
    Line::from(Span::styled(text.to_string(), theme::label()))
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

fn dim(frame: &mut Frame<'_>, area: Rect) {
    let buffer = frame.buffer_mut();
    for y in area.y..area.bottom() {
        for x in area.x..area.right() {
            if let Some(cell) = buffer.cell_mut((x, y)) {
                cell.set_style(cell.style().add_modifier(Modifier::DIM));
            }
        }
    }
}

fn page_focused(app: &App) -> bool {
    app.modal() == Modal::None
}

fn state_color(state: &str) -> Color {
    let value = state.to_ascii_lowercase();
    if value.contains("run") || value.contains("active") || value == "ok" {
        theme::GREEN
    } else if value.contains("stop")
        || value.contains("off")
        || value.contains("shut")
    {
        theme::YELLOW
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

fn wrap_status(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return Vec::new();
    }
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return vec![String::new()];
    }
    chars.chunks(width).map(|chunk| chunk.iter().collect()).collect()
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
    use crate::focus::{Aim, target_at};
    use crate::model::{PowerVerb, ServerKind, ServerRow};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::style::Modifier;

    fn column(line: &str, needle: &str) -> usize {
        let byte = line.find(needle).unwrap_or_else(|| {
            panic!("{needle} missing from {line}");
        });
        line[..byte].chars().count()
    }

    fn draw(app: &mut App, screen: &mut Screen) -> ratatui::buffer::Buffer {
        let backend = TestBackend::new(100, 32);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| render(frame, app, screen)).unwrap();
        terminal.backend().buffer().clone()
    }

    fn view(app: &mut App) -> String {
        let mut screen = Screen::default();
        let buffer = draw(app, &mut screen);
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

    #[test]
    fn the_first_frame_is_already_focused() {
        let mut app = App::new("vapi2.netactuate.com");
        let mut screen = Screen::default();
        let buffer = draw(&mut app, &mut screen);
        let tab = tab_glyph(&buffer, &screen, Tab::Servers);
        assert_eq!(tab.0, Color::Green, "servers tab");
        assert!(tab.1.contains(Modifier::BOLD));
        assert_eq!(
            buffer.cell((0, screen.body.y)).unwrap().fg,
            Color::Green,
            "empty list pane"
        );
        let right = screen.body.right().saturating_sub(1);
        assert_eq!(
            buffer.cell((right, screen.body.y)).unwrap().fg,
            Color::Gray
        );

        app.set_servers(
            vec![
                ServerRow::listed(
                    1001,
                    "web-01.example",
                    "running",
                    ServerKind::Virtual,
                ),
                ServerRow::listed(
                    1002,
                    "db-01.example",
                    "stopped",
                    ServerKind::Virtual,
                ),
            ],
            1001,
        );
        let buffer = draw(&mut app, &mut screen);
        let tab = tab_glyph(&buffer, &screen, Tab::Servers);
        assert_eq!(tab.0, Color::Green, "open tab stays focused");
        assert!(tab.1.contains(Modifier::BOLD));
        assert_eq!(buffer.cell((0, screen.body.y)).unwrap().fg, Color::Green);
        let row = glyph(&buffer, screen.list, "web-01");
        assert_eq!(row.0, Color::Green, "first row");
        assert!(row.1.contains(Modifier::BOLD));
        let other = glyph(&buffer, screen.list, "db-01");
        assert_eq!(other.0, Color::White);
        assert!(!other.1.contains(Modifier::BOLD));
    }

    fn tab_glyph(
        buffer: &ratatui::buffer::Buffer, screen: &Screen, tab: Tab,
    ) -> (Color, Modifier) {
        let hit = screen
            .hits
            .iter()
            .find(|hit| hit.target == Target::Tab(tab))
            .expect("tab");
        glyph(buffer, hit.area, tab.label())
    }

    #[test]
    fn the_frame_uses_plain_lines_and_green_marks_the_focused_pane() {
        let mut app = sample();
        let mut screen = Screen::default();
        let buffer = draw(&mut app, &mut screen);
        let split_x = screen.columns_split.x;
        let top = screen.body.y;
        let bottom = screen.rows_split.y;
        assert_eq!(screen.columns_split.width, 1);
        assert!(screen.columns_split.height >= 1);
        assert_eq!(screen.rows_split.height, 1);
        assert_eq!(buffer.cell((0, top)).unwrap().symbol(), "╭");
        assert_eq!(buffer.cell((split_x, top)).unwrap().symbol(), "┬");
        assert_eq!(
            buffer
                .cell((split_x, top.saturating_add(2)))
                .unwrap()
                .symbol(),
            "│"
        );
        assert_eq!(buffer.cell((0, bottom)).unwrap().symbol(), "╰");
        assert_eq!(buffer.cell((split_x, bottom)).unwrap().symbol(), "┴");
        assert_eq!(buffer.cell((1, bottom)).unwrap().symbol(), "─");
        assert!(screen.list_width >= 20);
        let detail = screen.body.width - screen.list_width - 1;
        assert!(detail >= 20);
        assert!(screen.log.height >= 3);
        assert_eq!(screen.log.y + screen.log.height, screen.hints.y);

        app.focus = Focus::Row(0);
        let buffer = draw(&mut app, &mut screen);
        assert_eq!(buffer.cell((0, screen.body.y)).unwrap().fg, Color::Green);
        assert_eq!(
            buffer
                .cell((screen.columns_split.x, screen.body.y))
                .unwrap()
                .fg,
            Color::Green
        );
        let right = screen.body.right().saturating_sub(1);
        let quiet = buffer.cell((right, screen.body.y)).unwrap();
        assert_eq!(quiet.fg, Color::Gray);
        assert!(!quiet.modifier.contains(Modifier::DIM));

        screen.list_width = 1;
        screen.log_rows = 100;
        let _ = draw(&mut app, &mut screen);
        assert!(screen.list_width >= 20);
        assert!(screen.log_rows >= 3);
        assert!(screen.body.height >= 6);
    }

    #[test]
    fn unfocused_text_stays_readable_and_focus_stays_green() {
        let mut app = sample();
        app.servers.push(ServerRow::listed(
            1002,
            "db-01.example",
            "stopped",
            ServerKind::Virtual,
        ));
        app.focus = Focus::Row(0);
        let mut screen = Screen::default();
        let buffer = draw(&mut app, &mut screen);

        let ssh = screen
            .hits
            .iter()
            .find(|hit| hit.target == Target::Tab(Tab::SshKeys))
            .expect("ssh tab");
        let outline = buffer.cell((ssh.area.x, ssh.area.y)).unwrap();
        assert_eq!(outline.fg, Color::Gray, "inactive tab outline");
        assert!(!outline.modifier.contains(Modifier::DIM));
        let letter = glyph(&buffer, ssh.area, "SSH");
        assert_eq!(letter.0, Color::White, "inactive tab label");
        assert!(!letter.1.contains(Modifier::DIM));
        assert!(!letter.1.contains(Modifier::BOLD));

        let label = glyph(&buffer, screen.detail, "fqdn");
        assert_eq!(label.0, Color::Gray, "field label");
        assert!(!label.1.contains(Modifier::DIM));

        let other = glyph(&buffer, screen.list, "db-01");
        assert_eq!(other.0, Color::White, "unfocused row");
        assert!(!other.1.contains(Modifier::REVERSED));
        assert!(!other.1.contains(Modifier::DIM));
        assert!(!other.1.contains(Modifier::BOLD));

        let focused_row = glyph(&buffer, screen.list, "web-01");
        assert_eq!(focused_row.0, Color::Green);
        assert!(focused_row.1.contains(Modifier::BOLD));

        let title = glyph(&buffer, screen.body, "server");
        assert_ne!(title.0, Color::Green, "unfocused pane title");
        assert!(!title.1.contains(Modifier::DIM));

        app.focus = Focus::Detail(DetailControl::On);
        let buffer = draw(&mut app, &mut screen);
        let selected = glyph(&buffer, screen.list, "web-01");
        assert_eq!(selected.0, Color::White, "selected row foreground");
        assert!(selected.1.contains(Modifier::REVERSED));
        assert!(!selected.1.contains(Modifier::DIM));
        assert_ne!(selected.0, Color::Green);
    }

    #[test]
    fn pane_actions_sit_above_the_list_and_the_detail() {
        let mut app = sample();
        let mut screen = Screen::default();
        let buffer = draw(&mut app, &mut screen);
        assert_eq!(screen.actions.height, 0);
        assert_eq!(screen.body.y, screen.tabs.bottom());
        let create = hit(&screen, Target::Action(Action::CreateServer));
        assert!(screen.list.y > create.y, "create sits above the list rows");
        assert!(create.x < screen.columns_split.x);
        assert!(
            create.y >= screen.body.y.saturating_add(2),
            "a blank row sits under the frame"
        );
        let header = hit_line(&buffer, "mbpkgid");
        assert!(header >= create.y.saturating_add(2), "a blank row sits under the button");
        let on = hit(&screen, Target::Detail(DetailControl::On));
        let rebuild = hit(&screen, Target::Action(Action::RebuildServer));
        let delete = hit(&screen, Target::Action(Action::DeleteServer));
        let cancel = hit(&screen, Target::Action(Action::CancelServer));
        assert!(
            on.y >= screen.body.y.saturating_add(2),
            "detail buttons sit below the frame"
        );
        assert!(on.x > screen.columns_split.x);
        assert!(rebuild.x > on.x);
        if delete.y == rebuild.y {
            assert!(delete.x > rebuild.x);
        } else {
            assert!(delete.y > rebuild.y);
        }
        if cancel.y == delete.y {
            assert!(cancel.x > delete.x);
        } else {
            assert_eq!(cancel.y, delete.y.saturating_add(1));
        }
        assert_eq!(on.y, rebuild.y);
        assert!(screen.detail.y > on.y);
        assert!(
            screen
                .hits
                .iter()
                .all(|hit| hit.target != Target::Action(Action::NewKey))
        );

        app.focus = Focus::Action(Action::CreateServer);
        let buffer = draw(&mut app, &mut screen);
        let create = hit(&screen, Target::Action(Action::CreateServer));
        assert_eq!(buffer.cell((create.x, create.y)).unwrap().fg, Color::Green);
        let rebuild = hit(&screen, Target::Action(Action::RebuildServer));
        assert_eq!(
            buffer.cell((rebuild.x, rebuild.y)).unwrap().fg,
            Color::Gray
        );

        let mut empty = App::new("vapi2.netactuate.com");
        let mut screen = Screen::default();
        let _ = draw(&mut empty, &mut screen);
        assert!(
            hit_opt(&screen, Target::Action(Action::CreateServer)).is_some()
        );
        assert!(hit_opt(&screen, Target::Detail(DetailControl::On)).is_none());
        assert!(
            hit_opt(&screen, Target::Action(Action::RebuildServer)).is_none()
        );

        app.tab = Tab::SshKeys;
        app.focus = Focus::Tab(Tab::SshKeys);
        app.set_keys(Vec::new(), None);
        let mut screen = Screen::default();
        let _ = draw(&mut app, &mut screen);
        assert!(hit_opt(&screen, Target::Action(Action::NewKey)).is_some());
        assert!(hit_opt(&screen, Target::Action(Action::UpdateKey)).is_none());
        assert!(hit_opt(&screen, Target::Action(Action::DeleteKey)).is_none());
        app.set_keys(
            vec![crate::model::SshRow {
                id: 9,
                name: "laptop".into(),
                fingerprint: "aa".into(),
            }],
            Some(9),
        );
        let _ = draw(&mut app, &mut screen);
        let update = hit(&screen, Target::Action(Action::UpdateKey));
        assert!(update.x > screen.columns_split.x);
        assert!(hit_opt(&screen, Target::Action(Action::DeleteKey)).is_some());
        let new_key = hit(&screen, Target::Action(Action::NewKey));
        assert!(new_key.x < screen.columns_split.x);

        app.tab = Tab::Dns;
        app.set_zones(Vec::new(), None);
        let _ = draw(&mut app, &mut screen);
        assert!(hit_opt(&screen, Target::Action(Action::CreateZone)).is_some());
        assert!(
            hit_opt(&screen, Target::Action(Action::CreateRecord)).is_some()
        );
        assert!(hit_opt(&screen, Target::Action(Action::DeleteZone)).is_none());
        app.set_zones(
            vec![crate::model::ZoneRow {
                id: 4,
                name: "example.com".into(),
                zone_type: "NATIVE".into(),
            }],
            Some(4),
        );
        let _ = draw(&mut app, &mut screen);
        assert!(hit_opt(&screen, Target::Action(Action::DeleteZone)).is_some());
        assert!(
            hit_opt(&screen, Target::Action(Action::UpdateRecord)).is_some()
        );
        assert!(
            hit_opt(&screen, Target::Action(Action::DeleteRecord)).is_some()
        );
        let zone = hit(&screen, Target::Action(Action::CreateZone));
        let record = hit(&screen, Target::Action(Action::DeleteRecord));
        assert!(zone.x < screen.columns_split.x);
        assert!(record.x > screen.columns_split.x);

        for tab in [Tab::Locations, Tab::Account] {
            app.tab = tab;
            let _ = draw(&mut app, &mut screen);
            assert!(
                screen.hits.iter().all(|hit| !matches!(
                    hit.target,
                    Target::Action(_) | Target::Detail(_)
                )),
                "{tab:?}"
            );
        }
    }

    #[test]
    fn a_click_hits_the_button_under_the_cursor() {
        let mut app = sample();
        let mut screen = Screen::default();
        let _ = draw(&mut app, &mut screen);
        let on = hit(&screen, Target::Detail(DetailControl::On));
        let off = hit(&screen, Target::Detail(DetailControl::Off));
        let mid = on.x + on.width / 2;
        assert_eq!(
            target_at(&screen, mid, on.y, Modal::None),
            Some(Target::Detail(DetailControl::On))
        );
        if off.y == on.y {
            assert!(off.x >= on.right().saturating_add(2));
            assert_eq!(target_at(&screen, on.right(), on.y, Modal::None), None);
            assert_eq!(
                target_at(&screen, off.x + off.width / 2, off.y, Modal::None),
                Some(Target::Detail(DetailControl::Off))
            );
        }
    }

    #[test]
    fn vim_keys_activate_the_selection_they_move_to() {
        let mut app = sample();
        app.focus = Focus::Tab(Tab::Servers);
        let mut screen = Screen::default();
        let _ = draw(&mut app, &mut screen);
        app.move_aim(&screen, Aim::Right);
        assert_eq!(app.tab, Tab::SshKeys);
        assert_eq!(app.focus, Focus::Tab(Tab::SshKeys));
        app.move_aim(&screen, Aim::Left);
        assert_eq!(app.tab, Tab::Servers);
        app.move_aim(&screen, Aim::Left);
        assert_eq!(app.tab, Tab::Servers);

        app.move_aim(&screen, Aim::Down);
        assert!(app.prompt.is_some(), "j onto Create server opens it");

        let mut app = sample();
        app.set_servers(
            vec![
                ServerRow::listed(
                    1001,
                    "web-01.example",
                    "running",
                    ServerKind::Virtual,
                ),
                ServerRow::listed(
                    1002,
                    "web-02.example",
                    "stopped",
                    ServerKind::Virtual,
                ),
            ],
            1002,
        );
        app.focus = Focus::Action(Action::CreateServer);
        let mut screen = Screen::default();
        let _ = draw(&mut app, &mut screen);
        app.move_aim(&screen, Aim::Down);
        assert_eq!(app.focus, Focus::Row(0));
        assert_eq!(app.selected_index(), Some(0));

        let mut app = sample();
        app.focus = Focus::Action(Action::CreateServer);
        let mut screen = Screen::default();
        let _ = draw(&mut app, &mut screen);
        app.move_aim(&screen, Aim::Right);
        assert_eq!(
            app.power.as_ref().map(|popup| popup.verb),
            Some(PowerVerb::On)
        );

        let mut app = sample();
        app.focus = Focus::Detail(DetailControl::On);
        let mut screen = Screen::default();
        let _ = draw(&mut app, &mut screen);
        app.move_aim(&screen, Aim::Right);
        assert_eq!(
            app.power.as_ref().map(|popup| popup.verb),
            Some(PowerVerb::Off)
        );

        let mut app = sample();
        app.focus = Focus::Detail(DetailControl::On);
        let mut screen = Screen::default();
        let _ = draw(&mut app, &mut screen);
        app.move_aim(&screen, Aim::Left);
        assert!(app.prompt.is_some(), "h from On opens Create server");

        let mut app = sample();
        app.focus = Focus::Detail(DetailControl::On);
        let mut screen = Screen::default();
        let _ = draw(&mut app, &mut screen);
        app.move_aim(&screen, Aim::Down);
        assert_eq!(app.focus, Focus::ServerPane(ServerPane::Jobs));
        assert_eq!(app.server_pane, ServerPane::Jobs);
        app.move_aim(&screen, Aim::Right);
        assert_eq!(app.server_pane, ServerPane::Address);
        app.move_aim(&screen, Aim::Left);
        assert_eq!(app.server_pane, ServerPane::Jobs);

        let mut app = sample();
        app.focus = Focus::Detail(DetailControl::On);
        let mut screen = Screen::default();
        let _ = draw(&mut app, &mut screen);
        app.move_aim(&screen, Aim::Up);
        assert_eq!(app.focus, Focus::Tab(Tab::Servers));
        assert_eq!(app.tab, Tab::Servers);
    }

    fn hit(screen: &Screen, target: Target) -> ratatui::layout::Rect {
        hit_opt(screen, target).expect("hit")
    }

    fn hit_opt(
        screen: &Screen, target: Target,
    ) -> Option<ratatui::layout::Rect> {
        screen
            .hits
            .iter()
            .find(|hit| hit.target == target)
            .map(|hit| hit.area)
    }

    fn hit_line(buffer: &ratatui::buffer::Buffer, needle: &str) -> u16 {
        let chars: Vec<char> = needle.chars().collect();
        for y in 0..buffer.area.height {
            let mut matched = 0usize;
            for x in 0..buffer.area.width {
                let ch = buffer
                    .cell((x, y))
                    .unwrap()
                    .symbol()
                    .chars()
                    .next()
                    .unwrap_or(' ');
                if matched < chars.len() && ch == chars[matched] {
                    matched += 1;
                    if matched == chars.len() {
                        return y;
                    }
                } else if ch == chars[0] {
                    matched = 1;
                } else {
                    matched = 0;
                }
            }
        }
        panic!("{needle} missing");
    }

    fn glyph(
        buffer: &ratatui::buffer::Buffer, area: Rect, needle: &str,
    ) -> (Color, Modifier) {
        let chars: Vec<char> = needle.chars().collect();
        for y in area.y..area.bottom() {
            let mut matched = 0usize;
            let mut start = area.x;
            for x in area.x..area.right() {
                let cell = buffer.cell((x, y)).unwrap();
                let ch = cell.symbol().chars().next().unwrap_or(' ');
                if matched < chars.len() && ch == chars[matched] {
                    if matched == 0 {
                        start = x;
                    }
                    matched += 1;
                    if matched == chars.len() {
                        let first = buffer.cell((start, y)).unwrap();
                        return (first.fg, first.modifier);
                    }
                } else if ch == chars[0] {
                    start = x;
                    matched = 1;
                    if chars.len() == 1 {
                        return (cell.fg, cell.modifier);
                    }
                } else {
                    matched = 0;
                }
            }
        }
        panic!("{needle} missing");
    }
}
