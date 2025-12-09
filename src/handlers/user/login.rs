use crate::common::valid::ValidJson;
use crate::entities::prelude::Users;
use crate::entities::{users, users::Model};
use crate::middlewares::auth::identity::Identity;
use crate::middlewares::auth::identity::RoleEnum;
use crate::middlewares::auth::jwt::get_default_jwt;
use crate::middlewares::auth::principal::Principal;
use crate::response::errors::ApiError;
use crate::response::{ApiResult, resp::ApiResponse};
use crate::state::app_state::AppState;
use crate::utils::crypto::verify_password;
use crate::utils::timezone::get_local_datetime_with_timezone;
use axum::Extension;
use axum::debug_handler;
use axum::extract::ConnectInfo;
use axum::extract::State;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use std::net::SocketAddr;

/// 用户登陆时的参数
#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct UserLoginParam {
    #[validate(length(min = 1, max = 255, message = "用户名长度必须在 1 到 255 之间"))]
    pub username: String,
    #[validate(length(min = 6, max = 12, message = "密码长度必须在 6 到 12 之间"))]
    pub password: String,
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    access_token: String,
}
/// 通过关键字，模糊查询
#[debug_handler]
#[tracing::instrument(name = "login", skip_all,fields(username = %params.username,ip = %addr.ip()))]
pub async fn user_login_handler(
    State(AppState {
        db_pool,
        redis_client,
        ..
    }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<UserLoginParam>,
) -> ApiResult<ApiResponse<LoginResult>> {
    tracing::info!("开始处理IP地址为 {} 的登陆逻辑...", addr.ip());
    let user_model: Model = Users::find()
        .filter(users::Column::Username.eq(&params.username))
        .one(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("帐号或密码不正确！")))?;
    if !user_model.is_active.unwrap() {
        return Err(ApiError::Biz(String::from(
            "此帐号已被限制登陆，请联系管理员",
        )));
    }
    // 比对密码是否正确，密码不对直接返回。
    if !verify_password(&params.password, &user_model.password_hash)? {
        return Err(ApiError::Biz(String::from("帐号或密码不正确！")));
    }
    let user_display_name = user_model.display_name.as_ref().unwrap().clone();
    // 新建一个 principal 结构体
    let principal = Principal {
        id: user_model.id as i64,
        name: user_display_name,
        level: 1,
        identity: Identity::role_to_identity(RoleEnum::User),
    };
    // 生成 access_token
    let access_token = get_default_jwt().encode(principal)?;
    // 存到 redis 中 yx_todo_list_1_user_name
    let redis_key = format!("yx_todo_list_{}_{}", user_model.id, user_model.username);
    let _: () = redis_client
        .set_ex(&redis_key, &access_token, 60 * 60)
        .await
        .unwrap();
    // 写入日志
    tracing::info!("ID为: {} 的用户登陆成功！", user_model.id);
    // 记录登陆时间并更新数据
    let mut user_active_model = user_model.into_active_model();
    user_active_model.last_login_at = Set(Some(get_local_datetime_with_timezone()));
    // 更新到数据库
    let _ = user_active_model.update(db_pool).await?;
    // 返回
    Ok(ApiResponse::ok(
        "登陆成功！",
        Some(LoginResult { access_token }),
    ))
}

#[debug_handler]
pub async fn get_user_info_handler(
    Extension(principal): Extension<Principal>,
) -> ApiResult<ApiResponse<Principal>> {
    Ok(ApiResponse::ok("ok", Some(principal)))
}
