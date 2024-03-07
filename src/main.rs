use axum::{
    routing::{get, post, put},
    Router,
};

use realworld_rust_axum::routes::users::{current_user, login, registry, update_user};

#[tokio::main]
async fn main() {
    // build our application with a single route


    let app = Router::new()
        // .route("/", get(|| async { Json((1, 2, 3)) }))
        .route("/api/users", post(registry))
        .route("/api/users/login", post(login))
        .route("/api/user", get(current_user))
        .route("/api/user", put(update_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
