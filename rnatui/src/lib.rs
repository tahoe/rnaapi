use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::io::BufRead;
use std::{env, fs, io, path};

pub fn get_pub_key(filepath: &str) -> Option<String> {
    let home = env::home_dir().unwrap_or(path::PathBuf::from("~/"));
    let pub_path = home.join(".ssh").join(filepath);
    println!("Inspecting '{}':", pub_path.to_string_lossy());
    let file = fs::File::open(&pub_path).expect("unable to open pubkey");
    let reader = io::BufReader::new(file);

    let found_key = 'pub_key: {
        if let Some((i, line)) = reader.lines().enumerate().next() {
            let line = line.unwrap_or_else(|_| {
                panic!("Unable to read key at line {}", i + 1)
            });
            let pubkey = openssh_keys::PublicKey::parse(&line)
                .expect("unable to parse pubkey");
            break 'pub_key Some(pubkey);
            // println!(" * Pubkey #{} -> {:?}", i + 1, pubkey.to_string());
        }
        None
    };

    Some(found_key?.to_string())
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
        // -i argument for picking an mbpkgid
        #[arg(short, long, default_value_t = 0)]
        mbpkgid: u32,
    },

    /// DNS subcommands
    Dns {
        // -i argument for picking a dns zone
        #[arg(short, long, default_value_t = 0)]
        id: u32,
    },

    /// SSh subcommands
    Ssh {
        // -i argument for ssh keyid
        #[arg(short, long, default_value_t = 0)]
        id: u32,
    },

    /// Job subcommands
    Job {
        // -i argument for picking a Job
        #[arg(short, long)]
        mbpkgid: u32,
        #[arg(short, long, default_value_t = 0)]
        jobid: u32,
    },

    /// IPs subcommands
    Ip {
        // --proto argument (-p) for 4 or 6
        // default to 4
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
        // -i argument for number to display
        #[arg(short, long, default_value_t = 5)]
        count: usize,
    },

    /// Invoices subcommands
    #[command(visible_alias = "sz")]
    Sizes {
        // -i argument for number to display
        #[arg(short, long)]
        id: u32,
    },

    /// Location subcommands
    #[command(visible_alias = "loc")]
    Location {},

    /// Images subcommands
    #[command(visible_alias = "img")]
    Image {},

    /// Account subcommands
    Account {},

    /// Get Pub Key from file
    Pubkey {},
}
