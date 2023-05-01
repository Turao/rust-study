use log::{debug};
use std::collections::HashMap;
use crate::entities::message::{ID, Message, MessageBuilder};


#[derive(Debug)]
struct Model {
  id: i64,
  content: String,
  author: i64,
}

impl Model {
  fn from_entity(entity: &Message) -> Model {
    Model {
      id: entity.get_id(),
      content: entity.get_content(),
      author: entity.get_author(),
    }
  }

  fn to_entity(&self) -> Message {
    return MessageBuilder::new().
      id(self.id).
      content(self.content.clone()).
      author(self.author).
      build().
      unwrap();
  }
}

pub struct Repository {
  messages: HashMap<ID, Model>
}

impl Repository {
  pub fn new() -> Result<Repository, String> {
    let messages: HashMap<ID, Model> = HashMap::new();
    Ok(Repository{
      messages: messages,
    })
  }

  pub fn save(&mut self, message: &Message) -> Result<(), String>{
    let model = Model::from_entity(message);
    debug!("Saving Model: {:?}", model);
    self.messages.insert(model.id, model);
    Ok(())
  }

  pub fn get_by_id(&self, id: ID) -> Option<Message> {
    match self.messages.get(&id) {
      Some(model) => {
        debug!("Model Found: {:?}", model);
        Some(model.to_entity())
      },
      None => None
    }
  }
}