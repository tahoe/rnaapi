# Screen layout

`rnatui` is one dark terminal, about 100 columns by 32 rows. The drawings are 1000 by 640 pixels, that grid at 10 by 20. One control has a gold outline. That outline is the keyboard focus. Tab and Shift-Tab move it forward and backward. Enter, Space, and a click activate the outlined control, and a click also moves the outline onto the control under the pointer. Those rules stay as written in [PLAN.md](../PLAN.md).

The screen is five regions: a top bar, a tab row, a left pane, a right pane, and a bottom bar. A popup can open over them.

The top bar shows `rnatui` and the API host. These drawings use `vapi2.netactuate.com`. The host is the whole connection line. No key or token is shown.

The tab row sits under the top bar. Each tab is a control, in order: Servers, SSH keys, Locations, DNS, and Account. The open tab is filled. Activating a tab opens it. Power is not a tab. It belongs to a server, so it lives on the Servers detail and in the power popup.

The left pane is the list for the open tab. Each row is a control. The selected row is filled. Activating a row selects it, and the right pane follows the selection.

The right pane is the detail for the selected row: its fields, its status, and its action buttons.

The bottom bar is the key legend: Tab move, Shift-Tab back, Enter or Space activate, click activates.

Focus order is the tabs from left to right, then the list rows from top to bottom, then the controls in the right pane in reading order. The fill on a tab or a row shows what is open or selected. The gold outline shows focus, and only one control has it.

## Tabs

The Servers list is mbpkgid, fqdn, kind, and state. Kind is `virtual` or `dedicated`, so every row shows which power call it would use. The detail is fqdn, mbpkgid, kind, state, ip, and location, then the power buttons On, Off, and Reboot, and a Force checkbox. Job, IP, and bandwidth take an mbpkgid, so they belong in this detail pane too.

The SSH keys list is name and id. The detail is name, id, and a truncated fingerprint, then New key (`new ssh`). New key reads a public key file. A bare file name is read from `~/.ssh`. With no file it generates `~/.ssh/id_ed25519`, and it refuses when that key already exists instead of overwriting it.

Locations, DNS, and Account use this same frame. Sizes takes a location id, so it belongs in the Locations detail pane. Invoice and image stay with Account.

## Power popup

On, Off, and Reboot on the Servers detail do not send anything. Each one opens a small popup in the middle of the screen for the selected server. The page behind it is dimmed.

The popup title names the action, such as Reboot server. The body names the server by mbpkgid and fqdn and gives its kind. It also shows the call that Confirm will send. A virtual server uses `power-virt` and a dedicated server uses `power-ded`, for example `power-virt reboot -m 1001`. A Force checkbox adds `--force`. It starts with the value of Force on the Servers detail and can be changed in the popup. Confirm and Cancel close the popup. The key bar stays the bottom row.

When the popup opens, Confirm has the focus. Tab and Shift-Tab move only among Force, Confirm, and Cancel while the popup is open. Confirm sends the call and Cancel sends nothing. Either way, the popup closes and focus goes back to the button that opened it.

## Drawings

- [servers.svg](servers.svg) shows Servers open. The outline is on the list row for 1001 (`web-01.example`, virtual), and that server is in the right pane with On, Off, Reboot, and Force.
- [power.svg](power.svg) shows the popup that Reboot opened for 1001 (`web-01.example`, virtual, `power-virt`) over the dimmed Servers screen, with the outline on Confirm. The key bar stays at the bottom.
- [ssh.svg](ssh.svg) shows SSH keys open. The outline is on the list row for `laptop` (id 1001), and that key is in the right pane with New key.

Sample rows are fictional: server ids 1001 through 1005, names such as `web-01.example` and `metal-01.example` (1004 and 1005 are dedicated), the key `laptop`, and the documentation address `192.0.2.10`. These files are the layout. The crate stays a placeholder until the ratatui screen is built.
