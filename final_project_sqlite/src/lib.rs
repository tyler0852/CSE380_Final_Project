use poem::handler;
use poem::web::Data;
use sqlx::sqlite::SqlitePool;
use rand::Rng;
use tokio::time::sleep;
use std::time::Duration;

////////////////////////////
///// BASELINE ENDPOINT ////
////////////////////////////
#[handler]
pub async fn baseline() -> String {
    "Server is running".to_string()
}

////////////////////////////////
///// WRITE HEAVY ENDPOINT /////
////////////////////////////////
#[handler]
pub async fn writeheavy(pool: Data<&SqlitePool>) -> String {
    let tables = ["table_a", "table_b", "table_c"];

    // Ensure tables exist
    for table in &tables {
        let create_query = format!(
            "CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY AUTOINCREMENT, value TEXT)",
            table
        );
        sqlx::query(&create_query)
            .execute(pool.0)
            .await
            .unwrap();
    }

    for table in &tables {
        let mut tx = pool.0.begin().await.unwrap();

        // INSERT #1
        let insert_query = format!("INSERT INTO {} (value) VALUES (?)", table);
        sqlx::query(&insert_query)
            .bind(format!("data {}", rand::thread_rng().gen_range(0..100000)))
            .execute(&mut *tx)
            .await
            .unwrap();

        // READ inside txn
        let count_query = format!("SELECT COUNT(*) FROM {}", table);
        sqlx::query(&count_query)
            .fetch_one(&mut *tx)
            .await
            .unwrap();

        // INSERT #2
        sqlx::query(&insert_query)
            .bind(format!("extra {}", rand::thread_rng().gen_range(0..100000)))
            .execute(&mut *tx)
            .await
            .unwrap();

        // Skip file read during tests
        #[cfg(not(test))]
        let _ = std::fs::read("/Users/tyler/tacc_research/tms_min/tms_min.db");

        tx.commit().await.unwrap();

        // Skip random delay during tests
        #[cfg(not(test))]
        {
            let delay = rand::thread_rng().gen_range(10..50);
            sleep(Duration::from_millis(delay)).await;
        }
    }

    "Write-heavy operation complete".to_string()
}

//////////////////////
///// READ HEAVY /////
//////////////////////
#[handler]
pub async fn readheavy(pool: Data<&SqlitePool>) -> String {
    let tables = ["table_a", "table_b", "table_c"];
    let mut total_rows = 0;

    // Ensure tables exist (needed for tests)
    for table in &tables {
        let create_query = format!(
            "CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY AUTOINCREMENT, value TEXT)",
            table
        );
        sqlx::query(&create_query)
            .execute(pool.0)
            .await
            .unwrap();
    }

    for table in &tables {

        // Skip delay during tests
        #[cfg(not(test))]
        {
            let delay = rand::thread_rng().gen_range(5..20);
            sleep(Duration::from_millis(delay)).await;
        }

        // Correct SELECT syntax
        let limit = rand::thread_rng().gen_range(5..15);
        let query = format!("SELECT * FROM {} LIMIT {}", table, limit);
        let rows = sqlx::query(&query)
            .fetch_all(pool.0)
            .await
            .unwrap();

        total_rows += rows.len();

        // COUNT query
        let count_query = format!("SELECT COUNT(*) FROM {}", table);
        let _ = sqlx::query(&count_query)
            .fetch_one(pool.0)
            .await
            .unwrap();

        // Skip file read during tests
        #[cfg(not(test))]
        let _ = std::fs::read("/Users/tyler/tacc_research/tms_min/tms_min.db");
    }

    format!("Read-heavy operation complete. Total rows read: {}", total_rows)
}
