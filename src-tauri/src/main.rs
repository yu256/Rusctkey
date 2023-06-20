// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod services;
use crate::services::DriveFile;

use async_recursion::async_recursion;
use chrono::{DateTime, Datelike, Duration, Local};
use once_cell::sync::Lazy;
use reqwest::multipart;
use serde_json::json;
use services::modules::note::{Reaction, Reactions};
use services::Note;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Read, Write};
use std::path::PathBuf;
use std::sync::RwLock;
use tauri::api::dialog::FileDialogBuilder;
use tauri::api::path::cache_dir;

static URL: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::from("")));
static TOKEN: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::from("")));

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

    if datetime.year() != current_datetime.year() {
        datetime.format("%Y/%m/%d|%X").to_string()
    } else if duration >= Duration::days(4) {
        datetime.format("%m/%d|%X").to_string()
    } else if duration >= Duration::days(1) {
        format!("{}日前", duration.num_days())
    } else if duration >= Duration::hours(1) {
        format!("{}時間前", duration.num_hours())
    } else if duration >= Duration::minutes(1) {
        format!("{}分前", duration.num_minutes())
    } else {
        format!("{}秒前", duration.num_seconds())
    }
}

fn open_file(path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

#[async_recursion]
async fn add_emojis(name: &str) -> String {
    let (reaction, _) = name[1..name.len() - 1].split_once("@").unwrap();
    let path = cache_dir().unwrap().join("emojis.json");
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
        None => {
            if fetch_emojis().await {
                add_emojis(name).await
            } else {
                String::from("")
            }
        }
    };

    url
}

async fn fetch_emojis() -> bool {
    let client: reqwest::Client = reqwest::Client::new();

    let res: Result<reqwest::Response, reqwest::Error> = client
        .post(&format!(
            "https://{}/api/emojis",
            URL.read().unwrap().clone()
        ))
        .json(&json!({}))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json_body = response.text().await.unwrap();
                let mut file =
                    BufWriter::new(File::create(cache_dir().unwrap().join("emojis.json")).unwrap());
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
        note.modifiedCreatedAt = Some(format_datetime(&note.createdAt));
        if let Some(ref mut renote) = &mut note.renote {
            renote.modifiedCreatedAt = Some(format_datetime(&renote.createdAt));
        }
    }
    res
}

#[tauri::command]
async fn fetch_notes(
    until_id: Option<String>,
    since_id: Option<String>,
    until_date: Option<String>,
) -> Vec<Note> {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let access_token: String = TOKEN.read().unwrap().clone();

    let mut json_body = json!({ "i": access_token, "limit": 20 });

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
        .post(&format!("https://{}/api/notes/timeline", url))
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
async fn post(text: String) -> bool {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let access_token: String = TOKEN.read().unwrap().clone();

    let res: Result<reqwest::Response, reqwest::Error> = client
        .post(&format!("https://{}/api/notes/create", url))
        .json(&json!({ "i": access_token, "text": text }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                true
            } else {
                false
            }
        }

        Err(_) => false,
    }
}

#[tauri::command]
fn set_token(token: String) {
    *TOKEN.write().unwrap() = token;
}

#[tauri::command]
fn set_instance(instance: &str) {
    let sliced_url = if instance.starts_with("https://") && instance.ends_with('/') {
        &instance[8..instance.len() - 1]
    } else if instance.starts_with("https://") {
        &instance[8..]
    } else {
        instance
    };
    *URL.write().unwrap() = sliced_url.to_string();
}

fn read_file_to_bytes(file_path: std::path::PathBuf) -> Vec<u8> {
    let mut file: BufReader<File> = open_file(&file_path).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

#[tauri::command]
async fn upload_files() -> Vec<DriveFile> {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
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
                    let access_token: String = TOKEN.read().unwrap().clone();
                    let file_bytes = read_file_to_bytes(path);
                    let now = Local::now().format("%Y%m%d-%H:%M:%S");

                    let form: multipart::Form =
                        multipart::Form::new().text("i", access_token).part(
                            "file",
                            multipart::Part::bytes(file_bytes).file_name(format!("{}", now)),
                        );

                    let res: DriveFile = client
                        .post(&format!("https://{}/api/drive/files/create", url))
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_token,
            set_instance,
            post,
            upload_files,
            fetch_notes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
