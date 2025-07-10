use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha256::digest;

#[derive(Deserialize, Debug)]
struct Block {
    data: Value,
}

#[derive(Deserialize, Debug)]
struct Input {
    difficulty: usize,
    block: Block,
}

#[derive(Serialize, Debug)]
struct Output {
    nonce: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = util::get_problem!(Input);
    let diff = json_data.difficulty;
    let data = json_data.block.data;

    let mut nonce = 0;
    #[allow(while_true)]
    while true {
        if nonce % 100000 == 0 {
            println!("nonce @ {}", nonce);
        }

        let block = json!({
            "data": &data,
            "nonce": nonce
        });
        let block_string = serde_json::to_string(&block).expect("Error: something went wrong with parsing block to string");
        let hash = digest(block_string);
        let hash_binary = hash.chars().fold("".to_string(), |prev, c| {
            format!("{}{:04b}", prev, u8::from_str_radix(&c.to_string(), 16).unwrap())
        });
        if hash_binary.chars().take(diff).all(|c| c == '0') {
            break;
        }
        nonce += 1;
    }
    println!("nonce = {}", nonce);
    let result = Output { nonce };
    util::post_answer!(result);
    Ok(())
}
