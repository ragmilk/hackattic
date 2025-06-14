use serde::{Deserialize, Serialize};
use base64::prelude::*;


#[derive(Deserialize, Debug)]
struct Data{
    bytes: String
}

#[derive(Serialize, Debug)]
struct Response{
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    big_endian_double: f64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
    let json_data = reqwest::get(format!("https://hackattic.com/challenges/help_me_unpack/problem?access_token={}", access_token))
    .await.expect("Error: something went wrong with GET reqwest")
    .json::<Data>()
    .await.expect("Error: something went wrong with parsing to json");

    let decoded_data = BASE64_STANDARD.decode(json_data.bytes.as_bytes()).expect("Failed to Decode Base64");
    assert_eq!(decoded_data.len(), 32);
    assert_eq!(decoded_data[10], 0);
    assert_eq!(decoded_data[11], 0);

    let data_i32: [u8; 4] = decoded_data[0..4].try_into()?;
    let data_u32: [u8; 4] = decoded_data[4..8].try_into()?;
    let data_short: [u8; 2] = decoded_data[8..10].try_into()?;
    let data_float: [u8; 4] = decoded_data[12..16].try_into()?;
    let data_double: [u8; 8] = decoded_data[16..24].try_into()?;
    let data_big_endian_double: [u8; 8] = decoded_data[24..32].try_into()?;

    let int = i32::from_le_bytes(data_i32);
    let uint = u32::from_le_bytes(data_u32);
    let short = i16::from_le_bytes(data_short);
    let float = f32::from_le_bytes(data_float);
    let double = f64::from_le_bytes(data_double);
    let big_endian_double = f64::from_be_bytes(data_big_endian_double);

    let response = Response{int, uint, short, float, double, big_endian_double};
    
    let client = reqwest::Client::new();
    let res = client
    .post(format!("https://hackattic.com/challenges/help_me_unpack/solve?access_token={}", access_token))
    .json(&response)
    .send()
    .await?;

    println!("{:?}", res);
    println!("{:?}", res.text().await?);
    Ok(())
}