use crate::response::resp::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("{0}")]
    Biz(String),
    // #[error("JWT Error: {0}")]
    // JWT(#[from] jsonwebtoken::errors::Error),
    #[error("尚未授权：{0}")]
    Unauthenticated(String),
    #[error("查询参数错误: {0}")]
    QueryError(#[from] QueryRejection),
    #[error("路径参数错误: {0}")]
    PathError(#[from] PathRejection),
    #[error("Body 参数错误: {0}")]
    JsonError(#[from] JsonRejection),
    #[error("参数校验失败：{0}")]
    ValidationError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("密码加密时出错：{0}")]
    Argon2HashingError(#[from] argon2::password_hash::Error),
    #[error("服务端错误: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ApiError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) => axum::http::StatusCode::OK,
            ApiError::Unauthenticated(_) => axum::http::StatusCode::UNAUTHORIZED,
            ApiError::QueryError(_)
            | ApiError::PathError(_)
            | ApiError::JsonError(_)
            | ApiError::ValidationError(_) => axum::http::StatusCode::BAD_REQUEST,
            ApiError::Argon2HashingError(_)
            | ApiError::Internal(_)
            | ApiError::DatabaseError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// 为 ApiError 实现转换为校验失败的 trait
impl From<axum_valid::ValidRejection<ApiError>> for ApiError {
    fn from(value: axum_valid::ValidRejection<ApiError>) -> Self {
        match value {
            axum_valid::ValidationRejection::Valid(errors) => {
                ApiError::ValidationError(errors.to_string())
            }
            axum_valid::ValidationRejection::Inner(error) => error,
        }
    }
}

/// From api error into axum response.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status_code(),
            axum::Json(ApiResponse::<()>::err(self.to_string())),
        )
            .into_response()
    }
}

/// From api error into axum http response with body.
impl From<ApiError> for axum::http::Response<axum::body::Body> {
    fn from(error: ApiError) -> Self {
        error.into_response()
    }
}
