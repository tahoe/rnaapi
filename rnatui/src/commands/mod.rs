use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
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
}
