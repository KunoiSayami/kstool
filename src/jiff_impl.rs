pub fn timestamp_fmt(timestamp: i64, fmt: &str) -> String {
    zone_fmt(
        jiff::Timestamp::from_second(timestamp)
            .expect("Invalid timestamp")
            .to_zoned(jiff::tz::TimeZone::system()),
        fmt,
    )
}

pub fn timestamp_to_string(timestamp: i64) -> String {
    timestamp_fmt(timestamp, "%Y-%m-%d %H:%M:%S")
}

pub fn current_time_string() -> String {
    zone_fmt(jiff::Zoned::now(), "%Y-%m-%d %H:%M:%S")
}

pub fn zone_fmt(zone: jiff::Zoned, fmt: &str) -> String {
    zone.strftime(fmt).to_string()
}

pub fn current_time_fmt(fmt: &str) -> String {
    zone_fmt(jiff::Zoned::now(), fmt)
}
