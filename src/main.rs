use chrono::Utc;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::PgExecutor;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let conn_str = std::env::var("DATABASE_URL").expect("database url");
    let pool = connect_with_conn_str(&conn_str, 4000).await;
    let conn = pool.acquire().await.expect("acquire connection");
    run(conn).await;

    println!("connected");
}

async fn run<T>(exec: T)
where
    for<'e> &'e mut T: PgExecutor<'e>,
    T: Send + Sync + 'static,
{
    let state = Arc::new(Mutex::new(State { exec }));
    let mut people = HashMap::new();
    people.insert("Bob", "bob@foo.com");
    people.insert("Alice", "alice@acme.inc");
    let threads: Vec<_> = people
        .into_iter()
        .map(|(key, value)| {
            let state = state.clone();
            tokio::spawn(async move {
                subscriptions(key.to_string(), value.to_string(), state).await;
            })
        })
        .collect();

    for t in threads {
        t.await.expect("thread panicked");
    }

    println!("done");
}

pub async fn subscriptions<T>(username: String, email: String, state: Arc<Mutex<State<T>>>)
where
    for<'e> &'e mut T: PgExecutor<'e>,
    T: Send + Sync,
{
    let mut guard = state.lock().await;
    let _ = sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, username, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        email,
        username,
        Utc::now()
    )
    .execute(&mut guard.exec)
    .await
    .expect("insert into subscriptions");
}

pub struct State<E> {
    pub exec: E,
}

pub async fn connect_with_conn_str(conn_str: &str, timeout: u64) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(timeout))
        .connect(conn_str)
        .await
        .expect("Postgres Pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test1() {
        let conn_str = std::env::var("DATABASE_URL").expect("database url");
        let pool = connect_with_conn_str(&conn_str, 4000).await;
        let tx = pool.begin().await.expect("begin transaction");
        run(tx).await;
        println!("done");
    }
}
