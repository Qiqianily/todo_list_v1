use crate::conf;
use crate::db::my_redis::RedisClient;
use anyhow::Context;
use sea_orm::DatabaseConnection;
use std::sync::OnceLock;

pub mod my_redis;
pub mod pgsql;

/// 全局 Postgres 数据库连接池实例
static GLOBAL_DATABASE_POOL: OnceLock<DatabaseConnection> = OnceLock::new();
/// 全局 Redis 连接池实例
static GLOBAL_REDIS_CLIENT: OnceLock<RedisClient> = OnceLock::new();

/// 获取全局的静态 Postgres 数据库连接池引用
pub fn get_global_database_pool() -> &'static DatabaseConnection {
    GLOBAL_DATABASE_POOL.get().expect("database pool lost")
}

/// 获取全局的静态 Redis 连接池引用
pub fn get_global_redis_client() -> &'static RedisClient {
    GLOBAL_REDIS_CLIENT
        .get()
        .expect("global redis client not set")
}

/// 初始化全局的静态数据库
pub async fn set_global_db() -> anyhow::Result<()> {
    // 读取 AppConfig
    let config = conf::get_app_config();

    // create redis pool
    let redis_pool = my_redis::create_redis_pool(
        config.redis().url(),
        config.redis().max_open(),
        config.redis().max_idle(),
        config.redis().timeout_sec(),
    )
    .await
    .context("failed to set redis pool")?;

    // 创建 redis client
    let redis_client = RedisClient::new(redis_pool);
    GLOBAL_REDIS_CLIENT
        .set(redis_client)
        .expect("Set GLOBAL_REDIS_CLIENT failed");

    // 创建数据库连接池
    let psql_pool = pgsql::init_postgres_database_pool(
        config.database().url(),
        config.database().min_connections(),
        config.database().max_connections(),
    )
    .await
    .context("failed to set psql pool")?;

    // 设置全局的数据库连接池
    GLOBAL_DATABASE_POOL
        .set(psql_pool)
        .expect("Set GLOBAL_DATABASE_POOL failed");
    // 返回 void
    Ok(())
}
