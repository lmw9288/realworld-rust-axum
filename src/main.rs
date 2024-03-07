use axum::{
    extract::Request,
    http::{Response, StatusCode},
    middleware::{self, Next},
    routing::{get, post, put},
    Router,
};

use realworld_rust_axum::routes::users::{current_user, login, registry, update_user};

#[tokio::main]
async fn main() {
    // build our application with a single route

    let app = Router::new()
        .route("/api/users", post(registry))
        .route("/api/users/login", post(login))
        .route("/api/user", get(current_user))
        .route("/api/user", put(update_user))
        .layer(middleware::from_fn(|request: Request, next: Next| async {
            // do something with `request`...

            let path = request.uri().path();
            println!("middleware request path: {}", path);

            let headers = request.headers();

            if (path == "/api/users/login") {
                // println!("request received, headers: {:?}", headers);
                let response = next.run(request).await;

                // println!("request processed");
                // do something with `response`...

                Ok(response)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
