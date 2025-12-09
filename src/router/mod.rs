use crate::response::ApiResult;
use crate::response::errors::ApiError;
use crate::state::app_state::AppState;

pub mod login;
pub mod user;
pub mod version;
/// combine all the routes into one router
pub fn merge_router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/get/current", version::get_version_router())
        .nest("/auth", login::create_user_login_route())
        .nest("/user", user::create_users_router())
        .fallback(async || -> ApiResult<()> {
            // 路径找不到
            tracing::warn!("Not Found");
            Err(ApiError::NotFound)
        })
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        }) // 方法不允许
}
