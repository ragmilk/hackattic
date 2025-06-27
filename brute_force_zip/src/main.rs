use serde::{Deserialize, Serialize};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Instant;

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
    let password_file_path = "data/passwords.txt";
    let zip_file_path = "data/problem.zip";

    println!("use 'cargo run --release'");
    println!("\nMake sure you have password.txt");
    println!("Do you wanna generate password file? [yn]: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    if input.trim() == "y" {
        pass_gen::password_generator()?;
    }

    let start_time = Instant::now();
    let json_data = util::get_problem::<Input>("brute_force_zip").await?;
    let mut file = std::fs::File::create(zip_file_path).unwrap();
    let response = reqwest::get(json_data.zip_url)
        .await
        .expect("Error: something went wrong with GET zip file");
    let mut content = std::io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)
        .expect("Error: something went wrong with writing zip file");

    println!("\nZip file downloaded.");
    println!("\nStart Password Finder...");
    let workers = 16;
    let password: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    pass_find::password_finder(zip_file_path, password_file_path, workers, password.clone());

    let password = password.lock().unwrap().clone();
    let secret = get_content(zip_file_path, password);
    let result = Output { secret };
    //println!("secret: {:?}", result);
    util::post_answer("brute_force_zip", result, false).await?;
    println!("\n Spent time: {} seconds.", start_time.elapsed().as_secs());
    Ok(())
}

fn get_content(zip_file_path: &str, password: String) -> String {
    let file = std::fs::File::open(zip_file_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut buffer = Vec::new();
    let mut secret_file = archive
        .by_name_decrypt("secret.txt", password.as_bytes())
        .unwrap();
    secret_file.read_to_end(&mut buffer).unwrap();
    let secret = String::from_utf8(buffer).unwrap().trim().to_string();
    secret
}
