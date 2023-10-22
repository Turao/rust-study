use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::operations::Operation;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PermissionId(String);

impl Default for PermissionId {
    fn default() -> Self {
        PermissionId(Uuid::new_v4().to_string())
    }
}

impl From<String> for PermissionId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Into<String> for PermissionId {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Permission {
    id: PermissionId,
    name: String,
    operation: Operation,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Permission {
    pub fn new(name: &str, operation: Operation) -> Permission {
        Permission {
            id: PermissionId::default(),
            name: name.to_string(),
            operation: operation,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn builder() -> PermissionBuilder {
        PermissionBuilder::new()
    }

    pub fn get_id(&self) -> PermissionId {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_operation(&self) -> Operation {
        self.operation.clone()
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug)]
pub struct PermissionBuilder {
    id: Option<PermissionId>,
    name: Option<String>,
    operation: Option<Operation>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl PermissionBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            operation: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn id(mut self, id: PermissionId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn operation(mut self, operation: Operation) -> Self {
        self.operation = Some(operation);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }

    pub fn build(self) -> Permission {
        Permission {
            id: self.id.unwrap(),
            name: self.name.unwrap(),
            operation: self.operation.unwrap(),
            created_at: self.created_at.unwrap(),
            updated_at: self.updated_at.unwrap(),
        }
    }
}
