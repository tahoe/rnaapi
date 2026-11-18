# rnatui

This is a starting plan for the terminal UI. The first screens come later, after this plan.

## What this crate is

`rnatui` is a terminal UI for the same NetActuate API client that `rnacli` uses. It sits beside `rnacli` and talks to the API through the `rnaapi` library crate.

`rnacli` stays the command-line tool. `rnatui` is the screen you operate with the keyboard and the mouse.

The crate is a placeholder today. `Cargo.toml` names it `rnatui`, and `src/main.rs` is an empty `main`. This file only starts the plan. It is not a finished design.

## Requirements

- The application must use the ratatui library.
- Every selectable item must be reachable with Tab, and Shift-Tab must move backward through the same items. Tab and Shift-Tab move focus. They are not activation keys.
- A focused selectable item can be activated with a mouse click, Space, or Enter. Space and Enter are the usual keys for activating the focused control. A mouse click both focuses and activates the item that was clicked.

## How we will build it

The screen will be ratatui widgets. Selectable controls will sit in one focus order. Tab and Shift-Tab will walk that order forward and backward. They move focus and do not activate the focused control.

ratatui mouse capture will be on. Space and Enter are the usual keys for activating the focused control. A mouse click both focuses and activates the item that was clicked.

The five screen regions are described under [Screen layout](#screen-layout). A settled color palette waits until after this plan.

## Screen layout

The screen is five regions: a top bar, a tab row, a left pane, a right pane, and a bottom bar. The top bar shows `rnatui` and the API host. The tab row is Servers, SSH keys, Locations, DNS, and Account, and the open tab is filled. The left pane is the list for the open tab. The right pane is the detail for the selected list item, with its status and action buttons. On Servers, for example, the left pane is the server list, with a kind column that reads virtual or dedicated. The right pane is the chosen server. The bottom bar reads Tab move, Shift-Tab back, Enter or Space activate, and click activates. Tabs, list rows, and right-pane controls are all in the focus order.

Power is not a tab, because it acts on a server. The Servers detail has On, Off, and Reboot buttons and a Force checkbox. Each button opens a centered popup over the dimmed screen. The popup names the server by mbpkgid and fqdn and says whether it is virtual (`power-virt`) or dedicated (`power-ded`). It has Force, Confirm, and Cancel. Confirm starts with focus, and focus stays inside the popup until it closes.

Drawings and a short explanation are in [docs/LAYOUT.md](docs/LAYOUT.md).

## Workspace build

`rnatui` stays in the root `Cargo.toml` `exclude` list. `cargo build --release` keeps building only `rnacli` until the app is far enough along that the same command should produce both `rnacli` and `rnatui`.
