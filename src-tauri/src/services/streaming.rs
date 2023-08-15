use super::{note_modifier::modify_notes, service::DATA};
use crate::services::Note;
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::Manager;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct StreamingBody {
    pub r#type: String,
    pub body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub id: String,
    pub r#type: String,
    pub body: Note,
}

pub async fn streaming(app_handle: tauri::AppHandle) -> Result<()> {
    let stream_url = format!("wss://{}/streaming?i={}", DATA.url, DATA.token);

    let (stream, _) = connect_async(stream_url).await?;

    let (mut write, mut read) = stream.split();

    write
        .send({
            let message = json!({
                "type": "connect",
                "body": {
                    "channel": "homeTimeline",
                    "id": "1",
                }
            });
            Message::Text(message.to_string())
        })
        .await?;

    while let Some(message) = read.next().await {
        if let Ok(mut body) = serde_json::from_str::<StreamingBody>(&message?.to_string()) {
            modify_notes(&mut body.body.body).await?;
            app_handle.emit_all("timeline", &body.body.body)?;
        }
    }

    Ok(())
}
