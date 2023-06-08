use cucumber::World;
use sqlx::{PgPool, Postgres, Transaction};

use tdb::connect_with_conn_str;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TestWorld {
    pub pool: PgPool,
    pub tx: Option<Transaction<'static, Postgres>>,
}

impl TestWorld {
    pub async fn new() -> Self {
        let conn_str = std::env::var("DATABASE_URL").expect("database url");

        let pool = connect_with_conn_str(&conn_str, 4000).await;

        TestWorld { pool, tx: None }
    }
}
