use crate::middlewares::auth::identity::Identity;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Principal {
    pub id: i64,            // id 用户id
    pub name: String,       // name 昵称
    pub level: i32,         // level 等级
    pub identity: Identity, // identity 身份信息
}

/// 手动实现 Debug trait
impl std::fmt::Debug for Principal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Principal")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("level", &self.level)
            .field("identity", &self.identity.as_str())
            .finish()
    }
}
