use serde::{Deserialize, Serialize};
mod dtmf;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Input {
    wav_url: String,
}

#[derive(Serialize, Debug)]
struct Output {
    sequence: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "problem.wav";
    //let wav_url = util::get_problem::<Input>("touch_tone_dialing").await?.wav_url;
    //util::download(&file_path, &wav_url).await?;
    let sequence = dtmf::decode(file_path);
    let result = Output { sequence };
    println!("result: {result:?}");
    //util::post_answer("touch_tone_dialing", result, false).await?;
    Ok(())
}
