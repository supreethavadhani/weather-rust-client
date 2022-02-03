use std::collections::HashMap;
use reqwest::header::{AUTHORIZATION};

mod model;
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("username", "username");
    map.insert("password", "password");


    //v1/auth Endpoint
    let response = client
        .post("http://localhost:3000/v1/auth")
        .json(&map)
        .send()
        .await?;

    let token_response = response
    .json::<model::TokenResponse>()
    .await?;

    //v1/hello endpoint
    let response1 = client
    .get("http://localhost:3000/v1/hello")
    .header(AUTHORIZATION, "Bearer ".to_owned()+&token_response.access_token)
    .send()
    .await?;

    let hello = response1
    .json::<model::HelloResponse>()
    .await?;

    //v1/weather
    let response2 = client
    .get("http://localhost:3000/v1/weather")
    .header(AUTHORIZATION, "Bearer ".to_owned()+&token_response.access_token)
    .send()
    .await?;

    let weather = response2
    .json::<model::Weather>()
    .await?;

    println!(" {:?}", token_response);
    println!(" {:?}", hello);
    println!(" {:?}", weather);

    Ok(())
}