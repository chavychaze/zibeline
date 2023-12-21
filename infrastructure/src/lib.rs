use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::sql_query;

    #[test]
    fn test_connection_established() {
        let mut connection = establish_connection();
        let query_result = sql_query("SELECT 1").execute(&mut connection);

        assert!(query_result.is_ok());
    }
}
