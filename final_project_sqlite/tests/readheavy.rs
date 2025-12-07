use poem::test::TestClient;
use poem::EndpointExt;
use sqlx::sqlite::SqlitePool;
use tms_min::readheavy;
use std::sync::Arc;

#[tokio::test]
async fn test_readheavy_runs() {
    let pool = Arc::new(
        SqlitePool::connect("sqlite:file:memdb1?mode=memory&cache=shared")
            .await
            .unwrap(),
    );

    let cli = TestClient::new(readheavy.data(pool.clone()));

    let resp = cli.get("/").send().await;

    resp.assert_status_is_ok();
}
