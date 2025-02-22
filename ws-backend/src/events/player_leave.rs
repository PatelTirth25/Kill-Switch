use crate::gamestate::{game::Game, map::Map};
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Deserialize, Debug)]
struct ClientData {
    id: String,
}

#[derive(Deserialize, Debug)]
struct ClientMessage {
    event_type: String,
    data: ClientData,
}

#[derive(Debug, Serialize)]
struct ServerData {
    id: String,
}

#[derive(Debug, Serialize)]
struct ServerMessage {
    event_type: String,
    data: ServerData,
}

#[derive(Debug, Serialize)]
struct ServerReject {
    event_type: String,
    error: String,
}

pub async fn handle_player_leave(
    msg: String,
    game: Arc<Mutex<Game>>,
    map: Arc<RwLock<Map>>,
) -> String {
    let client_message: ClientMessage = serde_json::from_str(&msg).unwrap();
    match game
        .lock()
        .await
        .remove_player(client_message.data.id.clone(), &map)
        .await
    {
        Ok(_) => {
            let server_message = ServerMessage {
                event_type: client_message.event_type.to_string(),
                data: ServerData {
                    id: client_message.data.id,
                },
            };

            serde_json::to_string(&server_message).unwrap()
        }
        Err(err) => {
            let server_message = ServerReject {
                event_type: "reject".to_string(),
                error: err,
            };

            serde_json::to_string(&server_message).unwrap()
        }
    }
}
