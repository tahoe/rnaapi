// Copyright (C) 2026 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
//! Public-key upload helper. Same rules as `rnacli new ssh`, kept in this crate.

use anyhow::{Context, Result, bail};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[derive(Debug)]
pub struct NewSshKey {
    pub name: String,
    pub ssh_key: String,
    pub private_path: Option<PathBuf>,
}

/// Reads a public key file, or generates `~/.ssh/id_ed25519` when `file` is empty.
///
/// An existing private key or `.pub` file is left untouched.
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

fn expand_tilde(raw: &str) -> Result<PathBuf> {
    let home = || {
        env::var_os("HOME")
            .map(PathBuf::from)
            .context("HOME is not set")
    };
    if raw == "~" {
        return home();
    }
    if let Some(rest) = raw.strip_prefix("~/") {
        return Ok(home()?.join(rest));
    }
    Ok(PathBuf::from(raw))
}

fn ssh_dir() -> Result<PathBuf> {
    let home = env::var_os("HOME")
        .map(PathBuf::from)
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

fn read_public_key_at(pub_path: &Path) -> Result<String> {
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

fn resolve_public_key_file(file: &str) -> Result<PathBuf> {
    let raw = if file.contains('/') || file.starts_with('~') {
        expand_tilde(file)?
    } else {
        ssh_dir()?.join(file)
    };
    if raw.is_file() {
        return Ok(raw);
    }
    if raw.extension().and_then(|ext| ext.to_str()) != Some("pub") {
        let sibling = PathBuf::from(format!("{}.pub", raw.display()));
        if sibling.is_file() {
            return Ok(sibling);
        }
    }
    bail!("unable to open {}", raw.display())
}

fn key_name(path: &Path) -> String {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("id_ed25519");
    file_name
        .strip_suffix(".pub")
        .unwrap_or(file_name)
        .to_owned()
}

fn generate_ed25519(private: &Path, name: &str) -> Result<()> {
    // ssh-keygen must not read the terminal while the UI owns stdin.
    let output = std::process::Command::new("ssh-keygen")
        .args(["-q", "-t", "ed25519", "-f"])
        .arg(private)
        .args(["-N", "", "-C"])
        .arg(name)
        .stdin(std::process::Stdio::null())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static HOME_LOCK: Mutex<()> = Mutex::new(());

    const SAMPLE_KEY: &str = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGyoL5v/yhwoeobbw29Kdoadx0HGQoPquQfjvRRiLZjy rnaapi-test\n";

    struct HomeGuard {
        previous: Option<std::ffi::OsString>,
        dir: PathBuf,
    }

    impl HomeGuard {
        fn set(dir: PathBuf) -> Self {
            let previous = env::var_os("HOME");
            // SAFETY: HOME_LOCK is held, so these tests do not touch HOME at once.
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

    fn lock_home(
        name: &str,
    ) -> (std::sync::MutexGuard<'static, ()>, HomeGuard, PathBuf) {
        let guard = HOME_LOCK.lock().unwrap_or_else(|err| err.into_inner());
        let dir = env::temp_dir().join(format!(
            "rnatui-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|elapsed| elapsed.as_nanos())
                .unwrap_or(0)
        ));
        fs::create_dir_all(dir.join(".ssh")).unwrap();
        let home = HomeGuard::set(dir.clone());
        (guard, home, dir)
    }

    #[test]
    fn new_key_refuses_to_overwrite_an_existing_id_ed25519() {
        {
            let (_lock, _home, dir) = lock_home("priv");
            let private = dir.join(".ssh").join("id_ed25519");
            fs::write(&private, "do-not-clobber").unwrap();
            let err = new_ssh_key(None).unwrap_err();
            assert!(err.to_string().contains("already exists"), "{err}");
            assert_eq!(fs::read_to_string(&private).unwrap(), "do-not-clobber");
            assert!(!private.with_extension("pub").exists());
        }
        let (_lock, _home, dir) = lock_home("pub");
        let public = dir.join(".ssh").join("id_ed25519.pub");
        fs::write(&public, SAMPLE_KEY).unwrap();
        let err = new_ssh_key(None).unwrap_err();
        assert!(err.to_string().contains("already exists"), "{err}");
        assert_eq!(fs::read_to_string(&public).unwrap(), SAMPLE_KEY);
        assert!(!dir.join(".ssh").join("id_ed25519").exists());
    }

    #[test]
    fn a_public_key_file_is_read_by_path_or_bare_name() {
        let (_lock, _home, dir) = lock_home("read");
        fs::write(dir.join(".ssh").join("test.pub"), SAMPLE_KEY).unwrap();
        let from_file = new_ssh_key(Some("~/.ssh/test.pub")).unwrap();
        let from_name = new_ssh_key(Some("test.pub")).unwrap();
        assert_eq!(from_file.ssh_key, from_name.ssh_key);
        assert_eq!(from_file.name, "test");
        assert!(from_file.private_path.is_none());
        assert!(from_file.ssh_key.starts_with("ssh-ed25519 "));
    }
}
