use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use chrono::{Date, DateTime, Duration, Local};
use sqlx::{query, query_as, MySql};
use tokio::sync::RwLock;

use crate::{
    config::{gen_token, Claims},
    db::my_pool,
    models::{user::{
        User, UserLoginRequest, UserRegistryRequest, UserResponse, UserResponseModel,
        UserUpdateRequest,
    }, SessionState},
};
/**
 * Registration
 */
pub async fn registry(
    Json(UserRegistryRequest { user }): Json<UserRegistryRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // todo: password encode
    let pool = &my_pool().await;
    let rows = query::<MySql>("insert into user (username,email,password) values (?,?,?)")
        .bind(user.username)
        .bind(user.email)
        .bind(user.password)
        .execute(pool)
        .await;

    match rows {
        Ok(rows) => {
            // println!("registry rows: {:#?}", rows);

            let last_insert_id = rows.last_insert_id();

            let one = query_as::<MySql, User>(
                "select id, username, email, bio, image from user where id = ?",
            )
            .bind(last_insert_id)
            .fetch_one(pool)
            .await;
            match one {
                Ok(one) => {
                    let my_claims = Claims {
                        user_id: one.clone().id,
                        sub: one.clone().email,
                        exp: (Local::now().timestamp() + 60 * 60 * 2) as usize,
                    };

                    let token = gen_token(&my_claims);

                    // println!("token: {:#?}", token);

                    match token {
                        Ok(token) => {
                            return Ok(Json(UserResponse {
                                user: UserResponseModel {
                                    username: one.clone().username,
                                    email: one.email,
                                    bio: one.bio,
                                    image: one.image,
                                    token: Some(token),
                                },
                            }));
                        }
                        Err(err) => {
                            println!("token error: {:#?}", err);
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    }
                }
                Err(err) => {
                    println!("one error: {:#?}", err);
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
        Err(err) => {
            println!("rows error: {:#?}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

/**
 * Authentication
 */
pub async fn login(
    Json(UserLoginRequest { user }): Json<UserLoginRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let pool = &my_pool().await;

    let one = query_as::<MySql, User>(
        "select id, username, email, bio, image from user where email = ? and password = ?",
    )
    .bind(user.email)
    .bind(user.password)
    .fetch_one(pool)
    .await;

    match one {
        Ok(one) => {
            let my_claims = Claims {
                user_id: one.clone().id,
                sub: one.clone().email,
                exp: (Local::now().timestamp() + 60 * 60 * 2) as usize,
            };

            let token = gen_token(&my_claims);

            match token {
                Ok(token) => {
                    return Ok(Json(UserResponse {
                        user: UserResponseModel {
                            username: one.clone().username,
                            email: one.email,
                            bio: one.bio,
                            image: one.image,
                            token: Some(token),
                        },
                    }));
                }
                Err(err) => return Err(StatusCode::BAD_REQUEST),
            }
        }
        Err(err) => return Err(StatusCode::BAD_REQUEST),
    }
}

/**
 * Get Current User
 */
pub async fn current_user(
    Extension(session_state): Extension<Arc<RwLock<SessionState>>>,
) -> Result<Json<UserResponse>, StatusCode>  {

    let mut session_state = session_state.write().await;

    if let Some(user_id) = &session_state.user_id {
        println!("current user id: {:#?}", user_id);

        Ok(Json(UserResponse {
            user: UserResponseModel {
                username: "user.username".to_owned(),
                email: "test@mail.com".to_owned(),
                token: Some("test".to_owned()),
                bio: None,
                image: None,
            },
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
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
