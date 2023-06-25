use super::{
    modules::note::{Reaction, Reactions},
    parser::{parse_text, parse_username},
    service::{open_file, DATAPATH},
    Note,
};
use chrono::{DateTime, Datelike, Duration, Local};
use std::{collections::HashMap, io::Read};

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
        note.user.name = Some(parse_username(
            note.user.name.as_ref().unwrap_or(&note.user.username),
            &note.user.emojis,
        ));
        if let Some(ref mut renote) = &mut note.renote {
            if let Some(text) = &renote.text {
                renote.text = Some(parse_text(
                    &text,
                    &renote.emojis.as_ref().unwrap_or(&HashMap::new()),
                ));
            }
            renote.modifiedCreatedAt = Some(format_datetime(&renote.createdAt));
            renote.user.name = Some(parse_username(
                renote.user.name.as_ref().unwrap_or(&renote.user.username),
                &renote.user.emojis,
            ));
        }
        if let Some(text) = note.text.take() {
            note.text = Some(parse_text(
                &text,
                &note.emojis.as_ref().unwrap_or(&HashMap::new()),
            ));
        }
    }
    res
}

async fn add_emojis(name: &str) -> String {
    let reaction = &name[1..name.len() - 3];
    let path = DATAPATH.join("emojis.json");

    let mut file = match open_file(&path) {
        Ok(file) => file,
        Err(_) => unreachable!(),
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
