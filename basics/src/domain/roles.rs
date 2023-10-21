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