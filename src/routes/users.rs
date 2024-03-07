use axum::{
    http::{status, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use sqlx::{query, query_as, MySql};

use crate::{
    config::{AppError, Claims},
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
) -> Result<Json<UserResponse>, StatusCode> {
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

                    let my_claims = Claims {
                        sub: one.clone().username,
                        exp: 1000 * 60 * 60 * 12,
                    };

                    let token = jsonwebtoken::encode(
                        &jsonwebtoken::Header::default(),
                        &my_claims,
                        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
                    );

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
