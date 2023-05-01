
pub type ID = i64;

#[derive(Debug, Clone)]
pub struct User {
  id: ID,
  username: String,
}

impl User {
  pub fn get_id(&self) -> ID {
    self.id
  }

  pub fn get_username(&self) -> String {
    self.username.clone()
  }
}

pub struct UserBuilder {
  id: ID,
  username: String,
}

impl UserBuilder {
  pub fn new() -> UserBuilder {
    UserBuilder{
      id: 1,
      username: "".to_string(),
    }
  }

  pub fn id(mut self, id: ID) -> UserBuilder {
    self.id = id;
    self
  }

  pub fn username(mut self, username: String) -> UserBuilder {
    self.username = username;
    self
  }

  pub fn build(self) -> Result<User, String> {
    Ok(User{
      id: self.id,
      username: self.username,
    })
  }
}
