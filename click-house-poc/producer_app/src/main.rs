use crate::constant::BROKER_ADDRESS;
use crate::helper::send_to_broker;
use crate::log::{InfoLog, Log, LogEntry};
use chrono::Utc;
use std::net::TcpStream;
mod constant;
mod log;
mod helper;
fn main() {
    let broker_address = BROKER_ADDRESS;
    println!("Attempting to connect to broker at {}", broker_address);

    match TcpStream::connect(broker_address) {
        Ok(mut stream) => {
            println!("Successfully connected to broker.");

            for i in 0..2 {
                let log_1 = Log::Entry(LogEntry::new(
                    Utc::now(),
                    "Log".to_string(),
                    format!("This is structured log message #{}", i),
                ));
                send_to_broker(&log_1,&mut stream);

                let log_2 = Log::Info(InfoLog::new(
                    Utc::now(),
                    "INFO".to_string(),
                    format!("This is action log message #{}", i),
                ));
                send_to_broker(&log_2,&mut stream);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to broker: {}", e);
        }
    }
}
