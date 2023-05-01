use log::{info, debug};
use crate::entities::message::{MessageBuilder};
use crate::api::messages::{CreateMessageRequest, CreateMessageResponse, GetByIDRequest, GetByIDResponse};
use crate::entities::user::UserBuilder;
use crate::repositories::message::Repository;

pub struct Service {
  repository: Repository
}

impl Service {
  pub fn new(repository: Repository) -> Result<Service, String> {
    Ok(Service {
      repository: repository
    })
  }

  pub fn create_message(&mut self, req: CreateMessageRequest) -> Result<CreateMessageResponse, String> {
    info!("creating message");

    let author = UserBuilder::new().id(req.author).build().unwrap();
    debug!("author: {:?}", author);

    let message = MessageBuilder::new().
      content(req.content).
      author(req.author).
      build().
      unwrap();
    self.repository.save(&message).unwrap();
    Ok(CreateMessageResponse{
      id: message.get_id(),
    })
  }

  pub fn get_by_id(self, req: GetByIDRequest) -> Result<GetByIDResponse, String> {
    info!("fetching by id");
    
    match self.repository.get_by_id(req.id) {
      Some(message) => Ok(GetByIDResponse{
        message: Some(message.clone())
      }),
      _ => Ok(GetByIDResponse{
        message: None
      })
    }
  }
}