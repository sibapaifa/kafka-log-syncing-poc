use crate::models::LogEntry;

pub fn route_topic(count:i32) -> String {
    match count % 2 {
      1 => "new-topic-1".to_string(),
      _ =>"new-topic-2".to_string(),
    }
}
