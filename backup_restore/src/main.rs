use serde::{Deserialize, Serialize};
use base64::prelude::*;
use std::io::prelude::*;
use flate2::read::GzDecoder;

#[derive(Deserialize, Debug)]
struct Input {
    dump: String,
}

#[allow(dead_code)]
#[derive(Serialize, Debug)]
struct Output {
    alive_ssns: Vec<String>
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = get("backup_restore").await.expect("Error: something went wrong while getting data");
    let compressed_data = BASE64_STANDARD.decode(response.dump.as_bytes()).expect("Failed to Decode Base64");
    
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut decoded = String::new();
    decoder.read_to_string(&mut decoded).unwrap();

    let mut alive_ssns:Vec<String> = Vec::new();
    for l in decoded.lines(){
        if l.split('\t').last() == Some("alive") {
            alive_ssns.push(l.split('\t').nth(3).unwrap().to_string());           
        }        
    }
    let result = Output { alive_ssns };
    post("backup_restore", result).await.expect("Error: something went wrong while posting answer");
    Ok(())
}


async fn get(problem: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let access_token =
        std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
    let response= reqwest::get(format!(
        "https://hackattic.com/challenges/{}/problem?access_token={}",
        problem, access_token
    ))
    .await
    .expect("Error: something went wrong with GET reqwest")
    .json::<Input>()
    .await
    .expect("Error: something went wrong with parsing to json");
    Ok(response)
}

async fn post(problem: &str, result: Output) -> Result<(), Box<dyn std::error::Error>> {
    let access_token =
        std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "https://hackattic.com/challenges/{}/solve?access_token={}",
            problem, access_token
        ))
        .json(&result)
        .send()
        .await?;
    println!("{:#?}", res);
    println!("{:#?}", res.text().await?);
    Ok(())
}