use chrono::{DateTime, Utc};

pub fn format_date_mmddyyyy(date_str: &str) -> String {
    let dt = DateTime::parse_from_rfc3339(date_str)
        .expect("Invalid datetime")
        .with_timezone(&Utc);

    dt.format("%m/%d/%Y").to_string()
}
