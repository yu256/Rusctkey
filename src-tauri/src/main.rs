// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod services;
use crate::{services::DriveFile, services::Note};

use chrono::{DateTime, Local, Duration, Datelike};
use once_cell::sync::Lazy;
use reqwest::multipart;
use serde_json::json;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::RwLock;
use tauri::api::dialog::FileDialogBuilder;

static URL: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("".to_string()));
static TOKEN: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("".to_string()));

fn extract_noteid(note_id: String) -> String {
    let temp_id: Vec<&str> = note_id.split('/').collect();
    temp_id[4].to_string()
}

fn format_datetime(datetime_str: &str) -> String {
    let datetime = datetime_str
        .parse::<DateTime<Local>>()
        .unwrap();

    let current_datetime = Local::now();
    let duration = current_datetime.signed_duration_since(datetime);

    if datetime.year() != current_datetime.year() {
        datetime.format("%Y/%m/%d").to_string()
    } else if duration >= Duration::days(4) {
        datetime.format("%m/%d").to_string()
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

#[tauri::command]
async fn get_timeline() -> Vec<Note> {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let access_token: String = TOKEN.read().unwrap().clone();

    let mut res: Vec<Note> = client
        .post(&format!("https://{}/api/notes/timeline", url))
        .json(&json!({ "i": access_token, "limit": 20 }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    for note in &mut res {
        note.modifiedCreatedAt = Some(format_datetime(&note.createdAt));
        if let Some(ref mut renote) = &mut note.renote {
            renote.modifiedCreatedAt = Some(format_datetime(&renote.createdAt));
        }
    }

    res
}

#[tauri::command]
async fn pagination(id: String) -> Vec<Note> {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let access_token: String = TOKEN.read().unwrap().clone();

    let mut res: Vec<Note> = client
        .post(&format!("https://{}/api/notes/timeline", url))
        .json(&json!({ "i": access_token, "limit": 20, "untilId": id }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    for note in &mut res {
        note.modifiedCreatedAt = Some(format_datetime(&note.createdAt));
        if let Some(ref mut renote) = &mut note.renote {
            renote.modifiedCreatedAt = Some(format_datetime(&renote.createdAt));
        }
    }

    res
}

#[tauri::command]
async fn get_note(note_id: String) -> Note {
    let extracted_note_id: String = extract_noteid(note_id);
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let access_token: String = TOKEN.read().unwrap().clone();

    let mut res: Note = client
        .post(&format!("https://{}/api/notes/show", url))
        .json(&json!({ "i": access_token, "noteId": extracted_note_id }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    res.modifiedCreatedAt = Some(format_datetime(&res.createdAt));
    if let Some(ref mut renote) = &mut res.renote {
        renote.modifiedCreatedAt = Some(format_datetime(&renote.createdAt));
    }

    res
}

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
        Ok(_) => true,
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
    let mut file = BufReader::new(File::open(file_path).unwrap());
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

#[tauri::command]
async fn upload_file() -> Vec<String> {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = URL.read().unwrap().clone();
    let (file_id_tx, file_id_rx) = async_std::channel::bounded(1);

    let handle = async_std::task::spawn(async move {
        let mut file_id: Vec<String> = Vec::new();
        while let Ok(res) = file_id_rx.recv().await {
            file_id.extend(res);
        }
        file_id
    });

    FileDialogBuilder::new().pick_files(move |file_paths: Option<Vec<std::path::PathBuf>>| {
        match file_paths {
            Some(v) => {
                async_std::task::spawn(async move {
                    let mut file_id: Vec<String> = Vec::new();
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

                        file_id.push(res.id);
                    }
                    let _ = file_id_tx.send(file_id).await;
                });
            }
            _ => {}
        }
    });

    handle.await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_note,
            set_token,
            set_instance,
            post,
            upload_file,
            get_timeline,
            pagination
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
