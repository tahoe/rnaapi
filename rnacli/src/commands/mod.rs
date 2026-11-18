use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;
use std::io::BufRead;
use std::{env, fs, io, path};

pub fn get_pub_key(filepath: &str) -> Result<String> {
    let home = env::var_os("HOME")
        .map(path::PathBuf::from)
        .context("HOME is not set")?;
    let pub_path = home.join(".ssh").join(filepath);
    let file = fs::File::open(&pub_path)
        .with_context(|| format!("unable to open {}", pub_path.display()))?;
    let mut lines = io::BufReader::new(file).lines();
    let line = match lines.next() {
        Some(line) => line.with_context(|| {
            format!("unable to read {}", pub_path.display())
        })?,
        None => bail!("{} is empty", pub_path.display()),
    };
    if line.trim().is_empty() {
        bail!("{} is empty", pub_path.display());
    }
    let pubkey = openssh_keys::PublicKey::parse(&line)
        .with_context(|| format!("unable to parse {}", pub_path.display()))?;
    Ok(pubkey.to_string())
}

/// Key material for `rnacli new ssh`.
#[derive(Debug)]
pub struct NewSshKey {
    pub name: String,
    pub ssh_key: String,
    /// Private key path when this call generated a new pair.
    pub private_path: Option<path::PathBuf>,
}

fn expand_tilde(raw: &str) -> Result<path::PathBuf> {
    let home = || {
        env::var_os("HOME")
            .map(path::PathBuf::from)
            .context("HOME is not set")
    };
    if raw == "~" {
        return home();
    }
    if let Some(rest) = raw.strip_prefix("~/") {
        return Ok(home()?.join(rest));
    }
    Ok(path::PathBuf::from(raw))
}

fn ssh_dir() -> Result<path::PathBuf> {
    let home = env::var_os("HOME")
        .map(path::PathBuf::from)
        .context("HOME is not set")?;
    let dir = home.join(".ssh");
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("unable to create {}", dir.display()))?;
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
    }
    Ok(dir)
}

fn read_public_key_at(pub_path: &path::Path) -> Result<String> {
    let file = fs::File::open(pub_path)
        .with_context(|| format!("unable to open {}", pub_path.display()))?;
    let mut lines = io::BufReader::new(file).lines();
    let line = match lines.next() {
        Some(line) => line.with_context(|| {
            format!("unable to read {}", pub_path.display())
        })?,
        None => bail!("{} is empty", pub_path.display()),
    };
    if line.trim().is_empty() {
        bail!("{} is empty", pub_path.display());
    }
    Ok(line)
}

fn resolve_public_key_file(file: &str) -> Result<path::PathBuf> {
    let raw = if file.contains('/') || file.starts_with('~') {
        expand_tilde(file)?
    } else {
        ssh_dir()?.join(file)
    };
    if raw.is_file() {
        return Ok(raw);
    }
    if raw.extension().and_then(|ext| ext.to_str()) != Some("pub") {
        let sibling = path::PathBuf::from(format!("{}.pub", raw.display()));
        if sibling.is_file() {
            return Ok(sibling);
        }
    }
    bail!("unable to open {}", raw.display())
}

fn key_name(path: &path::Path) -> String {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("id_ed25519");
    file_name
        .strip_suffix(".pub")
        .unwrap_or(file_name)
        .to_owned()
}

fn generate_ed25519(private: &path::Path, name: &str) -> Result<()> {
    let output = std::process::Command::new("ssh-keygen")
        .args(["-q", "-t", "ed25519", "-f"])
        .arg(private)
        .args(["-N", "", "-C"])
        .arg(name)
        .output()
        .context("unable to run ssh-keygen")?;
    if output.status.success() {
        return Ok(());
    }
    let detail = String::from_utf8_lossy(&output.stderr);
    let detail = detail.trim();
    if detail.is_empty() {
        bail!("ssh-keygen failed");
    }
    bail!("ssh-keygen failed: {detail}")
}

