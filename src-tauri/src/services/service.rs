use once_cell::sync::Lazy;
use serde_json::json;
use std::{
    fs::File,
    io::{BufReader, Error, Read},
    path::PathBuf,
};
use tauri::api::path::cache_dir;

pub static DATAPATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = cache_dir().unwrap();
    path.join(&path).join("com.yu256.rusctkey") // なぜか（Lazyのせい?）cache_dir().unwrap().join("com.yu256.rusctkey")とするとcache_dir().unwrap()の部分が空になる
});

pub static URL: Lazy<String> = Lazy::new(|| {
    let mut url = String::new();
    let mut file = open_file(&DATAPATH.join("instance"))
        .expect("インスタンスのURLが格納されたファイルが存在しません。");
    file.read_to_string(&mut url).unwrap();
    url
});

pub static TOKEN: Lazy<String> = Lazy::new(|| {
    let mut url = String::new();
    let mut file =
        open_file(&DATAPATH.join("i")).expect("トークンが格納されたファイルが存在しません。");
    file.read_to_string(&mut url).unwrap();
    url
});

// トークンが有効かどうかの確認に使う
pub(crate) async fn ping(url: &str, token: &str) -> bool {
    let client: reqwest::Client = reqwest::Client::new();
    let res: Result<reqwest::Response, reqwest::Error> = client
        .post(&format!("https://{}/api/ping", url))
        .json(&json!({ "i": token }))
        .send()
        .await;

    match res {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

pub fn read_file_to_bytes(file_path: std::path::PathBuf) -> Vec<u8> {
    let mut file: BufReader<File> = open_file(&file_path).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn open_file(path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}
