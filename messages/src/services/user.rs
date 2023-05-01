use log::{info, debug};
use crate::entities::user::{UserBuilder};
use crate::api::users::{CreateUserRequest, CreateUserResponse, GetByIDRequest, GetByIDResponse};
use crate::repositories::user::Repository;

pub struct Service {
  repository: Repository
}

impl Service {
  pub fn new(repository: Repository) -> Result<Service, String> {
    Ok(Service {
      repository: repository
    })
  }

  pub fn create_user(&mut self, req: CreateUserRequest) -> Result<CreateUserResponse, String> {
    info!("creating user");

    let user = UserBuilder::new().
      username(req.username).
      build().
      unwrap();
    debug!("user: {:?}", user);

    self.repository.save(&user).unwrap();
    Ok(CreateUserResponse{
      id: user.get_id(),
    })
  }

  pub fn get_by_id(self, req: GetByIDRequest) -> Result<GetByIDResponse, String> {
    info!("fetching by id");
    
    match self.repository.get_by_id(req.id) {
      Some(user) => Ok(GetByIDResponse{
        user: Some(user.clone())
      }),
      _ => Ok(GetByIDResponse{
        user: None
      })
    }
  }
}