/// Reads `-f`, or generates `~/.ssh/id_ed25519` when no file is given.
pub fn new_ssh_key(file: Option<&str>) -> Result<NewSshKey> {
    let (text, name, private_path) = if let Some(file) = file {
        let path = resolve_public_key_file(file)?;
        let text = read_public_key_at(&path)?;
        (text, key_name(&path), None)
    } else {
        let private = ssh_dir()?.join("id_ed25519");
        let public = private.with_extension("pub");
        if private.exists() || public.exists() {
            let existing = if private.exists() { &private } else { &public };
            bail!("{} already exists; pass -f <filename>", existing.display());
        }
        generate_ed25519(&private, "id_ed25519")?;
        let text = read_public_key_at(&public)?;
        (text, "id_ed25519".to_owned(), Some(private))
    };
    let pubkey = openssh_keys::PublicKey::parse(text.trim())
        .context("unable to parse the public key")?;
    Ok(NewSshKey {
        name,
        ssh_key: pubkey.to_string(),
        private_path,
    })
}

///
/// This is the CLI Args struct
///
#[derive(Parser, Debug)]
#[command(version, about)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Get {
        #[command(subcommand)]
        cmd: GetCommands,
    },
    /// Create a resource.
    New {
        #[command(subcommand)]
        cmd: NewCommands,
    },
    /// Start, stop, or reboot a virtual server.
    PowerVirt {
        #[command(subcommand)]
        cmd: PowerCommands,
    },
    /// Start, stop, or reboot a dedicated server.
    PowerDed {
        #[command(subcommand)]
        cmd: PowerCommands,
    },
    /// generate completions
    #[command(visible_alias = "gen")]
    GenerateCompletions { shell: Shell },
}

#[derive(Subcommand, Debug)]
pub enum GetCommands {
    /// Server subcommands
    #[command(visible_alias = "srv")]
    Server {
        /// Package id. Omit to list every server.
        #[arg(short, long)]
        mbpkgid: Option<u32>,
    },

    /// DNS subcommands
    Dns {
        /// Zone id. Omit to list every zone.
        #[arg(short, long)]
        id: Option<u32>,
    },

    /// SSh subcommands
    Ssh {
        /// Key id. Omit to list every key.
        #[arg(short, long)]
        id: Option<u32>,
    },

    /// Job subcommands
    Job {
        #[arg(short, long)]
        mbpkgid: u32,
        #[arg(short, long, default_value_t = 0)]
        jobid: u32,
    },

    /// IPs subcommands
    Ip {
        #[arg(short, long)]
        mbpkgid: u32,
    },

    /// Monthly Bandwidth subcommands
    #[command(visible_alias = "bw")]
    Bandwidth {
        #[arg(short, long)]
        mbpkgid: u32,
    },

    /// Invoices subcommands
    #[command(visible_alias = "inv")]
    Invoice {
        /// Number of invoices to print.
        #[arg(short, long, default_value_t = 5)]
        count: usize,
    },

    /// Sizes subcommands
    #[command(visible_alias = "sz")]
    Sizes {
        /// Location id.
        #[arg(short, long)]
        id: u32,
    },

    /// Location subcommands
    #[command(visible_alias = "loc")]
    Location {
        /// Location id. Omit to list every location.
        #[arg(short, long)]
        id: Option<u32>,
    },

    /// Images subcommands
    #[command(visible_alias = "img")]
    Image {},

    /// Account subcommands
    Account {},

    /// Print the first public key from a file in ~/.ssh.
    Pubkey {
        /// File name under ~/.ssh.
        #[arg(short, long, default_value = "butter_rsa.pub")]
        file: String,
    },
}

#[derive(Args, Debug)]
pub struct PowerTarget {
    /// Package id.
    #[arg(short, long)]
    pub mbpkgid: u32,
    /// Force the power change.
    #[arg(long)]
    pub force: bool,
}

