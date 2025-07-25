use base64::prelude::*;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct Input {
    dump: String,
}

#[derive(Serialize, Debug)]
struct Output {
    alive_ssns: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = util::get_problem!(Input);
    let decoded_data = BASE64_STANDARD.decode(json_data.dump.as_bytes()).expect("Failed to Decode Base64");

    let mut decoder = GzDecoder::new(&decoded_data[..]);
    let mut decoded = String::new();
    decoder.read_to_string(&mut decoded).unwrap();

    let mut alive_ssns: Vec<String> = Vec::new();
    for l in decoded.lines() {
        if l.split('\t').last() == Some("alive") {
            alive_ssns.push(l.split('\t').nth(3).unwrap().to_string());
        }
    }
    let result = Output { alive_ssns };
    util::post_answer!(result);
    Ok(())
}
