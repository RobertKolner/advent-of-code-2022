use std::{path::Path, sync::Arc};

use reqwest::{cookie, Client, Url};
use tokio::fs;

pub async fn load_data(day: u8, aoc_token: String) -> Result<String, Box<dyn std::error::Error>> {
    if data_file_exists(day) {
        let data = data_file_load(day).await?;
        return Ok(data);
    } else {
        let data = request_data(day, aoc_token).await?;
        data_file_save(day, data.clone()).await?;
        return Ok(data);
    }
}

fn data_file_path_str(day: u8) -> String {
    format!("fixtures/day{:02}.txt", day)
}

fn data_file_exists(day: u8) -> bool {
    let path_str = data_file_path_str(day);
    Path::new(&path_str).exists()
}

async fn data_file_save(day: u8, content: String) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = data_file_path_str(day);
    let path = Path::new(&path_str);

    let parent_exists = match path.parent() {
        Some(parent) => parent.exists(),
        _ => true,
    };
    if !parent_exists {
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent).await?;
    }

    fs::write(path, content).await?;
    Ok(())
}

async fn data_file_load(day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let path_str = data_file_path_str(day);
    let path = Path::new(&path_str);
    let content = fs::read_to_string(path).await?;
    Ok(content)
}

async fn request_data(day: u8, aoc_token: String) -> Result<String, Box<dyn std::error::Error>> {
    let url_str = format!("https://adventofcode.com/2022/day/{}/input", day);
    let url = url_str.parse::<Url>().unwrap();

    let jar = cookie::Jar::default();
    let cookie = format!("session={}", aoc_token);
    jar.add_cookie_str(cookie.as_str(), &url);

    let jar_ref = Arc::from(jar);
    let client = Client::builder().cookie_provider(jar_ref).build()?;

    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}
