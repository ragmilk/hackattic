use image;
use rqrr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Input {
    image_url: String,
}

#[allow(dead_code)]
#[derive(Serialize, Debug)]
struct Output {
    code: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token =
        std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
    let json_data = reqwest::get(format!(
        "https://hackattic.com/challenges/reading_qr/problem?access_token={}",
        access_token
    ))
    .await
    .expect("Error: something went wrong with GET reqwest")
    .json::<Input>()
    .await
    .expect("Error: something went wrong with parsing to json");

    let mut file = std::fs::File::create("qr.png").unwrap();
    let response = reqwest::get(json_data.image_url)
        .await
        .expect("Error: something went wrong with GET QR code");
    let mut content = std::io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)
        .expect("Error: something went wrong with writing QR code");

    let qr = image::open("qr.png")?.to_luma8();
    let mut qr = rqrr::PreparedImage::prepare(qr);
    let grids = qr.detect_grids();
    let code = grids[0].decode()?.1;
    let result = Output { code };

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "https://hackattic.com/challenges/reading_qr/solve?access_token={}",
            access_token
        ))
        .json(&result)
        .send()
        .await?;
    println!("{:#?}", res);
    println!("{:#?}", res.text().await?);

    Ok(())
}
