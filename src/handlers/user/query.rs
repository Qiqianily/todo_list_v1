use crate::common::valid::ValidPath;
use crate::entities::prelude::Users;
use crate::entities::users::Model;
use crate::handlers::common::model::QueryUserByIdParam;
use crate::response::ApiResult;
use crate::response::errors::ApiError;
use crate::response::resp::ApiResponse;
use crate::state::app_state::AppState;
use axum::debug_handler;
use axum::extract::State;
use sea_orm::EntityTrait;

/// 按 id 来查询
#[debug_handler]
pub async fn query_user_info_by_id_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(params): ValidPath<QueryUserByIdParam>,
) -> ApiResult<ApiResponse<Model>> {
    // 根据 ID 查询用户信息
    let user_model: Model = Users::find_by_id(params.id)
        .one(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("你要查的用户信息暂时没查询到！")))?;
    // 返回查询结果
    Ok(ApiResponse::ok("success", Some(user_model)))
}
