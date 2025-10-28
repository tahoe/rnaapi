use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

lazy_static! {
    pub static ref API_ADDRESS: String = set_address();
}

fn set_address() -> String {
    dotenv().ok();
    let address =
        std_env::var("API_ADDRESS").expect("API_ADDRESS needs to be set in the env or .env file");
    if address.is_empty() {
        panic!("API_ADDRESS must not be empty!");
    }
    address
}

lazy_static! {
    pub static ref API_KEY: String = set_key();
}

fn set_key() -> String {
    dotenv().ok();
    let apikey = std_env::var("API_KEY").expect("APP_KEY needs to be set in the env or .env file");
    if apikey.is_empty() {
        panic!("API_KEY must not be empty!");
    }
    apikey
}
