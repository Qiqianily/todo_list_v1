use crate::handlers::user::login::{get_user_info_handler, user_login_handler};
use crate::middlewares::auth::auth_layer::get_auth_layer;
use crate::state::app_state::AppState;

/// 创建用户相关的根路由
pub fn create_user_login_route() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/user/info", axum::routing::get(get_user_info_handler))
        .route_layer(get_auth_layer())
        .route("/login", axum::routing::post(user_login_handler))
}
