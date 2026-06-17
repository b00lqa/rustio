use std::{collections::HashMap, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = test_api("https://httpbin.org/ip").await?;
    println!("{resp:#?}");
    Ok(())
}

async fn test_api(url: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    Ok(reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?)
}
