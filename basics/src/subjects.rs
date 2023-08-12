use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::roles::{RoleId, Role};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SubjectId(String);

impl Default for SubjectId {
    fn default() -> Self {
        SubjectId(Uuid::new_v4().to_string())
    }
}

impl From<String> for SubjectId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Into<String> for SubjectId {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Subject {
    id: SubjectId,
    name: String,
    roles: HashMap<RoleId, Role>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Subject {
    pub fn new(name: &str) -> Subject {
        Subject {
            id: SubjectId::default(),
            name: name.to_string(),
            roles: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn builder() -> SubjectBuilder {
        SubjectBuilder::new()
    }

    pub fn get_id(&self) -> SubjectId {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role.get_id(), role);
        self.updated_at = Utc::now();
    }

    pub fn remove_role(&mut self, role_id: &RoleId) {
        self.roles.remove(role_id);
        self.updated_at = Utc::now();
    }

    pub fn get_roles(&self) -> Vec<&Role> {
        self.roles.values().collect()
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

   
}

pub struct SubjectBuilder {
    id: Option<SubjectId>,
    name: Option<String>,
    roles: Option<HashMap<RoleId, Role>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl SubjectBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            roles: None,
            created_at: None,
            updated_at: None,
        }
    }
    
    pub fn id(mut self, id: SubjectId) -> SubjectBuilder {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> SubjectBuilder {
        self.name = Some(name);
        self
    }

    pub fn roles(mut self, roles: HashMap<RoleId, Role>) -> SubjectBuilder {
        self.roles = Some(roles);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> SubjectBuilder {
        self.created_at = Some(created_at);
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> SubjectBuilder {
        self.updated_at = Some(updated_at);
        self
    }

    pub fn build(self) -> Subject {
        Subject {
            id: self.id.unwrap(),
            name: self.name.unwrap(),
            roles: self.roles.unwrap(),
            created_at: self.created_at.unwrap(),
            updated_at: self.updated_at.unwrap()
        }
    }
}