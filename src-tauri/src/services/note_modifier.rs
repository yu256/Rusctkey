use super::{
    modules::note::{Reaction, Reactions},
    parser::{parse_customemojis, parse_text},
    service::add_emojis,
    Note,
};
use chrono::{DateTime, Datelike, Duration, Local};
use html_escape::encode_text;
use std::collections::HashMap;

pub(crate) async fn modify_notes(note: &mut Note) {
    modify_reactions(
        &mut note.modifiedEmojis,
        &mut note.reactions,
        &mut note.reactionEmojis,
    );
    note.modifiedCreatedAt = Some(format_datetime(&note.createdAt));
    note.user.name = Some(parse_customemojis(
        &encode_text(note.user.name.as_ref().unwrap_or(&note.user.username)),
        &note.user.emojis,
        note.user.host.is_none(),
    ));
    if let Some(ref mut renote) = &mut note.renote {
        if let Some(text) = &renote.text {
            renote.text = Some(parse_text(
                &text,
                &renote.emojis.as_ref().unwrap_or(&HashMap::new()),
                renote.user.host.is_none(),
            ));
        }
        renote.modifiedCreatedAt = Some(format_datetime(&renote.createdAt));
        renote.user.name = Some(parse_customemojis(
            &encode_text(renote.user.name.as_ref().unwrap_or(&renote.user.username)),
            &renote.user.emojis,
            renote.user.host.is_none(),
        ));
        modify_reactions(
            &mut renote.modifiedEmojis,
            &mut renote.reactions,
            &mut renote.reactionEmojis,
        );
    }
    if let Some(text) = note.text.take() {
        note.text = Some(parse_text(
            &text,
            &note.emojis.as_ref().unwrap_or(&HashMap::new()),
            note.user.host.is_none(),
        ));
    }
}

fn modify_reactions(
    emojis: &mut Option<Reactions>,
    reactions: &mut HashMap<String, usize>,
    reaction_emojis: &mut HashMap<String, String>,
) {
    let emojis = emojis.get_or_insert_with(Reactions::new);
    for (reaction, count) in reactions {
        let reaction = Reaction {
            name: reaction.to_string(),
            url: if reaction.starts_with(':') {
                if let Some(url) = reaction_emojis.get(&reaction[1..reaction.len() - 1]) {
                    url.to_string()
                } else {
                    add_emojis(&reaction[1..reaction.len() - 3])
                }
            } else {
                String::new()
            },
            count: *count,
        };
        emojis.add_reaction(reaction);
    }
    reaction_emojis.clear();
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
