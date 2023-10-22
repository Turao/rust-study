use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::roles::RoleId;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
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
    version: i64,
    name: String,
    roles: HashSet<RoleId>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Subject {
    pub fn new(name: &str) -> Subject {
        Subject {
            id: SubjectId::default(),
            version: 0,
            name: name.to_string(),
            roles: HashSet::new(),
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

    pub fn get_version(&self) -> i64 {
        self.version
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn add_role(&mut self, roles: RoleId) {
        self.roles.insert(roles);
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn remove_role(&mut self, role: &RoleId) {
        self.roles.remove(role);
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn get_roles(&self) -> HashSet<RoleId> {
        self.roles.clone()
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
    version: Option<i64>,
    name: Option<String>,
    roles: Option<HashSet<RoleId>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl SubjectBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            version: None,
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

    pub fn version(mut self, version: i64) -> SubjectBuilder {
        self.version = Some(version);
        self
    }

    pub fn name(mut self, name: String) -> SubjectBuilder {
        self.name = Some(name);
        self
    }

    pub fn roles(mut self, roles: HashSet<RoleId>) -> SubjectBuilder {
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
            version: self.version.unwrap(),
            name: self.name.unwrap(),
            roles: self.roles.unwrap(),
            created_at: self.created_at.unwrap(),
            updated_at: self.updated_at.unwrap()
        }
    }
}