use crate::constant::{BROKER_ADDRESS, CLICKHOUSE_PASSWORD, CLICKHOUSE_URL, CLICKHOUSE_USER};
use crate::process::process_stream;
use tokio::net::TcpListener;
mod constant;
mod log;
mod process;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(BROKER_ADDRESS).await?;
    println!("Custom broker listening for JSON logs on 127.0.0.1:8080");

    let url = CLICKHOUSE_URL;
    let user = CLICKHOUSE_USER.to_string();
    let password = CLICKHOUSE_PASSWORD;

    let client = clickhouse::Client::default()
        .with_url(url)
        .with_user(user)
        .with_password(password);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Accepted new connection from: {}", addr);

        // Clone the client for the new task
        let client = client.clone();

        tokio::spawn(async move {
            if let Err(e) = process_stream(socket, client).await {
                eprintln!("Error processing stream from {}: {}", addr, e);
            }
        });
    }
}
