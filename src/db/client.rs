// Initializes and returns the PostgreSQL connection pool via sqlx
// All database queries will go through here

pub async fn connect(database_url: &str) -> sqlx::PgPool {
    // returns the postgresql conection pool, and if we don't get that we print that error message
    sqlx::PgPool::connect(database_url)
        .await
        .expect("failure to connect yo")
}
