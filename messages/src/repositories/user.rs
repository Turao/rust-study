use log::{debug};
use std::collections::HashMap;
use crate::entities::user::{ID, User, UserBuilder};


#[derive(Debug)]
struct Model {
  id: i64,
  username: String,
}

impl Model {
  fn from_entity(entity: &User) -> Model {
    Model {
      id: entity.get_id(),
      username: entity.get_username().clone(),
    }
  }

  fn to_entity(&self) -> User {
    return UserBuilder::new().
      id(self.id).
      username(self.username.clone()).
      build().
      unwrap();
  }
}

pub struct Repository {
  users: HashMap<ID, Model>
}

impl Repository {
  pub fn new() -> Result<Repository, String> {
    let users: HashMap<ID, Model> = HashMap::new();
    Ok(Repository{
      users: users,
    })
  }

  pub fn save(&mut self, user: &User) -> Result<(), String>{
    let model = Model::from_entity(user);
    debug!("Saving Model: {:?}", model);
    self.users.insert(model.id, model);
    Ok(())
  }

  pub fn get_by_id(&self, id: ID) -> Option<User> {
    match self.users.get(&id) {
      Some(model) => {
        debug!("Model Found: {:?}", model);
        Some(model.to_entity())
      },
      None => None
    }
  }
}