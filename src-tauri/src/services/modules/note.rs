use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Note {
    pub id: String,
    pub createdAt: String,
    pub modifiedCreatedAt: Option<String>,
    pub user: User,
    pub text: Option<String>,
    pub reactions: HashMap<String, usize>,
    pub emojis: Vec<Emojis>,
    pub files: Vec<Files>,
    pub renote: Option<Renote>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Renote {
    pub id: String,
    pub createdAt: String,
    pub modifiedCreatedAt: Option<String>,
    pub user: User,
    pub text: Option<String>,
    pub reactions: HashMap<String, usize>,
    pub emojis: Vec<Emojis>,
    pub files: Vec<Files>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Files {
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
    pub emojis: Vec<Emojis>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emojis {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Instance {
    pub name: String,
    pub softwareName: Option<String>,
    pub softwareVersion: Option<String>,
    pub iconUrl: String,
    pub faviconUrl: String,
    pub themeColor: String,
}
