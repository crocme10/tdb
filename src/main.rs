use chrono::Utc;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Executor;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let conn_str = std::env::var("DATABASE_URL").expect("database url");
    let pool = connect_with_conn_str(&conn_str, 4000).await;
    let mut conn = pool.acquire().await.expect("acquire connection");
    run(&mut conn).await;

    println!("connected");
}

async fn run<'c, E>(exec: E)
where
    E: Executor<'c, Database = sqlx::Postgres>,
{
    let state = Arc::new(Mutex::new(State { exec }));
    let mut people = HashMap::new();
    people.insert("Bob", "bob@foo.com");
    people.insert("Alice", "alice@acme.inc");
    let threads: Vec<_> = people
        .iter()
        .map(|(key, value)| {
            let state = state.clone();
            thread::spawn(move || async {
                subscriptions(key.to_string(), value.to_string(), state).await;
            })
        })
        .collect();

    for t in threads {
        t.join().expect("thread panicked");
    }

    println!("done");
}

pub async fn subscriptions<'c, E>(username: String, email: String, state: Arc<Mutex<State<E>>>)
where
    E: Executor<'c, Database = sqlx::Postgres>,
    for<'a> &'a E: Executor<'a, Database = sqlx::Postgres>
{
    // let exec = &state.lock().unwrap().exec;
    let _ = sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, username, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        email,
        username,
        Utc::now()
    )
    .execute(&state.lock().unwrap().exec)
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


