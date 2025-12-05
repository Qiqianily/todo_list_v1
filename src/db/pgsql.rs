use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};

/// 创建 Mysql 连接池
///
/// # 参数
/// - conn_url: Mysql 连接字符串 (例如: "mysql://root:pwd@localhost/yx_healthy")
/// - min_conn: 连接池最小连接数
/// - max_conn: 连接池最大连接数
pub async fn init_postgres_database_pool(
    conn_url: &str,
    min_conn: u32,
    max_conn: u32,
) -> anyhow::Result<DatabaseConnection> {
    //     连接选项配置
    let mut options = ConnectOptions::new(conn_url);
    options
        .min_connections(min_conn)
        .max_connections(max_conn)
        .connect_timeout(std::time::Duration::from_secs(10))
        .acquire_timeout(std::time::Duration::from_secs(6))
        .idle_timeout(std::time::Duration::from_secs(10))
        .max_lifetime(std::time::Duration::from_secs(20))
        .sqlx_logging(false)
        .set_schema_search_path("public"); // 在 Postgres 中，决定数据库查询默认使用的模式（schema）。
    let db = Database::connect(options).await?;
    // 测试数据库连接
    db.ping().await?;
    // ✅ 使用 sea-orm 的 execute_unprepared 设置时区（例如设置为 'Asia/Shanghai'）
    // db.execute_unprepared("SET time_zone = '+08:00'").await?;
    // 如果是连接池，就这样设置
    // let db = Database::connect_with(
    //     options.after_connect(|conn| {
    //         Box::pin(async move {
    //             conn.execute_unprepared("SET time_zone = '+08:00'").await?;
    //             Ok(())
    //         })
    //     })
    // ).await?;
    log_postgres_database_version(&db).await?;
    tracing::info!("✅ Postgres database pool initialized successfully.");
    Ok(db)
}

/// 输出当前数据库版本信息
///
/// # 参数
/// - db ： 数据库连接引用
///
/// # 返回值 无
async fn log_postgres_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version_result = db
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            String::from("SELECT version()"),
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("❌ Failed to get postgres database version"))?;

    tracing::info!(
        "Database version: {}",
        version_result.try_get_by_index::<String>(0)?
    );

    Ok(())
}
