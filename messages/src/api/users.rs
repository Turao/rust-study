use crate::entities;

#[derive(Debug)]
pub struct CreateUserRequest {
  pub username: String,
}

#[derive(Debug)]
pub struct CreateUserResponse {
  pub id: i64
}

#[derive(Debug)]
pub struct GetByIDRequest {
  pub id: i64
}

#[derive(Debug)]
pub struct GetByIDResponse {
  pub user: Option<entities::user::User>
}