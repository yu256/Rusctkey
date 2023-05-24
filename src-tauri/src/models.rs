use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct Note {
    pub createdAt: String,
    pub user: User,
    pub text: String,
    pub reactions: HashMap<String, usize>,
    pub emojis: Vec<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct User {
    pub username: String,
    pub host: Option<String>,
    pub name: String,
    pub avatarUrl: String,
    pub instance: Option<Instance>,
    pub onlineStatus: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Instance {
    pub name: String,
    pub softwareName: String,
    pub softwareVersion: String,
    pub iconUrl: String,
    pub faviconUrl: String,
    pub themeColor: String,
}