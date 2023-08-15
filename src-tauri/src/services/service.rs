use anyhow::{bail, Context as _, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::sync::LazyLock;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};
use tauri::api::path::cache_dir;

pub static DATAPATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = cache_dir().unwrap();
    path.join(&path).join("com.yu256.rusctkey") // なぜかcache_dir().unwrap().join("com.yu256.rusctkey")とするとcache_dir().unwrap()の部分が空になる
});

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub url: String,
    pub token: String,
}

impl Data {
    fn new() -> Self {
        Self {
            url: String::new(),
            token: String::new(),
        }
    }
}

pub static DATA: LazyLock<Data> = LazyLock::new(|| match read_json::<Data>("data") {
    Ok(conf) => conf,
    Err(_) => Data::new(),
});

pub(crate) async fn fetch_emojis(url: &str, token: &str) -> Result<()> {
    let client = reqwest::Client::new();

    let res = client
        .post(&format!("https://{}/api/emojis", url))
        .json(&json!({ "i": token }))
        .send()
        .await?;

    if res.status().is_success() {
        let json_body = res.text().await?;
        let mut file = BufWriter::new(File::create(DATAPATH.join("emojis.json"))?);
        file.write_all(json_body.as_bytes())?;
        Ok(())
    } else {
        bail!("{}", res.status());
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

pub fn read_json<T>(name: &str) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let mut file = open_file(&DATAPATH.join(format!("{}.json", name)))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn write_json<T>(data: &T, name: &str) -> Result<()>
where
    T: Serialize,
{
    let Ok(file) = File::create(&DATAPATH.join(format!("{}.json", name))) else {
        fs::create_dir_all(&*DATAPATH)?;
        return write_json(data, name);
    };

    let json = serde_json::to_string(&data)?;

    let mut file = BufWriter::new(file);
    file.write_all(json.as_bytes())?;

    Ok(())
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
