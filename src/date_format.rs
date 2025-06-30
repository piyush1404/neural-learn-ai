
use chrono::{Local, DateTime, Utc};

pub fn get_local_and_utc_iso() -> (String, String) {
    let local_time: DateTime<Local> = Local::now();
    let utc_time: DateTime<Utc> = local_time.with_timezone(&Utc);

    let local_iso = local_time.to_rfc3339(); // Includes local timezone offset
    let utc_iso = utc_time.to_rfc3339();     // UTC, ends with 'Z'

    (local_iso, utc_iso)
}

pub fn format_date_mmddyyyy(date_str: &str) -> String {
    let dt_utc = DateTime::parse_from_rfc3339(date_str)
        .expect("Invalid datetime");

    // Convert from UTC to local timezone
    let dt_local = dt_utc.with_timezone(&Local);

    dt_local.format("%m/%d/%Y").to_string()
}
