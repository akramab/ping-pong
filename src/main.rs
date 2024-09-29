use std::env;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(env::args()).unwrap();

    // Bind the listener to the address
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port)).await?;
    println!("Server is listening on port {}...", config.port);

    loop {
        // Accept incoming connections
        let (socket, addr) = listener.accept().await?;

        // Print requestor address
        println!("New connection from {}", addr);

        // Spawn a new task to handle the connection
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

struct Config {
    port: String,
}

async fn process(mut socket: TcpStream) {
    let mut buffer = [0; 1024];

    // Read data from the socket
    match socket.read(&mut buffer).await {
        Ok(n) if n == 0 => return, // connection closed
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Received request: {}", request);

            // Simulate ICMP reply (echo back the same message)
            let reply = format!("Echo: {}", request);

            // Write the reply back to the client
            if let Err(e) = socket.write_all(reply.as_bytes()).await {
                println!("Failed to send reply: {}", e);
            }
        }
        Err(e) => {
            println!("Failed to read from socket: {}", e);
        }
    }
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let port = match args.next() {
            Some(arg) => arg,
            None => env::var("PORT").unwrap_or("8080".to_string()),
        };

        Ok(Config { port })
    }
}
