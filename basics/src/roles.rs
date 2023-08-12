use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::permissions::{PermissionId, Permission};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
    permissions: HashMap<PermissionId, Permission>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Role {
    pub fn new(name: &str) -> Role {
        Role {
            id: RoleId::default(),
            name: name.to_string(),
            permissions: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn get_id(&self) -> RoleId {
        self.id.clone()
    }

    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission.get_id(), permission);
        self.updated_at = Utc::now();
    }

    pub fn get_permissions(&self) -> Vec<&Permission> {
        self.permissions.values().collect()
    }
}