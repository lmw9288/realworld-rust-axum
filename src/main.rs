use axum::{
    routing::{get, post},
    Json, Router,
};

use rand::prelude::*;

use serde::{Deserialize, Serialize};

use realworld_rust_axum::models::user::{ UserLoginRequest, UserRequest, UserResponse, UserResponseModel };

#[tokio::main]
async fn main() {
    // build our application with a single route

    let app = Router::new()
        // .route("/", get(|| async { Json((1, 2, 3)) }))
        .route(
            "/api/users",
            post(|Json(UserRequest { user }): Json<UserRequest>| async {
                println!("POST /api/users, body: {:?}", user);
                // let mut rng = rand::thread_rng();
                // let token: usize = rng.gen();
                let user_response = UserResponse {
                    user: UserResponseModel {
                        username: user.username,
                        email: user.email,
                        token: "test".to_owned(),
                        bio: "test".to_owned(),
                        image: None,
                    },
                };
                Json(user_response)
            }),
        )
        .route(
            "/api/users/login",
            post(
                |Json(UserLoginRequest { user }): Json<UserLoginRequest>| async {
                    println!("POST /api/users/login, body: {:?}", user);
                    let user_response = UserResponse {
                        user: UserResponseModel {
                            username: "user.username".to_owned(),
                            email: user.email,
                            token: "test".to_owned(),
                            bio: "test".to_owned(),
                            image: None,
                        },
                    };
                    Json(user_response)
                },
            ),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}




