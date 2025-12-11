use hostname::get;

pub fn route_topic(count: i32) -> String {
    match count % 2 {
        1 => "kafka-test-1".to_string(),
        _ => "kafka-test-2".to_string(),
    }
}

pub fn get_hostname() -> String {
    get()
        .unwrap_or_default()
        .into_string()
        .unwrap_or_else(|_| "unknown".into())
}

pub fn formatted_timestamp() -> String {
    chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string()
}
