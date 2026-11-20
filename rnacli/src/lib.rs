//! Command definitions for the `rnacli` binary.
//!
//! The binary is `rnacli <endpoint> <action>`. It needs `API_KEY`.
//! `API_ADDRESS` is optional and defaults to `https://vapi2.netactuate.com/api/`.
//!
//! `server` subcommands are `get`, `terminate`, `cancel`, `build`, `rebuild`,
//! `on`, `off`, and `reboot`. `terminate` posts `cloud/server/delete/{mbpkgid}`
//! with `{}`. `cancel` posts `cloud/package/cancel`. `build` posts
//! `cloud/server/buy_build`. `rebuild` posts `cloud/server/build/{mbpkgid}`.
//! `on` posts `cloud/server/{mbpkgid}/start`. `off` posts
//! `cloud/server/{mbpkgid}/shutdown`.
//!
//! The other endpoints are `dedicated`, `dns`, `ssh`, `job`, `ip`,
//! `bandwidth`, `invoice`, `sizes`, `location`, `image`, `account`, `pubkey`,
//! and `generate-completions`.
//!
//! From the workspace root, `cargo build --release` writes `rnacli` and
//! `rnatui` under `target/release/`. Copy those binaries onto `PATH`.

pub mod commands;
