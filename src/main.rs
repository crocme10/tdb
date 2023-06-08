use tdb::run;
use tdb::connect_with_conn_str;

#[tokio::main]
async fn main() {
    let conn_str = std::env::var("DATABASE_URL").expect("database url");
    let pool = connect_with_conn_str(&conn_str, 4000).await;
    let conn = pool.acquire().await.expect("acquire connection");
    let _ = run(conn).await;

    println!("connected");
}


