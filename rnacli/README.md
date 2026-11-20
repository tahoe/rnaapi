# rnacli

CLI for the NetActuate API. It uses the `rnaapi` crate.

```bash
export API_KEY='<your api key>'
# optional; defaults to https://vapi2.netactuate.com/api/
export API_ADDRESS='https://vapi2.netactuate.com/api/'
```

Commands are `rnacli <endpoint> <action>`.

## server

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

## Other commands

- `dedicated on`, `dedicated off`, and `dedicated reboot` change dedicated server power.
- `dns get` lists zones, or one zone when `-i` is set.
- `ssh get` lists account SSH keys. `ssh new` uploads a public key.
- `job get` lists jobs for a server.
- `ip get` lists addresses for a server.
- `bandwidth get` prints monthly bandwidth. Alias `bw`.
- `invoice get` prints recent invoices. Alias `inv`.
- `sizes get` lists plans at a location. Alias `sz`.
- `location get` lists locations. Alias `loc`.
- `image get` lists images. Alias `img`.
- `account get` prints account details.
- `pubkey get` prints the first public key from a file in `~/.ssh`.
- `generate-completions` writes a shell completion script. Alias `gen`.

## rnatui

`rnatui` is the terminal UI in this workspace. Its tabs are Servers, SSH keys, Locations, DNS, and Account. On Servers the actions are Create server, On, Off, Reboot, Rebuild, Terminate, and Cancel. The selected server also has Jobs, Address, and Bandwidth.

## Install

From the workspace root, build the release binaries and copy them:

```bash
cargo build --release
cp -f target/release/rnacli target/release/rnatui ~/.cargo/bin/
```
