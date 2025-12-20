use crate::log::{Log};
use clickhouse::Client;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
pub async fn process_stream(
    stream: TcpStream,
    client: Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        // Read one line (one log message) from the TCP stream
        match reader.read_line(&mut line).await {
            Ok(0) => {
                // 0 bytes read means the connection was closed by the client
                println!("Client disconnected.");
                break;
            }
            Ok(_) => {
                // Attempt to deserialize the received line into our LogEntry struct
                let trimmed_line = line.trim();
                if trimmed_line.is_empty() {
                    line.clear();
                    continue;
                }
                println!("trimmedline{}", trimmed_line);
                if let Ok(log_variant) = serde_json::from_str::<Log>(trimmed_line) {
                    // 2. DISPATCH CALL (Exactly once!)
                    if let Err(e) = log_variant.handle(&client).await {
                        eprintln!("Failed to handle log: {}", e);
                    }
                }
                line.clear(); // Important: clear the buffer for the next line
            }
            Err(e) => {
                // An error occurred reading from the socket
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        }
    }
    Ok(())
}
