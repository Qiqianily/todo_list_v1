/// redis 连接相关配置
#[derive(Debug, serde::Deserialize)]
pub struct RedisConfig {
    url: String,      // redis 连接字符串
    max_open: u64,    // 最大打开连接数
    max_idle: u64,    // 最大空闲连接数
    timeout_sec: u64, // 超时时间（秒）
}
/// 获取 redis 的配置信息
impl RedisConfig {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn max_open(&self) -> u64 {
        self.max_open
    }
    pub fn max_idle(&self) -> u64 {
        self.max_idle
    }
    pub fn timeout_sec(&self) -> u64 {
        self.timeout_sec
    }
}
