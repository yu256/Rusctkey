use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::Manager;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::services::Note;

use super::service::{TOKEN, URL};

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

pub async fn streaming(app_handle: tauri::AppHandle) {
    let url: &str = &URL;
    let token: &str = &TOKEN;
    let target_url = format!("wss://{}/streaming?i={}", url, token);
    let parsed_url: reqwest::Url = target_url.parse().unwrap();

    let (stream, _) = connect_async(parsed_url).await.expect("Failed to connect");

    let (mut write, read) = stream.split();

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
        .await
        .unwrap();

    read.for_each(|message| async {
        let message = message.unwrap().to_text().unwrap().to_string();
        println!("{}", message);

        let streaming_body: StreamingBody =
            match serde_json::from_str::<StreamingBody>(message.as_str()) {
                Ok(deserialized) => deserialized,
                Err(error) => {
                    eprintln!("Error: {}", error);
                    todo!()
                }
            };

        app_handle
            .emit_all("timeline", &streaming_body.body.body)
            .unwrap();
    })
    .await;
}
