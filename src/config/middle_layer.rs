use std::sync::Arc;

use axum::{
    body::{self, Body, Bytes},
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next, response::IntoResponse,
};
use hyper::header;
use tokio::sync::RwLock;

use crate::models::SessionState;

use super::parse_token;

pub async fn parse_jwt_layer(
    mut request: Request,
    next: Next,
) -> Result<axum::http::Response<axum::body::Body>, (StatusCode, String)> {
    // do something with `request`...

    let path = request.uri().path();
    let method: &Method = request.method();
    let headers = request.headers();

    let ignore_vec = vec![
        (&Method::POST, "/api/users"),
        (&Method::POST, "/api/users/login"),
    ];
    // skip registry and login
    if ignore_vec.contains(&(method, path)) {
        return Ok(next.run(request).await);
    }

    // header authorization not found
    if !headers.contains_key(header::AUTHORIZATION) {
        return Err((
            StatusCode::UNAUTHORIZED,
            format!("header Authorization not found"),
        ));
    }

    let authorization = headers.get(header::AUTHORIZATION);
    match authorization {
        Some(token) => {
            let token = &token.to_str().unwrap()[6..];
            match parse_token(token) {
                Ok(t) => {
                    // println!("Authorization claims: {:?}", t);

                    let claims = t.claims;

                    let user_id = claims.user_id;

                    let session_state = Arc::new(RwLock::new(SessionState {
                        user_id: Some(user_id),
                        token: Some(token.to_string()),
                    }));
                    request.extensions_mut().insert(session_state);

                    return Ok(next.run(request).await);
                }
                Err(error) => {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        format!("parse Authorization token error {}", error),
                    ));
                }
            }
        }
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                format!("Authorization token is empty"),
            ))
        }
    }
}


pub async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("{}", req.method());
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    // let (parts, body) = res.into_parts();
    // let bytes = buffer_and_print("response", body).await?;
    // let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print(direction: &str, body: Body) -> Result<Bytes, (StatusCode, String)>
{
    let bytes = match body::to_bytes(body, usize::MAX).await {
        Ok(collected) => collected,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        println!("{direction} body = {body:?}");
    }

    Ok(bytes)
}