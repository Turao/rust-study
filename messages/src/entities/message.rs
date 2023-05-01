use crate::entities::user;

pub type ID = i64;

#[derive(Debug, Clone)]
pub struct Message {
    id: ID,
    content: String,
    author: user::ID,
}

impl Message {
    pub fn get_id(&self) -> ID {
        self.id
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn get_author(&self) -> user::ID {
        self.author
    }
}

#[derive(Debug)]
pub struct MessageBuilder {
    id: ID,
    content: String,
    author: user::ID,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder {
            id: 0,
            content: "".to_string(),
            author: 0,
        }
    }

    pub fn id(mut self, id: ID) -> MessageBuilder {
        self.id = id;
        self
    }

    pub fn content(mut self, content: String) -> MessageBuilder {
        self.content = content;
        self
    }

    pub fn author(mut self, author: user::ID) -> MessageBuilder {
        self.author = author;
        self
    }

    pub fn build(self) -> Result<Message, String> {
        Ok(Message {
            id: self.id,
            content: self.content,
            author: self.author,
        })
    }
}
