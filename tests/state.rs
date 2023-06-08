use cucumber::World;
use sqlx::{Connection, PgConnection, PgPool, Postgres, Transaction};

use test_generic_axum_state::connect_with_conn_str;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TestWorld {
    pub pool: PgPool,
    pub tx: Option<Transaction<'static, Postgres>>,
}

impl TestWorld {
    pub async fn new() -> Self {

        let conn_str = std::env::var("DATABASE_URL").expect("database url");

        let pool = connect_with_conn_str(&conn_str, 4000)
            .await
            .unwrap_or_else(|_| panic!("Establishing a database connection with {conn_str}"));


        TestWorld {
            pool,
            tx: None,
        }
    }
}
