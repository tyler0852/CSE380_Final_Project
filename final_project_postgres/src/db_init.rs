use sqlx::postgres::PgPool;

pub async fn init_db() -> PgPool {
    // Connection string for Postgres running on localhost
    let database_url = "postgres://postgres:password@localhost:5432/postgres";

    // Create the connection pool
    let pool = PgPool::connect(database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Create the table if it doesn't already exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS test (
            id SERIAL PRIMARY KEY,
            value TEXT
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create test table");

    pool
}
