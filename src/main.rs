use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // build our application with a single route

    let app = Router::new()
        // .route("/", get(|| async { Json((1, 2, 3)) }))
        .route(
            "/api/users",
            post(|Json(UserRequest { user }): Json<UserRequest>| async {
                println!("{:?}", user);
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
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct UserRequest {
    user: UserRequestModel,
}

#[derive(Deserialize, Debug)]
struct UserRequestModel {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    user: UserResponseModel,
}

#[derive(Serialize)]
struct UserResponseModel {
    username: String,
    email: String,
    token: String,
    bio: String,
    image: Option<String>,
}

pub async fn registration() {
    // todo: implement

    // models::User {
    //     username: "test".to_owned(),
    //     password: "test".to_owned(),
    //     email: "test".to_owned(),
    // }
}
