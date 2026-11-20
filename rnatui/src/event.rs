// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton,
    MouseEvent, MouseEventKind,
};
use ratatui::layout::Position;

use crate::focus::{Aim, Focus, Modal, Screen, Split, target_at};
use crate::model::App;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Continue,
    Quit,
}

pub fn on_event(app: &mut App, screen: &mut Screen, event: Event) -> Action {
    match event {
        Event::Key(key) => on_key(app, screen, key),
        Event::Mouse(mouse) => {
            on_mouse(app, screen, mouse);
            Action::Continue
        }
        _ => Action::Continue,
    }
}

fn on_key(app: &mut App, screen: &mut Screen, key: KeyEvent) -> Action {
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
        KeyCode::Char('q') if !app.typing() && app.modal() == Modal::None => {
            return Action::Quit;
        }
        _ => {}
    }
    if ctrl || key.modifiers.contains(KeyModifiers::ALT) {
        return Action::Continue;
    }
    if app.modal() == Modal::None
        && let Focus::Split(split) = app.focus
        && !app.typing()
        && let Some(delta) = split_delta(split, key.code)
    {
        screen.nudge(split, delta);
        return Action::Continue;
    }
    let repeat = key.kind == KeyEventKind::Repeat;
    match key.code {
        KeyCode::BackTab => app.focus_move(true),
        KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => {
            app.focus_move(true)
        }
        KeyCode::Tab => app.focus_move(false),
        KeyCode::Up => {
            if !app.nudge_choice(-1) {
                app.move_list(-1);
            }
        }
        KeyCode::Down => {
            if !app.nudge_choice(1) {
                app.move_list(1);
            }
        }
        KeyCode::Left => {
            let _ = app.nudge_choice(-1);
        }
        KeyCode::Right => {
            let _ = app.nudge_choice(1);
        }
        KeyCode::Enter if !repeat => app.activate_focused(),
        KeyCode::Backspace if app.typing() => app.backspace(),
        KeyCode::Char('h') if !app.typing() && !repeat => {
            if !app.nudge_choice(-1) {
                app.move_aim(screen, Aim::Left);
            }
        }
        KeyCode::Char('j') if !app.typing() && !repeat => {
            app.move_aim(screen, Aim::Down)
        }
        KeyCode::Char('k') if !app.typing() && !repeat => {
            app.move_aim(screen, Aim::Up)
        }
        KeyCode::Char('l') if !app.typing() && !repeat => {
            if !app.nudge_choice(1) {
                app.move_aim(screen, Aim::Right);
            }
        }
        KeyCode::Char(ch) if app.typing() => app.type_char(ch),
        KeyCode::Char(' ') if !repeat => app.activate_focused(),
        _ => {}
    }
    Action::Continue
}

/// Left and `-` shrink the list or the log. Right, `=`, and `+` grow it.
/// On the log rule, Up shrinks the log and Down grows it.
fn split_delta(split: Split, code: KeyCode) -> Option<i16> {
    let delta = match code {
        KeyCode::Left | KeyCode::Char('-') => -1,
        KeyCode::Right | KeyCode::Char('=') | KeyCode::Char('+') => 1,
        KeyCode::Up if split == Split::Rows => -1,
        KeyCode::Down if split == Split::Rows => 1,
        _ => return None,
    };
    Some(delta)
}

