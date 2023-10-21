use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceId(String);

impl Default for ResourceId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    id: ResourceId,
    name: String,
}

impl Resource {
    pub fn new(name: &str) -> Resource {
        Self {
            id: ResourceId::default(),
            name: name.to_string(),
        }
    }

    pub fn get_id(&self) -> ResourceId {
        self.id.clone()
    }
}