use crate::handlers::user::query::query_user_info_by_id_handler;
use crate::middlewares::auth::auth_layer::get_auth_layer;
use crate::state::app_state::AppState;

/// 创建用户相关的路由，专门用来管理与用户相关的操作
pub fn create_users_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/query/info/id/{id}",
            axum::routing::get(query_user_info_by_id_handler),
        )
        .route_layer(get_auth_layer())
}
