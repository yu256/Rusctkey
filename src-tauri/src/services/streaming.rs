use std::pin::Pin;
use std::sync::{Arc, Mutex};

use futures::stream::SplitSink;
use futures::SinkExt;
use futures::{stream::SplitStream, StreamExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::Manager;
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};

use crate::services::Note;

use super::service::{TOKEN, URL};

struct WebSocketHandler {
    writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketHandler {
    async fn new() -> Self {
        let url: &str = &URL;
        let token: &str = &TOKEN;
        let stream_url = format!("wss://{}/streaming?i={}", url, token);

        let (stream, _) = connect_async(stream_url).await.expect("Failed to connect");

        let (writer, reader) = stream.split();

        WebSocketHandler { writer, reader }
    }
}

static WEBSOCKET_HANDLER: Lazy<Arc<Mutex<WebSocketHandler>>> = Lazy::new(|| {
    let websocket_handler = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { WebSocketHandler::new().await });
    Arc::new(Mutex::new(websocket_handler))
});

fn get_websocket_handler() -> Arc<Mutex<WebSocketHandler>> {
    Arc::clone(&*WEBSOCKET_HANDLER)
}

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
    let handler = get_websocket_handler();

    let write = &mut handler.lock().unwrap().writer;
    let reader = &mut handler.lock().unwrap().reader;
    let pinned_reader = Pin::new(reader);

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

    pinned_reader
        .for_each(|message| async {
            let message = message.unwrap().to_text().unwrap().to_string();

            let mut streaming_body: StreamingBody =
                match serde_json::from_str::<StreamingBody>(message.as_str()) {
                    Ok(deserialized) => deserialized,
                    Err(error) => {
                        eprintln!("Error: {}", error);
                        todo!()
                    }
                };

            super::note_modifier::modify_notes(&mut streaming_body.body.body).await;

            app_handle
                .emit_all("timeline", &streaming_body.body.body)
                .unwrap();
        })
        .await;
}
