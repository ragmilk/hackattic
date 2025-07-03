use itertools::Itertools;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::time::Instant;

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
    let zip_file_path = "problem.zip";
    let start_time = Instant::now();
    let json_data = util::get_problem::<Input>("brute_force_zip").await?;
    util::download(zip_file_path, &json_data.zip_url).await?;

    println!("\nStart Password Finder...");
    if let Some(password) = brute_force_zip(zip_file_path) {
        println!("\nPassword found: {}", password);
        let secret = get_content(zip_file_path, password);
        let result = Output { secret };
        util::post_answer("brute_force_zip", result, false).await?;
    } else {
        println!("\nPassword not found.");
    }
    println!("\n Spent time: {} seconds.", start_time.elapsed().as_secs());
    Ok(())
}

fn brute_force_zip(zip_file_path: &str) -> Option<String> {
    let zip_data = std::fs::read(zip_file_path).unwrap();
    let cursor = std::io::Cursor::new(&zip_data);
    let base_archive = zip::ZipArchive::new(cursor).unwrap();

    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    (4..=6)
        .flat_map(|len| {
            (0..len)
                .map(|_| chars.iter())
                .multi_cartesian_product()
                .map(|p| p.into_iter().collect::<String>())
        })
        .par_bridge()
        .find_any(|try_password| {
            let mut archive = base_archive.clone();
            if let Ok(mut zip) = archive.by_index_decrypt(0, try_password.as_bytes()) {
                let mut buffer = Vec::new();
                if zip.read_to_end(&mut buffer).is_ok() {
                    return true;
                }
            }
            false
        })
}

fn get_content(zip_file_path: &str, password: String) -> String {
    let file = std::fs::File::open(zip_file_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut buffer = Vec::new();
    let mut secret_file = archive.by_name_decrypt("secret.txt", password.as_bytes()).unwrap();
    secret_file.read_to_end(&mut buffer).unwrap();
    let secret = String::from_utf8(buffer).unwrap().trim().to_string();
    secret
}
