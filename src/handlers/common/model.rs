/// 定义查询用户的 id 参数
#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct QueryUserByIdParam {
    #[validate(range(min = 1, message = "查询用户的 id 必须大于 0"))]
    pub id: i32,
}
