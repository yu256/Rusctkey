use super::modules;
use crate::services::DriveFile;
use crate::services::Note;
use chrono::{DateTime, Datelike, Duration, Local};
use modules::note::{Reaction, Reactions};
use once_cell::sync::Lazy;
use reqwest::multipart;
use serde_json::json;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Error, Read, Write};
use std::path::PathBuf;
use tauri::api::dialog::FileDialogBuilder;
use tauri::api::path::cache_dir;

static DATAPATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = cache_dir().unwrap();
    path.join(&path).join("com.yu256.rusctkey") // なぜか（Lazyのせい?）cache_dir().unwrap().join("com.yu256.rusctkey")とするとcache_dir().unwrap()の部分が空になる
});

static URL: Lazy<String> = Lazy::new(|| {
    let mut url = String::new();
    let mut file = open_file(&DATAPATH.join("instance"))
        .expect("インスタンスのURLが格納されたファイルが存在しません。");
    file.read_to_string(&mut url).unwrap();
    url
});

static TOKEN: Lazy<String> = Lazy::new(|| {
    let mut url = String::new();
    let mut file =
        open_file(&DATAPATH.join("i")).expect("トークンが格納されたファイルが存在しません。");
    file.read_to_string(&mut url).unwrap();
    url
});

// fn extract_noteid(note_id: String) -> String {
//     let temp_id: Vec<&str> = note_id.split('/').collect();
//     temp_id[4].to_string()
// }

fn format_datetime(datetime_str: &str) -> String {
    let datetime = datetime_str
        .parse::<DateTime<Local>>()
        .unwrap_or(chrono::TimeZone::with_ymd_and_hms(&Local, 2023, 1, 1, 0, 0, 0).unwrap());

    let current_datetime = Local::now();
    let duration = current_datetime.signed_duration_since(datetime);

    if duration < Duration::minutes(1) {
        return format!("{}秒前", duration.num_seconds());
    }

    if duration < Duration::hours(1) {
        return format!("{}分前", duration.num_minutes());
    }

    if duration < Duration::days(1) {
        return format!("{}時間前", duration.num_hours());
    }

    if duration < Duration::days(4) {
        return format!("{}日前", duration.num_days());
    }

    if datetime.year() == current_datetime.year() {
        return datetime.format("%m/%d|%R").to_string();
    }

    datetime.format("%Y/%m/%d|%R").to_string()
}

