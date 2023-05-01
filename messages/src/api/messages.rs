use crate::entities;

#[derive(Debug)]
pub struct CreateMessageRequest {
  pub content: String,
  pub author: i64,
}

#[derive(Debug)]
pub struct CreateMessageResponse {
  pub id: i64
}

#[derive(Debug)]
pub struct GetByIDRequest {
  pub id: i64
}

#[derive(Debug)]
pub struct GetByIDResponse {
  pub message: Option<entities::message::Message>
}