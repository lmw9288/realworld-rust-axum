use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::{query, query_as, MySql};

use crate::{
    db::pool,
    models::user::{
        User, UserLoginRequest, UserRegistryRequest, UserResponse, UserResponseModel,
        UserUpdateRequest,
    },
};
/**
 * Registration
 */
pub async fn registry(
    Json(UserRegistryRequest { user }): Json<UserRegistryRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let pool = &pool().await;
    let rows = query::<MySql>("insert into user (username,email,password) values (?,?,?)")
        .bind(user.username)
        .bind(user.email)
        .bind(user.password)
        .execute(pool)
        .await;

    match rows {
        Ok(rows) => {
            println!("registry rows: {:#?}", rows);

            let last_insert_id = rows.last_insert_id();

            let one = query_as::<MySql, User>(
                "select id, username, email, bio, image from user where id = ?",
            )
            .bind(last_insert_id)
            .fetch_one(pool)
            .await;
            match one {
                Ok(one) => {
                    println!("one: {:#?}", one);
                    return Ok(Json(UserResponse {
                        user: UserResponseModel {
                            username: one.username,
                            email: one.email,
                            bio: one.bio,
                            image: one.image,
                            token: Some("test".to_owned()),
                        },
                    }));
                }
                Err(err) => {
                    println!("one error: {:#?}", err);
                    return Err(AppError::from(err));
                }
            }
        }
        Err(err) => {
            println!("rows error: {:#?}", err);
            return Err(AppError::from(err));
        }
    }
}

/**
 * Authentication
 */
pub async fn login(Json(UserLoginRequest { user }): Json<UserLoginRequest>) -> Json<UserResponse> {
    println!("POST /api/users/login, body: {:?}", user);
    let user_response = UserResponse {
        user: UserResponseModel {
            username: "user.username".to_owned(),
            email: user.email,
            token: Some("test".to_owned()),
            bio: None,
            image: None,
        },
    };
    Json(user_response)
}

/**
 * Get Current User
 */
pub async fn current_user() -> Json<UserResponse> {
    let user_response = UserResponse {
        user: UserResponseModel {
            username: "user.username".to_owned(),
            email: "test@mail.com".to_owned(),
            token: Some("test".to_owned()),
            bio: None,
            image: None,
        },
    };
    Json(user_response)
}

/**
 * Update User
 */
pub async fn update_user(
    Json(UserUpdateRequest { user }): Json<UserUpdateRequest>,
) -> Json<UserResponse> {
    let user_response = UserResponse {
        user: UserResponseModel {
            username: "user.username".to_owned(),
            email: user.email,
            token: Some("test".to_owned()),
            bio: None,
            image: None,
        },
    };
    Json(user_response)
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
