use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Note {
    pub id: String,
    pub createdAt: String,
    pub user: User,
    pub text: Option<String>,
    pub reactions: HashMap<String, usize>,
    pub reactionEmojis: HashMap<String, String>,
    pub modifiedEmojis: Option<Reactions>,
    pub files: Vec<File>,
    pub renote: Option<Renote>,
    pub emojis: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    pub name: String,
    pub url: String,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reactions {
    pub reactions: Vec<Reaction>,
}

impl Reactions {
    pub fn new() -> Reactions {
        Reactions {
            reactions: Vec::new(),
        }
    }
    pub fn add_reaction(&mut self, reaction: Reaction) {
        self.reactions.push(reaction);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Renote {
    pub id: String,
    pub createdAt: String,
    pub user: User,
    pub text: Option<String>,
    pub reactions: HashMap<String, usize>,
    pub reactionEmojis: HashMap<String, String>,
    pub modifiedEmojis: Option<Reactions>,
    pub files: Vec<File>,
    pub emojis: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct File {
    pub id: String,
    pub createdAt: String,
    pub name: String,
    pub r#type: String,
    pub md5: String,
    pub size: u32,
    pub isSensitive: bool,
    pub blurhash: Option<String>,
    pub properties: Properties,
    pub url: String,
    pub thumbnailUrl: Option<String>,
    pub comment: Option<String>,
    pub folderId: Option<String>,
    pub folder: Option<String>,
    pub userId: Option<String>,
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct User {
    pub username: String,
    pub host: Option<String>,
    pub name: Option<String>,
    pub avatarUrl: String,
    pub instance: Option<Instance>,
    pub onlineStatus: Option<String>,
    pub emojis: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Instance {
    pub name: Option<String>,
    pub softwareName: Option<String>,
    pub softwareVersion: Option<String>,
    pub iconUrl: Option<String>,
    pub faviconUrl: Option<String>,
    pub themeColor: Option<String>,
}
