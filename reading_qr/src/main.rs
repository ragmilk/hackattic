use image;
use rqrr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Input {
    image_url: String,
}

#[derive(Serialize, Debug)]
struct Output {
    code: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = util::get_problem::<Input>("backup_restore").await?;
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
    util::post_answer::<Output>("backup_restore", result, false).await?;

    Ok(())
}
