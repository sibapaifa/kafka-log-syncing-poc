use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use clickhouse::Client;
use clickhouse::insert::Insert;
use crate::log::LogEntry;
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
                match serde_json::from_str::<LogEntry>(trimmed_line) {
                    Ok(log_entry) => {
                        // Successfully parsed JSON, now insert into ClickHouse
                        println!("Received log: {:?}", log_entry);
                        let mut inserter: Insert<LogEntry> = client.insert("logs").await?;
                        inserter.write(&log_entry).await?;
                        inserter.end().await?;
                        println!("Successfully inserted log into ClickHouse.");
                    }
                    Err(e) => {
                        // The received data was not valid JSON for our struct
                        eprintln!("Failed to parse JSON: {}. Data: '{}'", e, trimmed_line);
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
