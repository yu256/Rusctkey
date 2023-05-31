use serde::{Deserialize, Serialize};

use super::note::Properties;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct DriveFile {
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
