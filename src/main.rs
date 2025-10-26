use clap::Parser;
use reqwest::{Client, ClientBuilder, Error, Response};
use reqwest_hickory_resolver::HickoryResolver;
use serde::Serialize;
use serde_json::{Result, Value};
use std::env;
use std::sync::Arc;

/*
Keys we care about in the api return for single items
{
    "code": 200,
    "data": [{dict1:value}, {dict2:value}...],
    "result": "success"
}

Keys we care about for list endpoints
{
    "code": 200,
    "data": [{dict1:value}, {dict2:value}...],
    "result": "success"
    "meta": {pagination_info...}
}

We should have a set of types that lets us represent these parts at least.
*/

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    // Get our env settings
    dotenv::dotenv().ok();

    // Defaults
    let single: bool = false;
    let mbpkgid: i32 = 0;
    let servers: &str = "servers";
    let mbpkgid_str: &str = "&mbpkgid=";

    // parse our args into args
    let args = Args::parse();

    if args.mbpkgid >= 1 {
        let mbpkgid: i32 = args.mbpkgid;
        let servers: &str = "server";
    } else {
        let single: bool = true;
    }

    // build the client to use local resolver, IE Ipv4
    let mut builder = ClientBuilder::new();
    builder = builder.dns_resolver(Arc::new(HickoryResolver::default()));
    let test_client = builder.build();

    // let test_client = Arc::new(Client::builder().dns_resolver(resolver).build());
    let api_key = std::env::var("API_KEY").expect("Need to have API_KEY set in .env");
    let api_result = test_client
        .expect("lb")
        .get(format!(
            "https://vapi2.netactuate.com/api/cloud/{servers}/?key={api_key}&mbpkgid={mbpkgid}"
        ))
        .send()
        .await?;
    let res_str = api_result.text().await?;
    let res_json: Value = serde_json::from_str(&res_str).expect("blah");
    println!("{res_json:#}");
    Ok(())
}

///
/// This is the Args struct
///
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    // -m argument for picking an mbpkgid
    #[arg(short, long, default_value_t = 0)]
    mbpkgid: i32,
}
