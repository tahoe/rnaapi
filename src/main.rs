use reqwest::{Client, ClientBuilder, Error, Response};
use serde_json::{Result, Value};
use std::env;

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
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("Need to have API_KEY set in .env");
    let test_client = Client::new();
    let api_result = test_client
        .get(format!(
            "https://vapi2.netactuate.com/api/cloud/servers?key={}",
            api_key
        ))
        .send()
        .await;
    let res_str = api_result.expect("bleh").text().await;
    let res_json: Value = serde_json::from_str(&res_str.expect("blah")).expect("blah");
    println!("{:#}", res_json);
    Ok(())
}