fn on_mouse(app: &mut App, screen: &mut Screen, mouse: MouseEvent) {
    let point = Position::new(mouse.column, mouse.row);
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if app.modal() == Modal::None
                && screen.columns_split.contains(point)
            {
                app.focus_split(Split::Columns);
                screen.drag = Some(Split::Columns);
            } else if app.modal() == Modal::None
                && screen.rows_split.contains(point)
            {
                app.focus_split(Split::Rows);
                screen.drag = Some(Split::Rows);
            } else {
                screen.drag = None;
                if let Some(target) =
                    target_at(screen, mouse.column, mouse.row, app.modal())
                {
                    app.activate_target(target);
                }
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => match screen.drag {
            Some(Split::Columns) => screen.drag_columns_to(mouse.column),
            Some(Split::Rows) => screen.drag_rows_to(mouse.row),
            None => {}
        },
        MouseEventKind::Up(MouseButton::Left) => screen.drag = None,
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
        let mut screen = Screen::default();
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Tab)),
            Action::Continue
        );
        assert_eq!(app.focus, Focus::Tab(Tab::SshKeys));
        assert_eq!(app.tab, Tab::Servers);
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Enter)),
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
        let mut screen = Screen::default();
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Esc)),
            Action::Continue
        );
        assert!(app.power.is_none());
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Esc)),
            Action::Quit
        );
    }

    #[test]
    fn a_focused_split_resizes_and_tab_resumes_the_old_outline() {
        let mut app = App::new("vapi2.netactuate.com");
        app.focus = Focus::Tab(Tab::Servers);
        let mut screen = Screen::default();
        screen.layout(
            ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: 100,
                height: 32,
            },
            0,
        );
        let width = screen.list_width;
        let log = screen.log_rows;
        let column = screen.columns_split.x;
        let row = screen.columns_split.y;
        on_event(
            &mut app,
            &mut screen,
            mouse(MouseEventKind::Down(MouseButton::Left), column, row),
        );
        assert_eq!(app.focus, Focus::Split(Split::Columns));
        on_event(
            &mut app,
            &mut screen,
            mouse(MouseEventKind::Drag(MouseButton::Left), column + 6, row),
        );
        assert_eq!(screen.list_width, width + 6);
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Left)),
            Action::Continue
        );
        assert_eq!(screen.list_width, width + 5);
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Char('-'))),
            Action::Continue
        );
        assert_eq!(screen.list_width, width + 4);
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Char('='))),
            Action::Continue
        );
        assert_eq!(screen.list_width, width + 5);
        on_event(
            &mut app,
            &mut screen,
            mouse(MouseEventKind::Up(MouseButton::Left), 0, 0),
        );
        assert!(screen.drag.is_none());
        // Tab leaves the divider and continues from Servers, so the next
        // outline is SSH keys. The list selection is untouched.
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Tab)),
            Action::Continue
        );
        assert_eq!(app.focus, Focus::Tab(Tab::SshKeys));
        assert_eq!(app.tab, Tab::Servers);

        app.focus_split(Split::Rows);
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Down)),
            Action::Continue
        );
        assert_eq!(screen.log_rows, log + 1);
        assert_eq!(
            on_event(&mut app, &mut screen, key(KeyCode::Up)),
            Action::Continue
        );
        assert_eq!(screen.log_rows, log);
        let log_row = screen.rows_split.y;
        on_event(
            &mut app,
            &mut screen,
            mouse(MouseEventKind::Down(MouseButton::Left), 0, log_row),
        );
        on_event(
            &mut app,
            &mut screen,
            mouse(MouseEventKind::Drag(MouseButton::Left), 0, 0),
        );
        assert!(screen.log_rows > log);
        assert!(screen.log_rows >= 3);
    }

    #[test]
    fn h_l_and_arrows_cycle_a_highlighted_dropdown() {
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
        app.focus = Focus::Action(crate::focus::Action::CancelServer);
        let mut screen = Screen::default();
        on_event(&mut app, &mut screen, key(KeyCode::Enter));
        on_event(&mut app, &mut screen, key(KeyCode::Enter));
        let value = |app: &App| app.prompt.as_ref().unwrap().fields[1].value.clone();
        on_event(&mut app, &mut screen, key(KeyCode::Char('l')));
        assert_eq!(value(&app), "Immediate");
        on_event(&mut app, &mut screen, key(KeyCode::Right));
        assert_eq!(value(&app), "End of Billing Period");
        on_event(&mut app, &mut screen, key(KeyCode::Char('h')));
        assert_eq!(value(&app), "Immediate");
        on_event(&mut app, &mut screen, key(KeyCode::Left));
        assert_eq!(value(&app), "End of Billing Period");
    }

    fn mouse(kind: MouseEventKind, column: u16, row: u16) -> Event {
        Event::Mouse(MouseEvent {
            kind,
            column,
            row,
            modifiers: KeyModifiers::NONE,
        })
    }
}
