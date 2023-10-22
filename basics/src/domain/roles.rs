use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::permissions::PermissionId;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RoleId(String);

impl Default for RoleId {
    fn default() -> Self {
        RoleId(Uuid::new_v4().to_string())
    }
}

impl From<String> for RoleId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Into<String> for RoleId {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Role {
    id: RoleId,
    name: String,
    permissions: HashSet<PermissionId>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Role {
    pub fn new(name: &str) -> Role {
        Role {
            id: RoleId::default(),
            name: name.to_string(),
            permissions: HashSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn builder() -> RoleBuilder {
        RoleBuilder::new()
    }

    pub fn get_id(&self) -> RoleId {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_permission(&mut self, permission: PermissionId) {
        self.permissions.insert(permission);
        self.updated_at = Utc::now();
    }

    pub fn get_permissions(&self) -> HashSet<PermissionId> {
        self.permissions.clone()
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at.clone()
    }

    pub fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at.clone()
    }
}

pub struct RoleBuilder {
    id: Option<RoleId>,
    name: Option<String>,
    permissions: Option<HashSet<PermissionId>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl RoleBuilder {
    pub fn new() -> RoleBuilder {
        Self {
            id: None,
            name: None,
            permissions: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn id(mut self, id: RoleId) -> RoleBuilder {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> RoleBuilder {
        self.name = Some(name);
        self
    }

    pub fn permissions(mut self, permissions: HashSet<PermissionId>) -> RoleBuilder {
        self.permissions = Some(permissions);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> RoleBuilder {
        self.created_at = Some(created_at);
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> RoleBuilder {
        self.updated_at = Some(updated_at);
        self
    }

    pub fn build(self) -> Role {
        Role {
            id: self.id.unwrap(),
            name: self.name.unwrap(),
            permissions: self.permissions.unwrap(),
            created_at: self.created_at.unwrap(),
            updated_at: self.updated_at.unwrap(),
        }
    }
}