fn open_file(path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

async fn add_emojis(name: &str) -> String {
    let reaction = &name[1..name.len() - 3];
    let path = DATAPATH.join("emojis.json");
    let mut file = match open_file(&path) {
        Ok(file) => file,
        Err(_) => {
            if fetch_emojis().await {
                open_file(&path).unwrap()
            } else {
                panic!("Not connected to server.")
            }
        }
    };
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    let emojis = json["emojis"]
        .as_array()
        .expect("emojis field does not exist in json.");

    let url = match emojis.iter().find_map(|emoji| {
        let emoji_name = emoji["name"].as_str().unwrap();
        if emoji_name == reaction {
            emoji["url"].as_str().map(|url| url.to_string())
        } else {
            None
        }
    }) {
        Some(emoji_url) => emoji_url,
        None => String::from(""),
    };

    url
}

async fn fetch_emojis() -> bool {
    let client: reqwest::Client = reqwest::Client::new();

    let res: Result<reqwest::Response, reqwest::Error> = client
        .post(&format!("https://{}/api/emojis", URL.to_string()))
        .json(&json!({}))
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

async fn modify_notes(mut res: Vec<Note>) -> Vec<Note> {
    for note in &mut res {
        note.modifiedEmojis = Some(Reactions::new());
        for (reaction, count) in &note.reactions {
            let reaction = Reaction {
                name: reaction.to_string(),
                url: if reaction.starts_with(':') {
                    if let Some(url) = note.reactionEmojis.get(&reaction[1..reaction.len() - 1]) {
                        url.to_string()
                    } else {
                        add_emojis(reaction).await
                    }
                } else {
                    String::from("")
                },
                count: *count,
            };
            if let Some(ref mut emojis) = note.modifiedEmojis {
                emojis.add_reaction(reaction);
            }
        }
        note.reactionEmojis.clear();
        note.modifiedCreatedAt = Some(format_datetime(&note.createdAt));
        if let Some(ref mut renote) = &mut note.renote {
            renote.modifiedCreatedAt = Some(format_datetime(&renote.createdAt));
        }
    }
    res
}

#[tauri::command]
pub async fn fetch_notes(
    until_id: Option<String>,
    since_id: Option<String>,
    until_date: Option<String>,
) -> Vec<Note> {
    let client: reqwest::Client = reqwest::Client::new();
    let url: &Lazy<String> = &URL;
    let access_token: &Lazy<String> = &TOKEN;

    let mut json_body = json!({ "i": **access_token, "limit": 20 });

    if let Some(id) = until_id {
        json_body["untilId"] = json!(id);
    }

    if let Some(id) = &since_id {
        json_body["sinceId"] = json!(id);
    }

    if let Some(until_date) = until_date {
        let num: u64 = until_date.parse().unwrap();
        json_body["untilDate"] = json!(num);
    }

    let request = client
        .post(&format!("https://{}/api/notes/timeline", url.to_string()))
        .json(&json_body);

    let mut res: Vec<Note> = request
        .send()
        .await
        .expect("Not connected to server.")
        .json()
        .await
        .unwrap();

    if let Some(_) = since_id {
        res.reverse();
    }

    modify_notes(res).await
}

// #[tauri::command]
// async fn get_note(note_id: String) -> Note {
//     let extracted_note_id: String = extract_noteid(note_id);
//     let client: reqwest::Client = reqwest::Client::new();
//     let url: String = URL.read().unwrap().clone();
//     let access_token: String = TOKEN.read().unwrap().clone();

//     let mut res: Note = client
//         .post(&format!("https://{}/api/notes/show", url))
//         .json(&json!({ "i": access_token, "noteId": extracted_note_id }))
//         .send()
//         .await
//         .unwrap()
//         .json()
//         .await
//         .unwrap();

//     res.modifiedCreatedAt = Some(format_datetime(&res.createdAt));
//     if let Some(ref mut renote) = &mut res.renote {
//         renote.modifiedCreatedAt = Some(format_datetime(&renote.createdAt));
//     }

//     res
// }

#[tauri::command]
pub async fn post(text: String) -> bool {
    let client: reqwest::Client = reqwest::Client::new();
    let url: &Lazy<String> = &URL;
    let access_token: &Lazy<String> = &TOKEN;

    let res: Result<reqwest::Response, reqwest::Error> = client
        .post(&format!("https://{}/api/notes/create", url.to_string()))
        .json(&json!({ "i": **access_token, "text": text }))
        .send()
        .await;

    match res {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

#[tauri::command]
pub async fn set_credentials(instance: String, token: String) -> bool {
    let url: &str = if instance.starts_with("https://") && instance.ends_with('/') {
        &instance[8..instance.len() - 1]
    } else if instance.starts_with("https://") {
        &instance[8..]
    } else {
        &instance
    };

    if ping(url, &token).await {
        let mut instance_file = BufWriter::new(File::create(DATAPATH.join("instance")).unwrap());
        instance_file.write_all(url.as_bytes()).unwrap();
        let mut token_file = BufWriter::new(File::create(DATAPATH.join("i")).unwrap());
        token_file.write_all(token.as_bytes()).unwrap();
        true
    } else {
        false
    }
}

async fn ping(url: &str, token: &str) -> bool {
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

#[tauri::command]
pub fn check_is_logged_in() -> bool {
    if let Ok(metadata) = fs::metadata(&DATAPATH.join("instance")) {
        metadata.is_file()
    } else {
        false
    }
}

fn read_file_to_bytes(file_path: std::path::PathBuf) -> Vec<u8> {
    let mut file: BufReader<File> = open_file(&file_path).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

#[tauri::command]
pub async fn upload_files() -> Vec<DriveFile> {
    let client: reqwest::Client = reqwest::Client::new();
    let url: &Lazy<String> = &URL;
    let (drive_file_tx, drive_file_rx) = async_std::channel::bounded(1);

    let handle: async_std::task::JoinHandle<Vec<DriveFile>> = async_std::task::spawn(async move {
        let mut drive_file: Vec<DriveFile> = Vec::new();
        while let Ok(res) = drive_file_rx.recv().await {
            drive_file.extend(res);
        }
        drive_file
    });

    FileDialogBuilder::new().pick_files(move |file_paths: Option<Vec<std::path::PathBuf>>| {
        if let Some(v) = file_paths {
            async_std::task::spawn(async move {
                let mut drive_file: Vec<DriveFile> = Vec::new();
                for path in v {
                    let access_token: &str = &TOKEN;
                    let file_bytes = read_file_to_bytes(path);
                    let now = Local::now().format("%Y%m%d-%H:%M:%S");

                    let form: multipart::Form =
                        multipart::Form::new().text("i", access_token).part(
                            "file",
                            multipart::Part::bytes(file_bytes).file_name(format!("{}", now)),
                        );

                    let res: DriveFile = client
                        .post(&format!(
                            "https://{}/api/drive/files/create",
                            url.to_string()
                        ))
                        .multipart(form)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    drive_file.push(res);
                }
                let _ = drive_file_tx.send(drive_file).await;
            });
        }
    });

    handle.await
}
