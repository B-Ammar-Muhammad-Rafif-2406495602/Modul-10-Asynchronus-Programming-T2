use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    
    ws_stream.send(Message::text("Welcome to chat! Type a message")).await?;

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From client {addr}: \"{text}\"");
                            // Broadcast with sender's address included
                            let formatted = format!("{addr}: {text}");
                            bcast_tx.send(formatted)?;
                        }
                    }
                    _ => break,
                }
            }
            msg = bcast_rx.recv() => {
                let text = msg?;
                ws_stream.send(Message::text(text)).await?;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = broadcast::channel(16);
    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    println!("Listening on port 8888");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            let ws_stream = ServerBuilder::new().accept(socket).await.unwrap();
            handle_connection(addr, ws_stream, bcast_tx).await.unwrap();
        });
    }
}