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
