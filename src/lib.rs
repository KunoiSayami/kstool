pub mod prelude {
    pub use super::time::{get_current_timestamp, get_current_duration};
}

pub mod time {
    use std::time::Duration;

    pub fn get_current_timestamp() -> u128 {
        get_current_duration().as_millis()
    }
    pub fn get_current_duration() -> Duration {

        let start = std::time::SystemTime::now();
        start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")

    }
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn it_works() {
        get_current_timestamp();
    }
}
