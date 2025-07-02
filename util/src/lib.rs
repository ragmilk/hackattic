use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

pub async fn get_problem<T: DeserializeOwned>(
    problem: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let access_token =
        std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
    println!("\nFetching problem data...");
    let response = reqwest::get(format!(
        "https://hackattic.com/challenges/{}/problem?access_token={}",
        problem, access_token
    ))
    .await
    .expect("Error: something went wrong with GET reqwest")
    .json::<T>()
    .await
    .expect("Error: something went wrong with parsing to json");
    println!("Done fetccing problem data!\n");
    Ok(response)
}

pub async fn post_answer<T: Serialize + Debug>(
    problem: &str,
    result: T,
    playground: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let access_token =
        std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
    let client = reqwest::Client::new();
    let p = if playground { "&playground=1" } else { "" };
    println!("\nPosting solution...");
    let res = client
        .post(format!(
            "https://hackattic.com/challenges/{problem}/solve?access_token={access_token}{p}",
        ))
        .json(&result)
        .send()
        .await?;
    println!("{:#?}", res);
    println!("{:#?}", res.text().await?);
    Ok(())
}

pub async fn download(file_path: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(file_path).unwrap();
    let response = reqwest::get(url).await?;
    let mut content = std::io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
