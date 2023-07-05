pub mod prelude {
    pub use super::time::{get_current_timestamp, get_current_duration};
}

pub mod time {
    use std::time::Duration;

    pub fn get_current_timestamp() -> u128 {
        get_current_duration().as_millis()
    }

    pub fn get_current_second() -> u64 {
        get_current_duration().as_secs()
    }

    pub fn get_current_duration() -> Duration {

        let start = std::time::SystemTime::now();
        start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")

    }
}

#[cfg(feature = "sqlx")]
pub mod sqlx {
    use sqlx::SqliteConnection;

    async fn update_database_version(conn: &mut SqliteConnection, table_name: &'static str, version: &'static str) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(r#"UPDATE "{}" SET "value" = ? WHERE "key" = 'version'"#, table_name))
            .bind(version)
            .execute(conn)
            .await
            .map(|_| ())
    }

    async fn insert_database_version(conn: &mut SqliteConnection, table_name: &'static str, version: &'static str) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(r#"INSERT INTO "{}" VALUES ("version", ?)"#, table_name))
            .bind(version)
            .execute(conn)
            .await
            .map(|_| ())
    }

    async fn check_database(conn: &mut SqliteConnection, table_name: &'static str) -> Result<bool, sqlx::Error> {
        sqlx::query_as(
            r#"SELECT COUNT(*) FROM "sqlite_master" WHERE "type" = 'table' AND "name" = 'meta';"#,
        )
            .bind(table_name)
            .fetch_one(conn)
            .await
            .map(|(count,): (i32,)| count > 0)
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
