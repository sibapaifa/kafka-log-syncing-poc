use std::net::TcpStream;
use std::io::Write;
use serde::Serialize;

pub fn send_to_broker<T:Serialize>(log:&T,stream:&mut TcpStream) {
    let json_log = match serde_json::to_string(&log) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to serialize log to JSON: {}", e);
            return;
        }
    };

    if let Err(e) = writeln!(stream, "{}", json_log) {
        eprintln!("Failed to send log to broker: {}. Exiting.", e);
        return;
    }

    println!("Sent log: {}", json_log);
    std::thread::sleep(std::time::Duration::from_secs(1));
}
