// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton,
    MouseEvent, MouseEventKind,
};
use ratatui::layout::Position;

use crate::focus::{Modal, Screen, target_at};
use crate::model::App;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Continue,
    Quit,
}

pub fn on_event(app: &mut App, screen: &Screen, event: Event) -> Action {
    match event {
        Event::Key(key) => on_key(app, key),
        Event::Mouse(mouse) => {
            on_mouse(app, screen, mouse);
            Action::Continue
        }
        _ => Action::Continue,
    }
}

fn on_key(app: &mut App, key: KeyEvent) -> Action {
    if key.kind == KeyEventKind::Release {
        return Action::Continue;
    }
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    match key.code {
        KeyCode::Char('c') if ctrl => return Action::Quit,
        KeyCode::Esc => {
            if app.modal() != Modal::None {
                app.cancel_popup();
                return Action::Continue;
            }
            return Action::Quit;
        }
        KeyCode::Char('q') if !app.typing() => return Action::Quit,
        _ => {}
    }
    if ctrl || key.modifiers.contains(KeyModifiers::ALT) {
        return Action::Continue;
    }
    let repeat = key.kind == KeyEventKind::Repeat;
    match key.code {
        KeyCode::BackTab => app.focus_move(true),
        KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => {
            app.focus_move(true)
        }
        KeyCode::Tab => app.focus_move(false),
        KeyCode::Up => app.move_list(-1),
        KeyCode::Down => app.move_list(1),
        KeyCode::Enter if !repeat => app.activate_focused(),
        KeyCode::Backspace if app.typing() => app.backspace(),
        KeyCode::Char(ch) if app.typing() => app.type_char(ch),
        KeyCode::Char(' ') if !repeat => app.activate_focused(),
        _ => {}
    }
    Action::Continue
}

fn on_mouse(app: &mut App, screen: &Screen, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(target) =
                target_at(screen, mouse.column, mouse.row, app.modal())
            {
                app.activate_target(target);
            }
        }
        MouseEventKind::ScrollUp => {
            scroll(app, screen, mouse.column, mouse.row, -1)
        }
        MouseEventKind::ScrollDown => {
            scroll(app, screen, mouse.column, mouse.row, 1)
        }
        _ => {}
    }
}

fn scroll(app: &mut App, screen: &Screen, column: u16, row: u16, delta: i32) {
    let point = Position::new(column, row);
    if screen.list.contains(point) {
        app.scroll_list(delta);
    } else if screen.detail.contains(point) {
        app.scroll_detail(delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::focus::{Focus, Tab};
    use crate::model::{PowerVerb, ServerKind, ServerRow};

    fn key(code: KeyCode) -> Event {
        Event::Key(KeyEvent::new(code, KeyModifiers::NONE))
    }

    #[test]
    fn tab_moves_focus_and_enter_activates() {
        let mut app = App::new("vapi2.netactuate.com");
        app.focus = Focus::Tab(Tab::Servers);
        let screen = Screen::default();
        assert_eq!(
            on_event(&mut app, &screen, key(KeyCode::Tab)),
            Action::Continue
        );
        assert_eq!(app.focus, Focus::Tab(Tab::SshKeys));
        assert_eq!(app.tab, Tab::Servers);
        assert_eq!(
            on_event(&mut app, &screen, key(KeyCode::Enter)),
            Action::Continue
        );
        assert_eq!(app.tab, Tab::SshKeys);
    }

    #[test]
    fn escape_closes_the_popup_before_it_quits() {
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
        app.open_power(PowerVerb::Reboot);
        let screen = Screen::default();
        assert_eq!(
            on_event(&mut app, &screen, key(KeyCode::Esc)),
            Action::Continue
        );
        assert!(app.power.is_none());
        assert_eq!(
            on_event(&mut app, &screen, key(KeyCode::Esc)),
            Action::Quit
        );
    }
}
