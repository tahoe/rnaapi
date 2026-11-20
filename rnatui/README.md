# rnatui

Terminal UI for the NetActuate API. It uses the `rnaapi` crate.

```bash
export API_KEY='<your api key>'
# optional; defaults to https://vapi2.netactuate.com/api/
export API_ADDRESS='https://vapi2.netactuate.com/api/'
rnatui
```

Tabs are Servers, SSH keys, Locations, DNS, and Account. On Servers the actions are Create server, On, Off, Reboot, Rebuild, Terminate, and Cancel. The selected server also has Jobs, Address, and Bandwidth.

The same server operations are on `rnacli`: `get`, `terminate`, `cancel`, `build`, `rebuild`, `on`, and `off`. See the `rnacli` README.

## Install

From the workspace root, build the release binaries and copy them:

```bash
cargo build --release
cp -f target/release/rnacli target/release/rnatui ~/.cargo/bin/
```
