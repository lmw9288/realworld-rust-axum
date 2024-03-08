use std::sync::Arc;

use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::{self, Next},
    routing::{get, post, put},
    Router,
};

use realworld_rust_axum::{
    config::parse_token,
    db::my_pool,
    models::{user::User, SessionState},
    routes::users::{current_user, login, registry, update_user},
};
use sqlx::{query_as, MySql};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // build our application with a single route

    let app = Router::new()
        .route("/api/users", post(registry))
        .route("/api/users/login", post(login))
        .route("/api/user", get(current_user))
        .route("/api/user", put(update_user))
        .layer(middleware::from_fn(
            |mut request: Request, next: Next| async {
                // do something with `request`...

                let path = request.uri().path();
                let method = request.method();
                // println!("middleware request path: {} {}", method, path);

                let headers = request.headers();

                let ignore_vec = vec![
                    (&Method::POST, "/api/users"),
                    (&Method::POST, "/api/users/login"),
                ];

                if !ignore_vec.contains(&(method, path)) {
                    if headers.contains_key("Authorization") {
                        let authorization = headers.get("Authorization");

                        match authorization {
                            Some(token) => {
                                let token = &token.to_str().unwrap()[6..];
                                match parse_token(token) {
                                    Ok(t) => {
                                        // println!("Authorization claims: {:?}", t);

                                        let claims = t.claims;

                                        let user_id = claims.user_id;

                                        // let t =
                                        //     query_as::<MySql, User>("select * from user where id = ?")
                                        //         .bind(user_id)
                                        //         .fetch_one(&my_pool().await)
                                        //         .await;

                                        let session_state = Arc::new(RwLock::new(SessionState {
                                            user_id: Some(user_id),
                                        }));
                                        request.extensions_mut().insert(session_state);

                                        return Ok(next.run(request).await);
                                    }
                                    Err(error) => {
                                        println!("Authorization token is empty {}", error);
                                        return Err(StatusCode::UNAUTHORIZED);
                                    }
                                }
                            }
                            None => return Err(StatusCode::UNAUTHORIZED),
                        }
                    } else {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
                Ok(next.run(request).await)
            },
        ));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
