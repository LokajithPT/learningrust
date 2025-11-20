use tokio::net::{TcpListener};
use tokio::sync::{broadcast, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::error::Error;
use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;

struct ServerState {
    clients: HashMap<SocketAddr, String>,
}

type SharedState = Arc<Mutex<ServerState>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Chat server listening on 127.0.0.1:8080");

    let state = Arc::new(Mutex::new(ServerState {
        clients: HashMap::new(),
    }));

    let (tx, _rx) = broadcast::channel::<String>(16);

    loop {
        let (stream, addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let state = state.clone();

        tokio::spawn(async move {
            println!("Accepted connection from: {}", addr);
            let (reader, mut writer) = stream.into_split();
            let mut reader = BufReader::new(reader);
            let mut username = String::new();
            let mut line = String::new();

            writer.write_all(b"Please enter your username: ").await.unwrap();
            if reader.read_line(&mut username).await.is_err() {
                return;
            }
            let username = username.trim().to_string();
            state.lock().await.clients.insert(addr, username.clone());
            let join_msg = format!("{} has joined the chat.\n", username);
            tx.send(join_msg).unwrap();

            loop {
                line.clear();
                tokio::select! {
                    Ok(msg) = rx.recv() => {
                        if writer.write_all(msg.as_bytes()).await.is_err() {
                            break;
                        }
                    }

                    result = reader.read_line(&mut line) => {
                        match result {
                            Ok(0) => break,
                            Ok(_) => {
                                let full_msg = format!("{}: {}", username, line);
                                tx.send(full_msg).unwrap();
                            }
                            Err(_) => break,
                        }
                    }
                }
            }

            state.lock().await.clients.remove(&addr);
            let part_msg = format!("{} has left the chat.\n", username);
            tx.send(part_msg).unwrap();
            println!("Connection closed for: {}", addr);
        });
    }
}
