use super::{
    modules::note::{Reaction, Reactions},
    parser::parse_text,
    service::{open_file, DATAPATH, URL},
    Note,
};
use chrono::{DateTime, Datelike, Duration, Local};
use serde_json::json;
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
};

pub(crate) async fn modify_notes(mut res: Vec<Note>) -> Vec<Note> {
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
                    String::new()
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
        if let Some(text) = note.text.take() {
            note.text = Some(parse_text(&text));
        }
        if let Some(mut renote) = note.renote.take() {
            if let Some(text) = renote.text {
                renote.text = Some(parse_text(&text));
            }
            note.renote = Some(renote);
        }
    }
    res
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
        None => String::new(),
    };

    url
}

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

async fn fetch_emojis() -> bool {
    let client: reqwest::Client = reqwest::Client::new();
    let url: &str = &URL;

    let res: Result<reqwest::Response, reqwest::Error> = client
        .post(&format!("https://{}/api/emojis", url))
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
