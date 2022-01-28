//! Small tool to get public IP address.
#![forbid(missing_debug_implementations, missing_docs, unsafe_code)]

use std::collections::HashMap;
use reqwest::Client;

/// Make a request to the specified `url` and returns a `HashMap` containing the JSON response.
async fn json_resp(client: &Client, url: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = client.get(url)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let ipv4 = json_resp(&client, "https://api.ipify.org?format=json").await?;
    let ipv6 = json_resp(&client, "https://api64.ipify.org?format=json").await?;

    println!("IPv4: {}", ipv4.get("ip").unwrap());
    println!("IPv6: {}", ipv6.get("ip").unwrap());
    Ok(())
}
