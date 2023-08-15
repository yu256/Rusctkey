use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use serde_json::json;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};
use tauri::api::path::cache_dir;

pub static DATAPATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = cache_dir().unwrap();
    path.join(&path).join("com.yu256.rusctkey") // なぜか（Lazyのせい?）cache_dir().unwrap().join("com.yu256.rusctkey")とするとcache_dir().unwrap()の部分が空になる
});

pub static URL: Lazy<String> = Lazy::new(|| {
    let mut url = String::new();
    match open_file(&DATAPATH.join("instance")) {
        Ok(mut file) => {
            file.read_to_string(&mut url).unwrap();
            url
        }
        Err(_) => String::new(),
    }
});

pub static TOKEN: Lazy<String> = Lazy::new(|| {
    let mut url = String::new();
    match open_file(&DATAPATH.join("i")) {
        Ok(mut file) => {
            file.read_to_string(&mut url).unwrap();
            url
        }
        Err(_) => String::new(),
    }
});

pub(crate) async fn fetch_emojis(url: &str, token: &str) -> bool {
    let client: reqwest::Client = reqwest::Client::new();

    let res: Result<reqwest::Response, reqwest::Error> = client
        .post(&format!("https://{}/api/emojis", url))
        .json(&json!({ "i": token }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json_body = response.text().await.unwrap();
                let mut file = BufWriter::new(File::create(DATAPATH.join("emojis.json")).unwrap());
                file.write_all(json_body.as_bytes()).unwrap();
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

pub(crate) fn add_emojis(name: &str) -> Result<String> {
    let path = DATAPATH.join("emojis.json");

    let mut file = open_file(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let json: serde_json::Value = serde_json::from_str(&content)?;
    let emojis = json["emojis"].as_array().context("emojis is not array")?;

    let url = emojis.iter().find_map(|emoji| {
        let emoji_name = emoji["name"].as_str()?;
        if emoji_name == name {
            emoji["url"].as_str().map(|url| url.to_string())
        } else {
            None
        }
    });

    Ok(url.unwrap_or(String::new()))
}

pub fn read_file_to_bytes(file_path: &PathBuf) -> Result<Vec<u8>> {
    let mut file: BufReader<File> = open_file(&file_path)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn open_file(path: &PathBuf) -> Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}
