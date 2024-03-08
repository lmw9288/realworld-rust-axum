use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use chrono::Local;
use sqlx::{
    query, query_as, MySql,
};
use tokio::sync::RwLock;

use crate::{
    config::{gen_token, Claims},
    db::my_pool,
    models::{
        user::{
            User, UserLoginRequest, UserRegistryRequest, UserResponse, UserResponseModel,
            UserUpdateRequest,
        },
        SessionState,
    },
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
                Err(_err) => return Err(StatusCode::BAD_REQUEST),
            }
        }
        Err(_err) => return Err(StatusCode::BAD_REQUEST),
    }
}

/**
 * Get Current User
 */
pub async fn current_user(
    Extension(session_state): Extension<Arc<RwLock<SessionState>>>,
) -> Result<Json<UserResponse>, StatusCode> {
    let session_state = session_state.read().await;

    if let Some(user_id) = &session_state.user_id {
        println!("current user id: {:#?}", user_id);

        let one = query_as::<MySql, User>(
            "select id, username, email, bio, image from user where id = ?",
        )
        .bind(user_id)
        .fetch_one(&my_pool().await)
        .await;

        match one {
            Ok(one) => {
                return Ok(Json(UserResponse {
                    user: UserResponseModel {
                        username: one.clone().username,
                        email: one.email,
                        bio: one.bio,
                        image: one.image,
                        token: session_state.clone().token,
                    },
                }));
            }

            Err(_err) => return Err(StatusCode::BAD_REQUEST),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/**
 * Update User
 */
pub async fn update_user(
    Extension(session_state): Extension<Arc<RwLock<SessionState>>>,
    Json(UserUpdateRequest { user }): Json<UserUpdateRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let session_state = session_state.read().await;

    // let mut columns = vec![];
    // let mut values = vec![];

    if let Some(user_id) = &session_state.user_id {
        let mut columns = vec![];
        let mut values = vec![];
        if let Some(username) = user.username {
            columns.push("username = ?");
            values.push(username);
        }
        if let Some(email) = user.email {
            columns.push("email = ?");
            values.push(email);
        }
        if let Some(password) = user.password {
            columns.push("password = ?");
            values.push(password);
        }
        if let Some(bio) = user.bio {
            columns.push("bio = ?");
            values.push(bio);
        }
        if let Some(image) = user.image {
            columns.push("image = ?");
            values.push(image);
        }
        let sql = format!("update user set {} where id = ?", columns.join(", "));
        let sql = sql.as_str();

        let mut q = query::<MySql>(sql);
        for ele in values {
            q = q.bind(ele);
        }
        let result = q.bind(user_id).execute(&my_pool().await).await;

        match result {
            Ok(_result) => {
                let one = query_as::<MySql, User>(
                    "select id, username, email, bio, image from user where id = ?",
                )
                .bind(user_id)
                .fetch_one(&my_pool().await)
                .await;
                match one {
                    Ok(one) => {
                        return Ok(Json(UserResponse {
                            user: UserResponseModel {
                                username: one.clone().username,
                                email: one.email,
                                token: session_state.clone().token,
                                bio: one.bio,
                                image: one.image,
                            },
                        }))
                    }
                    Err(_err) => {
                        println!("get user error");
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
            }
            Err(err) => {
                println!("update user error {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
