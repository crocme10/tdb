use sqlx::postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;
use chrono::Utc;
use sqlx::Executor;

#[tokio::main]
async fn main() {
    let conn_str = std::env::var("DATABASE_URL").expect("database url");
    let pool = connect_with_conn_str(&conn_str, 4000).await;
    let mut conn = pool.acquire().await.expect("acquire connection");
    subscriptions("Alice".to_string(), "Bob".to_string(), &mut conn).await;

    println!("connected");
}

pub async fn connect_with_conn_str(conn_str: &str, timeout: u64) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(timeout))
        .connect(conn_str)
        .await
        .expect("Postgres Pool")
}

pub async fn subscriptions<'c, E>(
    username: String,
    email: String,
    exec: E,
) 
    where
    E: Executor<'c, Database = sqlx::Postgres>,
{
    let _ = sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, username, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        email,
        username,
        Utc::now()
    )
    .execute(exec)
    .await
    .expect("insert into subscriptions");
}


