use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, sqlx::FromRow, sqlx::Decode)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

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
pub struct UserRegistryRequest {
    pub user: UserRegistryModel,
}

#[derive(Deserialize, Debug)]
pub struct UserRegistryModel {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub user: UserResponseModel,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserResponseModel {
    pub username: String,
    pub email: String,
    pub token: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateRequest {
    pub user: UserUpdateModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateModel {
    pub email: String,
    pub bio: String,
    pub image: Option<String>,
}
