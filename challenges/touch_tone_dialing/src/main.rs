use serde::{Deserialize, Serialize};
mod dtmf;

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
    let wav_url = util::get_problem!(Input).wav_url;
    util::download(&file_path, &wav_url).await?;
    let sequence = dtmf::decode(file_path);
    let result = Output { sequence };
    println!("result  : {}", result.sequence);
    util::post_answer!(result);
    Ok(())
}
