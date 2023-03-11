pub mod prelude {
    pub use super::time::get_current_timestamp;
}

pub mod time {
    pub fn get_current_timestamp() -> u128 {
        let start = std::time::SystemTime::now();
        let since_the_epoch = start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_millis()
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
