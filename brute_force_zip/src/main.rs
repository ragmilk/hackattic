use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod pass_find;
mod pass_gen;

#[derive(Deserialize, Debug)]
struct Input {
    zip_url: String,
}

#[derive(Serialize, Debug)]
struct Output {
    secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("I suggest you to use 'cargo run --release'");
    println!("Do you wanna generate password file? [yn]: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    if input.trim() == "y" {
        pass_gen::password_generator()?;
    } else {
        println!("\nMake sure you have password.txt")
    }

    let json_data = util::get::<Input>("brute_force_zip").await?;
    let mut file = std::fs::File::create("problem.zip").unwrap();
    let response = reqwest::get(json_data.zip_url)
        .await
        .expect("Error: something went wrong with GET zip file");
    let mut content = std::io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)
        .expect("Error: something went wrong with writing zip file");

    println!("\nZip file downloaded.");
    println!("\nStart Password Finder...");
    let workers = 16;
    let secret: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    pass_find::password_finder("problem.zip", "passwords.txt", workers, secret.clone());
    let secret = secret.lock().unwrap().clone();
    // only ascii
    let secret: String = secret.chars().filter(|c| c.is_ascii()).collect();
    let result = Output { secret };
    //println!("secret: {:?}", result);
    util::post("brute_force_zip", result, false).await?;
    Ok(())
}
