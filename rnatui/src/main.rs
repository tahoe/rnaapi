// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
//! Terminal UI for the NetActuate API.

use std::io::{self, Stdout};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture, EventStream};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
    enable_raw_mode,
};
use futures_util::StreamExt;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use rnaapi::NaClient;
use rnaapi::config::Settings;
use tokio::sync::mpsc::{
    UnboundedReceiver, UnboundedSender, unbounded_channel,
};

use crate::api::spawn;
use crate::event::{Action, on_event};
use crate::focus::Screen;
use crate::model::{App, Reply, api_host};

mod api;
mod event;
mod focus;
mod model;
mod ssh;
mod ui;

type Term = Terminal<CrosstermBackend<Stdout>>;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> anyhow::Result<()> {
    // Connect before the terminal is claimed so that await does not hold it.
    let (client, host, startup_error) = connect().await;
    let mut guard = TermGuard::install()?;
    let (tx, rx) = unbounded_channel();
    let mut app = App::new(host);
    if let Some(error) = startup_error {
        app.banner_error(error);
    } else {
        app.boot();
    }
    dispatch(&mut app, client.as_ref(), &tx);
    let result =
        event_loop(guard.terminal_mut(), &mut app, client, &tx, rx).await;
    // Stop taking results, then restore the screen before the runtime drops tasks.
    app.stop_accepting();
    drop(guard);
    result
}

async fn connect() -> (Option<Arc<NaClient>>, String, Option<String>) {
    match Settings::new() {
        Ok(settings) => {
            let host = api_host(&settings.api_url);
            match NaClient::new(settings.api_key, settings.api_url).await {
                Ok(client) => (Some(Arc::new(client)), host, None),
                Err(err) => (None, host, Some(err.to_string())),
            }
        }
        Err(err) => {
            (None, api_host(Settings::API_ADDRESS), Some(err.to_string()))
        }
    }
}

/// How often the open server list and its detail are fetched again.
const SERVER_REFRESH: Duration = Duration::from_secs(2);

async fn event_loop(
    terminal: &mut Term, app: &mut App, client: Option<Arc<NaClient>>,
    tx: &UnboundedSender<Reply>, mut rx: UnboundedReceiver<Reply>,
) -> anyhow::Result<()> {
    let mut events = EventStream::new();
    let mut screen = Screen::default();
    let mut server_refresh = tokio::time::interval(SERVER_REFRESH);
    server_refresh
        .set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    // The first tick is immediate. Boot already requested the server list.
    server_refresh.tick().await;
    loop {
        // `draw` finishes before the next await, so the backend is not borrowed
        // across a suspension point.
        if !app.typing() {
            let _ = terminal.hide_cursor();
        }
        terminal.draw(|frame| ui::render(frame, app, &mut screen))?;
        let action = tokio::select! {
            event = events.next() => match event {
                Some(Ok(event)) => on_event(app, &mut screen, event),
                Some(Err(err)) => return Err(anyhow::anyhow!("terminal input: {err}")),
                None => Action::Quit,
            },
            message = rx.recv() => {
                if let Some(reply) = message {
                    app.apply(reply);
                }
                Action::Continue
            }
            _ = server_refresh.tick(), if client.is_some() => {
                app.refresh_servers();
                Action::Continue
            }
        };
        dispatch(app, client.as_ref(), tx);
        if action == Action::Quit {
            break;
        }
    }
    Ok(())
}

fn dispatch(
    app: &mut App, client: Option<&Arc<NaClient>>, tx: &UnboundedSender<Reply>,
) {
    for request in app.drain_requests() {
        if let Some(client) = client {
            spawn(Arc::clone(client), request, tx.clone());
        } else {
            app.abandon(request);
        }
    }
}

struct TermGuard {
    terminal: Term,
}

impl TermGuard {
    fn install() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        if let Err(err) =
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        {
            let _ = disable_raw_mode();
            return Err(err);
        }
        let backend = CrosstermBackend::new(stdout);
        match Terminal::new(backend) {
            Ok(mut terminal) => {
                let _ = terminal.hide_cursor();
                Ok(Self { terminal })
            }
            Err(err) => {
                let _ = disable_raw_mode();
                let _ = execute!(
                    io::stdout(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                );
                Err(err)
            }
        }
    }

    fn terminal_mut(&mut self) -> &mut Term {
        &mut self.terminal
    }
}

impl Drop for TermGuard {
    fn drop(&mut self) {
        restore(&mut self.terminal);
    }
}

fn restore(terminal: &mut Term) {
    let _ = terminal.show_cursor();
    let _ = disable_raw_mode();
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
}
