use crate::gamestate::{game::Game, map::Map};
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Debug)]
struct ClientDataOut {
    id: String,
}

#[derive(Serialize, Debug)]
struct ClientMessageOut {
    event_type: String,
    data: ClientDataOut,
}

#[derive(Deserialize, Debug)]
struct ClientData {
    id: String,
    ox: i32,
    oy: i32,
    nx: i32,
    ny: i32,
}

#[derive(Deserialize, Debug)]
struct ClientMessage {
    event_type: String,
    data: ClientData,
}

#[derive(Debug, Serialize)]
struct ServerData {
    id: String,
    x: i32,
    y: i32,
    weapon: bool,
    map: Vec<Vec<u8>>,
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

pub async fn handle_player_move(
    msg: String,
    game: &Arc<Mutex<Game>>,
    map: &Arc<RwLock<Map>>,
) -> (String, bool, String) {
    let client_message: ClientMessage = serde_json::from_str(&msg).unwrap();
    let mut g = game.lock().await;
    match g
        .update_player(
            client_message.data.id.clone(),
            client_message.data.ox.clone(),
            client_message.data.oy.clone(),
            client_message.data.nx.clone(),
            client_message.data.ny.clone(),
            &map,
        )
        .await
    {
        Ok(m) => {
            let server_message = ServerMessage {
                event_type: client_message.event_type.to_string(),
                data: ServerData {
                    id: client_message.data.id,
                    weapon: m.1,
                    x: client_message.data.nx,
                    y: client_message.data.ny,
                    map: m.0,
                },
            };

            if m.2 .0 {
                let mm = m.2 .1.unwrap_or("".to_string());
                let out_reply = ClientMessageOut {
                    event_type: "player_leave".to_string(),
                    data: ClientDataOut { id: mm.clone() },
                };
                for p in g.players.lock().await.iter_mut() {
                    if p.id == mm {
                        p.die = true;
                        break;
                    }
                }
                return (
                    serde_json::to_string(&server_message).unwrap(),
                    true,
                    serde_json::to_string(&out_reply).unwrap(),
                );
            }

            (
                serde_json::to_string(&server_message).unwrap(),
                false,
                "".to_string(),
            )
        }
        Err(err) => {
            let server_message = ServerReject {
                event_type: "reject".to_string(),
                error: err,
            };

            (
                serde_json::to_string(&server_message).unwrap(),
                false,
                "".to_string(),
            )
        }
    }
}
