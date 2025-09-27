use chrono::{format, prelude::*};

fn main() {
    let event_data = vec![
        (
            "2025**04**19 !! 16:00:00 -04:00",
            "Started Rust study session",
        ),
        ("2025**04**20 !! 08:05:30 -04:00", "Made breakfast"),
        ("ERR", "ERR"),
        ("2025**04**22 !! 22:10:45 -04:00", "Went to bed"),
        ("ERR", "ERR"),
        ("2025**04**25 !! 09:00:03 -04:00", "Resumed Rust study"),
    ];

    let format = "%Y**%m**%d !! %H:%M:%S %:z";

    // Filter out invalid events
    let valid_events: Vec<(&str, &str)> = event_data
        .into_iter()
        .filter(|(date_str, msg)| *date_str != "ERR" && *msg != "ERR")
        .collect();

    let mut previous_event_time: Option<DateTime<Utc>> = None;

    for (date_str, msg) in valid_events {
        // Parse the datetime string into a DateTime object
        let event_time = DateTime::parse_from_str(&date_str, format)
            .expect("Failed to parse date")
            .with_timezone(&Utc);

        // Print event time and message
        println!("Event time: {}", event_time.format("%Y-%m-%d %H:%M:%S"));
        println!("Event message: {}", msg);

        // If there's a previous event, calculate and print the time gap
        if let Some(prev_time) = previous_event_time {
            let duration = event_time.signed_duration_since(prev_time);
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;
            let seconds = duration.num_seconds() % 60;
            println!(
                "Time since previous event: {}h {}m {}s",
                hours, minutes, seconds
            );
        }

        // Update the previous event time
        previous_event_time = Some(event_time);
        println!(); // Print a newline for better readability
    }
}
