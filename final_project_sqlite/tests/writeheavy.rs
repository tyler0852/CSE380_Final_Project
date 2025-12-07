use sqlx::sqlite::SqlitePool;
use poem::web::Data;
use std::sync::Arc;

// Import the actual handler function from your lib.rs
use tms_min::lib::writeheavy;

#[tokio::test]
async fn test_writeheavy_runs() {
    // Shared in-memory SQLite DB
    let pool = Arc::new(
        SqlitePool::connect("sqlite:file:memdb1?mode=memory&cache=shared")
            .await
            .unwrap(),
    );

    // Directly call handler
    let resp = writeheavy(Data(&*pool)).await;

    assert_eq!(resp, "Write-heavy operation complete");
}
