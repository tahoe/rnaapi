# rnaapi

NetActuate API client, CLI, and terminal UI.

`API_KEY` is required. `API_ADDRESS` is optional and defaults to `https://vapi2.netactuate.com/api/`.

## rnacli

Commands are `rnacli <endpoint> <action>`.

### server

- `get` lists servers, or one server when `-m` is set.
- `terminate` terminates a server. `POST cloud/server/delete/{mbpkgid}` with `{}`.
- `cancel` cancels a package. `POST cloud/package/cancel`. `--cancel-type` is `Immediate` or `End of Billing Period`, and `--agree` is required.
- `build` builds a server. `POST cloud/server/buy_build`.
- `rebuild` rebuilds a server. `POST cloud/server/build/{mbpkgid}`.
- `on` starts a cloud server. `POST cloud/server/{mbpkgid}/start`. Alias `start`.
- `off` shuts a cloud server down. `POST cloud/server/{mbpkgid}/shutdown`. Alias `shutdown`.
- `reboot` reboots a cloud server. `POST cloud/server/{mbpkgid}/reboot`.

```text
rnacli server get
rnacli server get -m <mbpkgid>
rnacli server on -m <mbpkgid>
rnacli server off -m <mbpkgid>
rnacli server terminate -m <mbpkgid>
rnacli server cancel -m <mbpkgid> --cancel-type Immediate --agree
rnacli server build --fqdn <name> --plan <plan> --location <id> --image <id>
rnacli server rebuild -m <mbpkgid> --image <id>
```

Other commands: `dedicated on`, `dedicated off`, `dedicated reboot`, `dns get`, `ssh get`, `ssh new`, `job get`, `ip get`, `bandwidth get`, `invoice get`, `sizes get`, `location get`, `image get`, `account get`, `pubkey get`, and `generate-completions`.

## rnatui

Terminal UI for the same API. Tabs are Servers, SSH keys, Locations, DNS, and Account. On Servers the actions are Create server, On, Off, Reboot, Rebuild, Terminate, and Cancel. The selected server also has Jobs, Address, and Bandwidth.

## Install

From the workspace root:

```bash
cargo build --release
cp -f target/release/rnacli target/release/rnatui ~/.cargo/bin/
```

A push of a `v*` tag publishes a GitHub release with the Linux `rnacli` and `rnatui` binaries and a `.deb` that installs both under `/usr/bin`.
