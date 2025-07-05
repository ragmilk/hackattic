use core::str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Input {
    presence_token: String,
}

#[derive(Serialize, Debug)]
struct Output {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting Proxy List...");
    let proxy_list = {
        let response = reqwest::get(
            "https://api.proxyscrape.com/v2/?request=displayproxies&protocol=http&timeout=10000&country=all&ssl=all&anonymity=all&skip=0&limit=2000",
        )
        .await?;
        let text = response.text().await?;
        text.lines().map(|line| line.to_string()).collect::<Vec<String>>()
    };
    println!("Done!");

    let presence_token = util::get_problem::<Input>("a_global_presence").await?.presence_token;
    let url = format!("https://hackattic.com/_/presence/{presence_token}");

    println!("Connecting  Proxies...");
    let mut tasks = vec![];
    for proxy_url in proxy_list {
        let url_clone = url.clone();
        let task = tokio::spawn(async move {
            let proxy = match reqwest::Proxy::https(&proxy_url) {
                Ok(p) => p,
                Err(_) => {
                    return;
                }
            };

            let client = match reqwest::Client::builder().proxy(proxy).timeout(std::time::Duration::from_secs(10)).build() {
                Ok(c) => c,
                Err(_) => {
                    return;
                }
            };

            match client.get(&url_clone).send().await {
                Ok(response) => {
                    println!("Success from proxy {}: Status {}", proxy_url, response.status());
                }
                Err(_) => {
                    return;
                }
            }
        });
        tasks.push(task);
    }
    futures::future::join_all(tasks).await;
    let result = Output {};
    util::post_answer("a_global_presence", result, false).await?;
    Ok(())
}
