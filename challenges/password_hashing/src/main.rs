use base64::prelude::*;
use hmac::Mac;
use pbkdf2::pbkdf2_hmac;
use scrypt::{Params, scrypt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Pbkdf2 {
    hash: String,
    rounds: usize,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Scrypt {
    N: usize,
    r: usize,
    p: usize,
    buflen: usize,
    _control: String,
}

#[derive(Deserialize, Debug)]
struct Input {
    password: String,
    salt: String,
    pbkdf2: Pbkdf2,
    scrypt: Scrypt,
}

#[derive(Debug, Serialize)]
struct Output {
    sha256: String,
    hmac: String,
    pbkdf2: String,
    scrypt: String,
}

type HmacSha256 = hmac::Hmac<sha2::Sha256>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = util::get_problem!(Input);

    //sha256
    let plane_password = json_data.password;
    let sha256 = sha256::digest(&plane_password);
    println!("Calculated Sha256: {sha256}");

    //hmac sha256
    let salt = BASE64_STANDARD.decode(json_data.salt.as_bytes())?;
    let mut mac = HmacSha256::new_from_slice(&salt)?;
    mac.update(&plane_password.as_bytes());
    let hmac = mac.finalize().into_bytes().into_iter().collect::<Vec<u8>>();
    let hmac = u8_to_string(&hmac);
    println!("Calculated HmacSha256: {hmac}");

    //pkdf2
    let iterations = json_data.pbkdf2.rounds as u32;
    assert_eq!("sha256".to_string(), json_data.pbkdf2.hash);
    let mut key = vec![0_u8; 32];
    pbkdf2_hmac::<sha2::Sha256>(&plane_password.as_bytes(), &salt, iterations, &mut key);
    let pbkdf2 = u8_to_string(&key);
    println!("Calculated Pbkdf2: {pbkdf2}");

    //scrypt
    let buf_len = json_data.scrypt.buflen;
    let ex_params = Params::new(7, 4, 8, buf_len)?;
    let mut ex_key = vec![0_u8; buf_len];
    scrypt("rosebud".as_bytes(), "pepper".as_bytes(), &ex_params, &mut ex_key)?;
    assert_eq!(u8_to_string(&ex_key), json_data.scrypt._control);

    let log_n = format!("{:0b}", json_data.scrypt.N).len() as u8 - 1;
    let params = Params::new(log_n, json_data.scrypt.r as u32, json_data.scrypt.p as u32, buf_len)?;
    let mut key = vec![0_u8; buf_len];
    scrypt(&plane_password.as_bytes(), &salt, &params, &mut key)?;
    let scrypt = u8_to_string(&key);
    println!("Calculated Scrypt: {scrypt}");

    let result = Output { sha256, hmac, pbkdf2, scrypt };

    util::post_answer!(result);
    Ok(())
}

fn u8_to_string(v: &[u8]) -> String {
    v.iter().map(|s| format!("{:02x}", s)).collect::<String>()
}
