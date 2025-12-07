use poem::{get, post, handler, listener::TcpListener, web::Data, Route, Server, EndpointExt};
use sqlx::sqlite::SqlitePool;
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

mod db_init;
use db_init::init_db;

mod lib;

use lib::baseline;
use lib::readheavy;
use lib::writeheavy;



////////////////
///// MAIN /////
////////////////
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = init_db().await;

    let app = Route::new()
        .at("/baseline", get(baseline))
        .at("/writeheavy", post(writeheavy))
        .at("/readheavy", get(readheavy))
        .data(pool);

    println!("Server running at http://localhost:3000");
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
