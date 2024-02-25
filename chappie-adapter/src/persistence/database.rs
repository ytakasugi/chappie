use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<PgPool>);

impl Db {
    pub async fn new() -> Db {
        dotenv().ok();
        // 設定ファイルからデータベースURLを取得
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE URL MUST BE SET.");
        // 設定ファイルから最大接続数を取得
        let env_connection_pool =
            std::env::var("CONNECTION_POOL").expect("CONNECTION POOL MUST BE SET.");
        // `sqlx::pool::PoolOptions::max_connections`の引数は、u32なので型変換
        let max_connection_pool = env_connection_pool.parse::<u32>().unwrap();

        let pool = PgPoolOptions::new()
            .max_connections(max_connection_pool)
            .connect(&database_url)
            .await
            .unwrap_or_else(|_| panic!("Failed create connection pool."));

        Db(Arc::new(pool))
    }
}
