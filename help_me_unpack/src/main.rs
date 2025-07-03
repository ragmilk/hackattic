use base64::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Input {
    bytes: String,
}

#[derive(Serialize, Debug)]
struct Output {
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    big_endian_double: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = util::get_problem::<Input>("help_me_unpack").await?;
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

    let result = Output {
        int,
        uint,
        short,
        float,
        double,
        big_endian_double,
    };

    util::post_answer::<Output>("help_me_unpack", result, false).await?;
    Ok(())
}
