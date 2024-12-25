use futures::{lock::Mutex, stream::SplitSink, StreamExt};
use gamestate::{game::Game, map::Map};
use handle_messages::handle_messages;
use log::{error, info, warn};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};

mod config;
mod events;
mod gamestate;
mod handle_messages;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:3001".parse().expect("Invalid address");

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    let clients = Arc::new(Mutex::new(Vec::new()));

    info!("Listening on: {}", addr);
    let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game::new()));
    let map: Arc<RwLock<Map>> = Arc::new(RwLock::new(Map::new()));

    while let Ok((stream, _)) = listener.accept().await {
        // Spawn a new task for each connection
        let client = Arc::clone(&clients);
        let game = Arc::clone(&game);
        let map = Arc::clone(&map);
        tokio::spawn(handle_connection(stream, client, game, map));
    }
}

async fn handle_connection(
    stream: TcpStream,
    client: Arc<Mutex<Vec<Arc<Mutex<SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>>>>>>,
    game: Arc<Mutex<Game>>,
    map: Arc<RwLock<Map>>,
) {
    // Accept the WebSocket connection
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    // Split the WebSocket stream into a sender and receiver
    let (sender, mut receiver) = ws_stream.split();
    let sender = Arc::new(Mutex::new(sender));

    // Add the sender to the list of connected clients
    {
        let mut clients = client.lock().await;
        clients.push(Arc::clone(&sender));
    }

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                handle_messages(
                    text,
                    Arc::clone(&client),
                    Arc::clone(&game),
                    Arc::clone(&map),
                )
                .await;
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Ok(_) => {
                warn!("Unexpected message type");
            }
            Err(e) => {
                error!("Error processing message: {}", e);
                break;
            }
        }
    }

    // Remove the sender from the list of connected clients
    {
        println!("Removing sender");
        let mut clients_guard = client.lock().await;
        clients_guard.retain(|client| !Arc::ptr_eq(client, &sender));
    }
}
