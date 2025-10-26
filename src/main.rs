use reqwest::{Client, ClientBuilder, Error, Response};
use reqwest_hickory_resolver::HickoryResolver;
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
    dotenv::dotenv().ok();

    let mut builder = ClientBuilder::new();
    builder = builder.dns_resolver(Arc::new(HickoryResolver::default()));
    let test_client = builder.build();

    // let test_client = Arc::new(Client::builder().dns_resolver(resolver).build());
    let api_key = std::env::var("API_KEY").expect("Need to have API_KEY set in .env");
    let api_result = test_client
        .expect("lb")
        .get(format!(
            "https://vapi2.netactuate.com/api/cloud/server/?key={api_key}&mbpkgid=571543"
        ))
        .send()
        .await?;
    let res_str = api_result.text().await?;
    let res_json: Value = serde_json::from_str(&res_str).expect("blah");
    println!("{res_json:#}");
    Ok(())
}
