#![forbid(unsafe_code)]

use goose::prelude::*;

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------
#[tokio::main]
async fn main() -> Result<(), GooseError> {
    println!("Starting tms_min_loadtest");

    // Define scenarios like tms_loadtest does
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("baseline")
                .register_transaction(transaction!(baseline))
        )
        .register_scenario(
            scenario!("writeheavy")
                .register_transaction(transaction!(writeheavy))
        )
        .register_scenario(
            scenario!("readheavy")
                .register_transaction(transaction!(readheavy))
        )
        .execute()
        .await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Transactions
// ---------------------------------------------------------------------------
async fn baseline(user: &mut GooseUser) -> TransactionResult {
    let _ = user.get("/baseline").await?;
    Ok(())
}

async fn writeheavy(user: &mut GooseUser) -> TransactionResult {
    let _ = user.post("/writeheavy", "").await?;
    Ok(())
}

async fn readheavy(user: &mut GooseUser) -> TransactionResult {
    let _ = user.get("/readheavy").await?;
    Ok(())
}

