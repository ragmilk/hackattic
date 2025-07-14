use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Bytes, Utf8Bytes, protocol::Message},
};

#[derive(Serialize, Debug)]
struct Output {
    secret: String,
}

#[derive(Deserialize, Debug)]
struct Input {
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = util::get_problem!(Input).token;
    let url = format!("wss://hackattic.com/_/ws/{token}");
    let intervals: Vec<u128> = vec![700, 1500, 2000, 2500, 3000];

    println!("Starting connection...");
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();
    let first_msg = read.next().await.unwrap()?;
    println!("msg: {first_msg:?}");
    let mut prev = Instant::now();
    let mut secret = String::new();

    while let Some(msg) = read.next().await {
        let now = Instant::now();
        let msg = msg?;
        println!("msg: {msg:?}");
        if let Message::Text(text) = msg {
            if text.contains("ping!") {
                let time_diff = (now - prev).as_millis();
                println!("  time_diff: {time_diff}");
                prev = now;
                let interval = intervals
                    .iter()
                    .min_by(|&&a, &&b| {
                        let a = time_diff.abs_diff(a);
                        let b = time_diff.abs_diff(b);
                        a.cmp(&b)
                    })
                    .unwrap();
                let interval_bytes = Bytes::from(interval.to_string());
                let response = unsafe { Message::Text(Utf8Bytes::from_bytes_unchecked(interval_bytes)) };
                println!("  gonna send: {response:?}");
                write.send(response).await?;
            } else if text.contains("ouch!") {
                return Ok(());
            } else if text.contains("congratulations!") {
                let parts: Vec<&str> = text.split('"').collect();
                secret = parts[1].to_string();
            }
        }
    }

    let result = Output { secret };
    util::post_answer!(result);
    Ok(())
}
