#![allow(dead_code, unused_imports)]
use ngrok::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct Output {
    solution: String,
}

#[derive(Deserialize, Debug)]
struct Credentials {
    user: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct Input {
    credentials: Credentials,
    ignition_key: String,
    trigger_token: String,
}

#[derive(Serialize, Debug)]
struct Registry {
    registry_host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    // publish localhost:3000 with ngrok
    let tunnel = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await?
        .http_endpoint()
        .listen_and_forward(url::Url::parse("http://localhost:3000").unwrap())
        .await?;

    let registry = Registry {
        registry_host: tunnel.url().to_string().strip_prefix("https://").unwrap().to_string(),
    };
    */

    //let json_data = util::get_problem::<Input>().await?;
    //let cred = json_data.credentials;
    //println!("{:?}", cred);

    let url = util::get_problem!(Input);
    println!("{url:?}");

    //let url = format!("https://hackattic/_/push/{}", json_data.trigger_token);
    //util::post(url.as_str(), registry).await?;

    /*
    let output = Output {
        solution: "dockerized_solutions".to_string(),
    };

    util::post_answer(output, false).await?;
    Ok(())
    */

    Ok(())
}
