use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

pub async fn get_problem<T: DeserializeOwned>(problem: &str) -> Result<T, Box<dyn std::error::Error>> {
    let access_token = std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
    println!("\nFetching problem data...");
    let response = reqwest::get(format!("https://hackattic.com/challenges/{}/problem?access_token={}", problem, access_token))
        .await
        .expect("Error: something went wrong with GET reqwest")
        .json::<T>()
        .await
        .expect("Error: something went wrong with parsing to json");
    println!("Done!\n");
    Ok(response)
}

pub async fn post_answer<T: Serialize + Debug>(problem: &str, result: T, playground: bool) -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
    let client = reqwest::Client::new();
    let p = if playground { "&playground=1" } else { "" };
    println!("\nPosting solution...");
    let res = client
        .post(format!("https://hackattic.com/challenges/{problem}/solve?access_token={access_token}{p}",))
        .json(&result)
        .send()
        .await?;
    let response = res.text().await?;
    println!("{}", response);
    Ok(())
}

pub async fn post<T: Serialize + Debug>(url: &str, result: T) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post(url).json(&result).send().await?;
    println!("{:#?}", res.text().await?);
    Ok(())
}

pub async fn download(file_path: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(file_path).unwrap();
    println!("\nDownloading...");
    let response = reqwest::get(url).await?;
    let mut content = std::io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    println!("Done!\n");
    Ok(())
}
