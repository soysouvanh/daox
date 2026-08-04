use sqlx::{Executor, sqlite::SqlitePoolOptions};
#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let sql = "CREATE TABLE test (id INTEGER); INSERT INTO test VALUES (1);";
    pool.execute(sql).await.unwrap();
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM test").fetch_one(&pool).await.unwrap();
    println!("count: {}", count.0);
}