#[derive(Subcommand, Debug)]
pub enum PowerCommands {
    /// Start the server.
    #[command(visible_alias = "start")]
    On(PowerTarget),
    /// Shut the server down.
    #[command(visible_alias = "shutdown")]
    Off(PowerTarget),
    /// Reboot the server.
    Reboot(PowerTarget),
}

#[derive(Subcommand, Debug)]
pub enum NewCommands {
    /// Upload an account SSH public key.
    Ssh {
        /// Public key file. A bare name is read from ~/.ssh.
        /// Omit this to generate ~/.ssh/id_ed25519 and publish its public key.
        #[arg(short, long)]
        file: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct HomeGuard {
        previous: Option<std::ffi::OsString>,
        dir: path::PathBuf,
    }

    impl HomeGuard {
        fn set(dir: path::PathBuf) -> Self {
            let previous = env::var_os("HOME");
            unsafe { env::set_var("HOME", &dir) };
            Self { previous, dir }
        }
    }

    impl Drop for HomeGuard {
        fn drop(&mut self) {
            match self.previous.clone() {
                Some(home) => unsafe { env::set_var("HOME", home) },
                None => unsafe { env::remove_var("HOME") },
            }
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    #[test]
    fn reads_the_first_public_key() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let dir =
            env::temp_dir().join(format!("rnaapi-ssh-{}", std::process::id()));
        let ssh = dir.join(".ssh");
        fs::create_dir_all(&ssh).unwrap();
        fs::write(
            ssh.join("test.pub"),
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGyoL5v/yhwoeobbw29Kdoadx0HGQoPquQfjvRRiLZjy rnaapi-test\n",
        )
        .unwrap();
        let _guard = HomeGuard::set(dir);
        let key = get_pub_key("test.pub").unwrap();
        assert!(key.starts_with("ssh-ed25519 "));
        assert!(key.contains("rnaapi-test"));
    }

    #[test]
    fn missing_public_key_is_an_error() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let dir = env::temp_dir()
            .join(format!("rnaapi-ssh-missing-{}", std::process::id()));
        fs::create_dir_all(dir.join(".ssh")).unwrap();
        let _guard = HomeGuard::set(dir);
        let err = get_pub_key("missing.pub").unwrap_err();
        assert!(err.to_string().contains("unable to open"));
    }

    #[test]
    fn a_public_key_file_is_read_by_path_or_name() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let dir = env::temp_dir()
            .join(format!("rnaapi-ssh-create-{}", std::process::id()));
        let ssh = dir.join(".ssh");
        fs::create_dir_all(&ssh).unwrap();
        let line = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGyoL5v/yhwoeobbw29Kdoadx0HGQoPquQfjvRRiLZjy rnaapi-test\n";
        fs::write(ssh.join("test.pub"), line).unwrap();
        let _guard = HomeGuard::set(dir);
        let from_file = new_ssh_key(Some("~/.ssh/test.pub")).unwrap();
        let from_name = new_ssh_key(Some("test.pub")).unwrap();
        assert_eq!(from_file.ssh_key, from_name.ssh_key);
        assert_eq!(from_file.name, "test");
        assert!(from_file.private_path.is_none());
        assert!(from_file.ssh_key.starts_with("ssh-ed25519 "));
    }

    #[test]
    fn a_missing_file_generates_a_pair_in_ssh() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let dir = env::temp_dir()
            .join(format!("rnaapi-ssh-gen-{}", std::process::id()));
        let _guard = HomeGuard::set(dir.clone());
        let created = new_ssh_key(None).unwrap();
        let private = dir.join(".ssh").join("id_ed25519");
        assert_eq!(created.private_path.as_deref(), Some(private.as_path()));
        assert_eq!(created.name, "id_ed25519");
        assert!(private.is_file());
        assert!(private.with_extension("pub").is_file());
        assert!(created.ssh_key.starts_with("ssh-ed25519 "));
        let err = new_ssh_key(None).unwrap_err();
        assert!(err.to_string().contains("already exists"));
    }
}
