/// 数据库连接池相关配置
#[derive(Debug, serde::Deserialize)]
pub struct DbConfig {
    url: String,          // 数据库连接字符串
    min_connections: u32, // 最小连接数
    max_connections: u32, // 最大连接数
}

/// 获取数据库连接的相关配置
impl DbConfig {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn min_connections(&self) -> u32 {
        self.min_connections
    }
    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }
}
