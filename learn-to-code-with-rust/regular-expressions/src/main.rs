#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    category: String,
    endpoint: String,
    id: String,
}

fn main() {
    let server_logs = vec![
        "[2025-06-15 14:32:10] INFO: User 127.0.0.1 requested /api/orders/12345",
        "[2025-06-15 14:33:02] ERROR: Failed to fetch /api/users/9876",
        "[2025-06-15 14:34:55] DEBUG: User 10.0.0.1 requested /api/products/999",
        "!!!!",
        "[2025-06-15 14:35:01] INFO: User 192.168.1.5 requested /api/orders/54321",
        "[2025-06-15 14:35:45] ERROR: Failed to fetch /api/products/4567",
        "!!!!",
        "[2025-06-15 14:36:12] WARN: User 10.0.0.99 requested /api/users/101",
        "[2025-06-15 14:37:09] INFO: User 127.0.0.1 requested /api/categories/88",
        "[2025-06-15 14:37:59] WARN: Malformed URL detected",
        "[2025-06-15 14:38:42] INFO: User 172.16.0.10 requested /api/orders/11111",
        "[2025-06-15 14:39:00] DEBUG: Skipping health check ping",
        "[2025-06-15 14:40:20] ERROR: Failed to fetch /api/orders/00000",
        "!!!!",
    ];

    // for log in server_logs {
    //     if let Some(entry) = parse_log_entry(log) {
    //         println!("{:#?}", entry);
    //     }
    // }

    fn parse_log_entry(log: &str) -> Option<LogEntry> {
        let re = regex::Regex::new(r"\[(.*?)\] (.*?): .*?/api/([^/]+)/(\d+)").unwrap();
        if let Some(caps) = re.captures(log) {
            Some(LogEntry {
                timestamp: caps[1].to_string(),
                category: caps[2].to_string(),
                endpoint: caps[3].to_string(),
                id: caps[4].to_string(),
            })
        } else {
            None
        }
    }

    let entries = server_logs
        .into_iter()
        .filter_map(|log| parse_log_entry(log))
        .collect::<Vec<LogEntry>>();

    println!("{:#?}", entries);
}
