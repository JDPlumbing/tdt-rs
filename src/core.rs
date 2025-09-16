use chrono::{DateTime, TimeZone, Utc};

/// Count ticks between two datetimes in a chosen unit
pub fn count_ticks(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>, unit: &str) -> i64 {
    let start = start.unwrap_or_else(|| Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap());
    let end = end.unwrap_or_else(Utc::now);

    match unit {
        "days" => (end - start).num_days(),
        "hours" => (end - start).num_hours(),
        "minutes" => (end - start).num_minutes(),
        "seconds" => (end - start).num_seconds(),
        "milliseconds" => (end - start).num_milliseconds(),
        "microseconds" => (end - start).num_microseconds().unwrap_or(i64::MAX),
        "nanoseconds" => (end - start).num_nanoseconds().unwrap_or(i64::MAX),
        _ => panic!("Unsupported unit: {}", unit),
    }
}

/// Breakdown of elapsed time (approx, calendar aware only at days-level)
pub fn breakdown(start: DateTime<Utc>, end: DateTime<Utc>) -> (i64, i64, i64, i64, i64, i64) {
    let delta = end - start;
    let days = delta.num_days();

    let years = days / 365;
    let months = (days % 365) / 30;
    let days_left = (days % 365) % 30;

    let hours = delta.num_hours() % 24;
    let minutes = delta.num_minutes() % 60;
    let seconds = delta.num_seconds() % 60;

    (years, months, days_left, hours, minutes, seconds)
}

/// Pretty print breakdown like "27 years, 5 months, 18 days"
pub fn pretty_breakdown(start: DateTime<Utc>, end: Option<DateTime<Utc>>, max_units: usize) -> String {
    let end = end.unwrap_or_else(Utc::now);
    let (years, months, days, hours, minutes, seconds) = breakdown(start, end);

    let mut parts = vec![];
    if years > 0 {
        parts.push(format!("{} year{}", years, if years != 1 { "s" } else { "" }));
    }
    if months > 0 {
        parts.push(format!("{} month{}", months, if months != 1 { "s" } else { "" }));
    }
    if days > 0 {
        parts.push(format!("{} day{}", days, if days != 1 { "s" } else { "" }));
    }
    if hours > 0 {
        parts.push(format!("{} hour{}", hours, if hours != 1 { "s" } else { "" }));
    }
    if minutes > 0 {
        parts.push(format!("{} minute{}", minutes, if minutes != 1 { "s" } else { "" }));
    }
    if seconds > 0 && parts.is_empty() {
        parts.push(format!("{} second{}", seconds, if seconds != 1 { "s" } else { "" }));
    }

    let mut display = parts.into_iter().take(max_units).collect::<Vec<_>>();
    if display.is_empty() {
        display.push("0 seconds".to_string());
    }
    display.join(", ")
}
