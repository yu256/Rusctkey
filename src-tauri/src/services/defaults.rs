use super::{modules::note::User, Note};
use std::collections::HashMap;

pub(crate) fn err_notes() -> Vec<Note> {
    vec![Note {
        id: String::new(),
        createdAt: String::new(),
        modifiedCreatedAt: None,
        user: User {
            username: String::new(),
            host: None,
            name: None,
            avatarUrl: String::new(),
            instance: None,
            onlineStatus: None,
            emojis: HashMap::new(),
        },
        text: Some(String::from(
            "サーバーに接続されていないか、トークンが無効になっています。",
        )),
        reactions: HashMap::new(),
        reactionEmojis: HashMap::new(),
        modifiedEmojis: None,
        files: Vec::new(),
        renote: None,
        emojis: Some(HashMap::new()),
    }]
}
