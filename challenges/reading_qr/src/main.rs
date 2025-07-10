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
    let qr_file_path = "qr.png";
    let url = util::get_problem!(Input).image_url;
    util::download(qr_file_path, &url).await?;

    let qr = image::open(qr_file_path)?.to_luma8();
    let mut qr = rqrr::PreparedImage::prepare(qr);
    let grids = qr.detect_grids();
    let code = grids[0].decode()?.1;

    let result = Output { code };
    util::post_answer!(result);

    Ok(())
}
