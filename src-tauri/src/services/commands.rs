use reqwest::multipart;
use serde_json::json;
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};
use tauri::api::dialog::{FileDialogBuilder, MessageDialogBuilder};

use super::{
    defaults::err_notes,
    service::{fetch_emojis, read_file_to_bytes, DATAPATH, TOKEN, URL},
    DriveFile, Note,
};

#[tauri::command]
pub async fn fetch_notes(
    until_id: Option<String>,
    since_id: Option<String>,
    until_date: Option<String>,
) -> Vec<Note> {
    let client: reqwest::Client = reqwest::Client::new();
    let url: &str = &URL;

    let mut json_body = json!({ "i": *TOKEN, "limit": 20 });

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

    let Ok(res) = request
        .send()
        .await else { return err_notes(); };

    let mut deserialized: Vec<Note> = res.json().await.unwrap();

    if let Some(_) = since_id {
        deserialized.reverse();
    }

    for note in &mut deserialized {
        super::note_modifier::modify_notes(note).await;
    }
    deserialized
}

#[tauri::command]
pub async fn post(text: Option<String>, files: Option<Vec<DriveFile>>) -> bool {
    let client: reqwest::Client = reqwest::Client::new();
    let url: &str = &URL;

    let mut json_body = json!({ "i": *TOKEN, "text": text });

    if let Some(drive_files) = files {
        let id: Vec<&str> = drive_files
            .iter()
            .map(|drive_file| drive_file.id.as_str())
            .collect();
        json_body["fileIds"] = json!(id);
    }

    let res: Result<reqwest::Response, reqwest::Error> = client
        .post(&format!("https://{}/api/notes/create", url))
        .json(&json!(&json_body))
        .send()
        .await;

    match res {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

#[tauri::command]
pub async fn set_credentials(instance: Option<String>, token: Option<String>) -> bool {
    match (instance, token) {
        (Some(instance), Some(token)) => {
            let url: &str = if instance.starts_with("https://") && instance.ends_with('/') {
                &instance[8..instance.len() - 1]
            } else if instance.starts_with("https://") {
                &instance[8..]
            } else {
                &instance
            };

            if fetch_emojis(url, &token).await {
                let mut instance_file =
                    BufWriter::new(File::create(DATAPATH.join("instance")).unwrap());
                instance_file.write_all(url.as_bytes()).unwrap();
                let mut token_file = BufWriter::new(File::create(DATAPATH.join("i")).unwrap());
                token_file.write_all(token.as_bytes()).unwrap();
                true
            } else {
                MessageDialogBuilder::new(
                    "Error",
                    "Invalid credentials or Network error occurred while connecting to server.",
                )
                .show(|_| {});
                false
            }
        }
        _ => {
            MessageDialogBuilder::new("Error", "One or both fields are empty.").show(|_| {});
            false
        }
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

#[tauri::command]
pub async fn upload_files() -> Vec<DriveFile> {
    let client: reqwest::Client = reqwest::Client::new();
    let (drive_file_tx, drive_file_rx) = async_std::channel::bounded(1);

    let handle: async_std::task::JoinHandle<Vec<DriveFile>> = async_std::task::spawn(async move {
        let mut drive_file: Vec<DriveFile> = Vec::new();
        while let Ok(res) = drive_file_rx.recv().await {
            drive_file.extend(res);
        }
        drive_file
    });

    FileDialogBuilder::new().pick_files(move |file_paths: Option<Vec<std::path::PathBuf>>| {
        if let Some(paths) = file_paths {
            async_std::task::spawn(async move {
                let mut drive_file: Vec<DriveFile> = Vec::new();
                for path in paths {
                    let access_token: &str = &TOKEN;
                    let url: &str = &URL;
                    let file_bytes = read_file_to_bytes(&path).unwrap();
                    let file_name = path.file_name().unwrap();

                    let form: multipart::Form =
                        multipart::Form::new().text("i", access_token).part(
                            "file",
                            multipart::Part::bytes(file_bytes)
                                .file_name(file_name.to_string_lossy().to_string()),
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
