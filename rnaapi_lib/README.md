# rnaapi

Async Rust client for the NetActuate API.

Set `API_KEY`. `API_ADDRESS` is optional and defaults to `https://vapi2.netactuate.com/api/`.

The `rnacli` crate in this workspace is the CLI. `rnatui` is the terminal UI. From the workspace root, `cargo build --release` writes both binaries under `target/release/`. Copy `rnacli` and `rnatui` onto `PATH`.
