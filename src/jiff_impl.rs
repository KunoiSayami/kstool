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

#[cfg(test)]
mod test {
    #[cfg(feature = "jiff-tz-full")]
    #[test]
    fn test_jiff_tz_parse() {
        use jiff::{civil::DateTime, Timestamp};

        let t: DateTime = "2026-01-02 21:09:47".parse().unwrap();

        t.in_tz("UTC").unwrap();

        let t: Timestamp = "2021-04-22T00:12:18Z".parse().unwrap();

        t.in_tz("Asia/Taipei").unwrap();
    }
}
