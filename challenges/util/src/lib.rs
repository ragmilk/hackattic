#[macro_export]
macro_rules! form_url {
    ($t:expr) => {{
        let cdir = std::env::current_dir().unwrap();
        let path = cdir.file_name().unwrap().to_str().unwrap();
        let access_token = std::env::var("HACKATTIC_ACCESS_TOKEN").expect("Please set HACKATTIC_ACCESS_TOKEN");
        let url = format!("https://hackattic.com/challenges/{}/{}?access_token={}", path, $t, access_token);
        url
    }};
}

#[macro_export]
macro_rules! get_problem {
    ($T:ty) => {{
        println!("\nFetching problem data...");
        let url = util::form_url!("problem");
        let response = reqwest::get(url)
            .await
            .expect("Error: something went wrong with GET reqwest")
            .json::<$T>()
            .await
            .expect("Error: something went wrong with parsing to json");
        println!("Done!\n");
        response
    }};
}

#[macro_export]
macro_rules! post_answer {
    ($res:expr) => {
        println!("\nPosting solution...");
        let url = util::form_url!("solve");
        let client = reqwest::Client::new();
        let res = client.post(url).json(&$res).send().await?;
        let response = res.text().await?;
        println!("{}", response);
    };
}

#[macro_export]
macro_rules! post_answer_with_playground {
    ($res:expr) => {
        println!("\nPosting solution with playground...");
        let url = util::form_url!("solve");
        let url = format!("{url}&playground=1");
        let client = reqwest::Client::new();
        let res = client.post(url).json(&$res).send().await?;
        let response = res.text().await?;
        println!("{}", response);
    };
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
