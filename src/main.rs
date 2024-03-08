
use axum::{
    middleware, routing::{get, post, put}, Router
};

use realworld_rust_axum::{
    config::middle_layer::{parse_jwt_layer, print_request_response},
    routes::users::{current_user, login, registry, update_user},
};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // build our application with a single route

    let app = Router::new()
        .route("/api/users", post(registry))
        .route("/api/users/login", post(login))
        .route("/api/user", get(current_user))
        .route("/api/user", put(update_user))
        .layer(middleware::from_fn(parse_jwt_layer))
        // .layer(middleware::from_fn(print_request_response))
        // .route_layer(middleware::from_extractor::<RequireAuth>())
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
