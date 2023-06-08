use cucumber::given;
use std::net::TcpListener;
use sqlx::Transaction;

use zero2prod::{server};

use crate::state;

/// This function should mark the beginning of its test.
/// We have to start a new transaction.
/// Spawn
#[given("the service has been started")]
async fn start_service(world: &mut state::TestWorld) {
    let tx = world.pool.begin().await.expect("begin transaction");
    world.tx = Some(tx);
    spawn_service(&world.listener, tx).await
}
pub async fn spawn_service<'c>(listener: &TcpListener, transaction: Transaction<'c, sqlx::Postgres>) {
    let _ = tokio::spawn(server::run(listener, conn));
}
