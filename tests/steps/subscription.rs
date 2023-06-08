use cucumber::{then, when};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::state;

use tdb::{subscriptions, State};

#[when(regex = r#"the user subscribes with username "(\S*)" and email "(\S*)""#)]
async fn subscribes_full(world: &mut state::TestWorld, username: String, email: String) {
    let exec = world.tx.take().expect("take transaction");
    let state = Arc::new(Mutex::new(State { exec }));
    let state2 = state.clone();
    let handle = tokio::spawn(async move { subscriptions(username, email, state).await; });
    handle.await.expect("thread panicked");
    let tx = Arc::into_inner(state2).expect("try unwrap").into_inner().exec;
    world.tx = Some(tx);
}

#[then(regex = r#"the database stored the username "(\S+)" and the email "(\S+)""#)]
async fn query_database(world: &mut state::TestWorld, username: String, email: String) {
    let mut exec = world.tx.take().expect("take transaction");
    let saved = sqlx::query!(
        r#"SELECT email, username FROM subscriptions WHERE username = $1"#,
        username
    )
    .fetch_one(&mut exec)
    .await
    .expect("Failed to fetch saved subscription");
    assert_eq!(saved.email, email);
    assert_eq!(saved.username, username);
    world.tx = Some(exec);
}
