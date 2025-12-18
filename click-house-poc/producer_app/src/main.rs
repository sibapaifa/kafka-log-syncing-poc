use crate::constant::BROKER_ADDRESS;
use crate::log::LogEntry;
use chrono::Utc;
use std::io::Write;
use std::net::TcpStream;
mod log;
mod constant;
fn main() {
    let broker_address = BROKER_ADDRESS;
    println!("Attempting to connect to broker at {}", broker_address);

    match TcpStream::connect(broker_address) {
        Ok(mut stream) => {
            println!("Successfully connected to broker.");

            for i in 0..10 {
                let log = LogEntry::new(
                    Utc::now(),
                    "INFO".to_string(),
                    format!("This is structured log message #{}", i),
                );

                let json_log = match serde_json::to_string(&log) {
                    Ok(json) => json,
                    Err(e) => {
                        eprintln!("Failed to serialize log to JSON: {}", e);
                        continue;
                    }
                };

                if let Err(e) = writeln!(stream, "{}", json_log) {
                    eprintln!("Failed to send log to broker: {}. Exiting.", e);
                    break;
                }

                println!("Sent log: {}", json_log);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to broker: {}", e);
        }
    }
}
