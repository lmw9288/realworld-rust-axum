use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserLoginRequest {
    pub user: UserLoginModel,
}

#[derive(Deserialize, Debug)]
pub struct UserLoginModel {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub user: UserRequestModel,
}

#[derive(Deserialize, Debug)]
pub struct UserRequestModel {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub user: UserResponseModel,
}

#[derive(Serialize, Debug)]
pub struct UserResponseModel {
    pub username: String,
    pub email: String,
    pub token: String,
    pub bio: String,
    pub image: Option<String>,
}
