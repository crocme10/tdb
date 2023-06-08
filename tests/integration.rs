use cucumber::World;
use futures::FutureExt;

mod state;
mod steps;

// This runs before everything else, so you can setup things here.
#[tokio::main]
async fn main() {
    state::TestWorld::cucumber()
        .before(move |_feature, _rule, _scenario, world| {
            async {
                let tx = world.pool.begin().await.expect("Unable to begin new transaction");
                world.tx = Some(tx);
            }
            .boxed()
        })
        .after(move |_feature, _rule, _scenario, _event, world| {
            async {
                let tx = world.expect("world").tx.take().expect("take transaction");
                tx.rollback().await.expect("Unable to rollback transaction");
            }
            .boxed()
        })
        .run("tests/features")
        .await;
}
