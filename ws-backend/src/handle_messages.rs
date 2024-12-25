use super::events::player_join::handle_player_join;
use super::gamestate::game::Game;
use crate::{
    events::{player_leave::handle_player_leave, player_move::handle_player_move},
    gamestate::map::Map,
};
use futures::{lock::Mutex, stream::SplitSink, SinkExt};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};

async fn send_messages(
    msg: String,
    client: &Arc<
        Mutex<Vec<Arc<Mutex<SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>>>>>,
    >,
) {
    let clients = client.lock().await;
    for client in clients.iter() {
        let c = Arc::clone(&client);
        let _ = c.lock().await.send(Message::Text(msg.clone())).await;
    }
}

pub async fn handle_messages(
    msg: String,
    client: Arc<Mutex<Vec<Arc<Mutex<SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>>>>>>,
    game: Arc<Mutex<Game>>,
    map: Arc<RwLock<Map>>,
) {
    // Get event_type from message
    let mut event_type = String::from("unknown");
    if let Some(e) = msg.split('"').nth(3) {
        event_type = e.to_string();
    }

    match event_type.as_str() {
        "player_join" => {
            let reply: String = handle_player_join(msg, game, map).await;
            send_messages(reply, &client).await;
        }
        "player_leave" => {
            let reply: String = handle_player_leave(msg, game, map).await;
            send_messages(reply, &client).await;
        }
        "player_move" => {
            let reply = handle_player_move(msg, &game, &map).await;
            send_messages(reply.0, &client).await;
            if reply.1 {
                let out_reply = handle_player_leave(reply.2, game, map).await;
                send_messages(out_reply, &client).await;
            }
        }
        _ => {
            println!("Unknown event type: {}", event_type);
        }
    }
}
