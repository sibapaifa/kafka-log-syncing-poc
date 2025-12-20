// --- Batching Configuration ---
/// A safe limit for the payload size in bytes. OpenSearch's default is 100MB.
/// We'll use 10MB as a conservative threshold to stay well within the limits.
pub const MAX_PAYLOAD_SIZE_BYTES: usize = 10 * 1024 * 1024; // 10 MB

/// A secondary limit on the number of documents per batch to avoid overly complex requests.
pub const MAX_DOCS_PER_BATCH: usize = 10000;

pub const CLICKHOUSE_URL: &str = "http://localhost:8123";
pub const CLICKHOUSE_USER: &str = "default";
pub const CLICKHOUSE_PASSWORD: &str = "dev_password";
pub const OPENSEARCH_URL: &str = "https://search-aops-cerebro-dev-eyip4s2x3jo5yvguwsme7mfc7q.us-east-1.es.amazonaws.com/click-house-logs-3/_bulk";
pub const OPENSEARCH_USER: &str = "aopsdevadmin";
pub const OPENSEARCH_PASSWORD: &str = "Cerebro%2589";
