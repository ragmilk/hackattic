#[derive(serde::Serialize, serde::Deserialize)]
struct Answer {
    message: Option<String>,
    result: Option<String>,
    rejected: Option<String>,
}

#[doc(hidden)]
pub fn print_response(response_text: &String) {
    println!();
    match serde_json::from_str::<Answer>(&response_text) {
        Ok(answer) => {
            if answer.message.is_none() && answer.result.is_none() && answer.rejected.is_none() {
                println!("Raw Response:");
                println!("{}", response_text);
            } else {
                if let Some(_) = &answer.message {
                    println!("Consider 'HACKATTIC_PLAYGROUND=1 cargo run --release'");
                }
                if let Some(result) = &answer.result {
                    println!("{}", result);
                }
                if let Some(rejected) = &answer.rejected {
                    println!("{}", rejected);
                }
            }
        }
        Err(_) => {
            println!("Raw Response:");
            println!("{}", response_text);
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! form_url {
    ($t:expr) => {{
        let cdir = std::env::current_dir().unwrap();
        let path = cdir.file_name().unwrap().to_str().unwrap();
        let access_token = std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
        let playground = match std::env::var("HACKATTIC_PLAYGROUND") {
            Ok(_) => format!("&playground=1"),
            Err(_) => String::new(),
        };
        let url = format!("https://hackattic.com/challenges/{}/{}?access_token={}{}", path, $t, access_token, playground);
        url
    }};
}

#[macro_export]
macro_rules! get_problem {
    ($T:ty) => {{
        println!("\nFetching problem data...");
        let url = util::form_url!("problem");
        util::get_problem_for_macro::<$T>(&url).await?
    }};
}

#[macro_export]
macro_rules! post_answer {
    ($res:expr) => {
        println!("\nPosting solution...");
        let url = util::form_url!("solve");
        let res = util::post_answer_for_macro(&url, $res).await?;
        util::print_response(&res);
    };
}

#[doc(hidden)]
pub async fn get_problem_for_macro<T: serde::de::DeserializeOwned + std::fmt::Debug>(url: &String) -> Result<T, Box<dyn std::error::Error>> {
    let response = reqwest::get(url)
        .await
        .expect("Error: something went wrong with GET reqwest")
        .json::<T>()
        .await
        .expect("Error: something went wrong with parsing to json");
    println!("Done!\n");
    Ok(response)
}

#[doc(hidden)]
pub async fn post_answer_for_macro<T: serde::Serialize + std::fmt::Debug>(url: &String, result: T) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post(url).json(&result).send().await?;
    let response = res.text().await?;
    Ok(response)
}

pub async fn post<T: serde::Serialize + std::fmt::Debug>(url: &String, result: T) -> Result<(), Box<dyn std::error::Error>> {
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
