use cucumber::World;
use futures::FutureExt;

use tdb::connect_with_conn_str;

mod state;
mod steps;

use steps::service::spawn_service;

// This runs before everything else, so you can setup things here.
#[tokio::main]
async fn main() {
    state::TestWorld::cucumber()
        .before(move |_feature, _rule, _scenario, world| {
            async {
                world.tx = Some(
                    world
                        .pool
                        .begin()
                        .await
                        .expect("Unable to begin transaction"),
                );
            }
            .boxed()
        })
        .after(move |_feature, _rule, _scenario, _event, world| {
            async {
                let tx = world.unwrap().tx.unwrap();
                tx.rollback().await.expect("Unable to rollback transaction");
            }
            .boxed()
        })
        .run("tests/features")
        .await;
}
