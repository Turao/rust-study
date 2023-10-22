use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashSet;

use super::subjects::SubjectId;
use super::roles::RoleId;

#[derive(Debug, Clone)]
pub struct GroupId(String);

impl Default for GroupId {
    fn default() -> Self {
        GroupId(Uuid::new_v4().to_string())
    }
}

impl From<String> for GroupId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Into<String> for GroupId {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub struct Group {
    id: GroupId,
    name: String,
    subjects: HashSet<SubjectId>,
    roles: HashSet<RoleId>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Group {
    pub fn new(name: &str) -> Group {
        Group {
            id: GroupId::default(),
            name: name.to_string(),
            subjects: HashSet::new(),
            roles: HashSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn builder() -> GroupBuilder {
        GroupBuilder::new()
    }

    pub fn get_id(&self) -> GroupId {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_subject(&mut self, subject_id: SubjectId) {
        self.subjects.insert(subject_id);
        self.updated_at = Utc::now();
    }

    pub fn remove_subject(&mut self, subject_id: &SubjectId) {
        self.subjects.remove(subject_id);
        self.updated_at = Utc::now();
    }

    pub fn get_subjects(&self) -> &HashSet<SubjectId> {
        &self.subjects
    }

    pub fn add_role(&mut self, role: RoleId) {
        self.roles.insert(role);
        self.updated_at = Utc::now();
    }

    pub fn remove_role(&mut self, role_id: &RoleId) {
        self.roles.remove(role_id);
        self.updated_at = Utc::now();
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

pub struct GroupBuilder {
    id: Option<GroupId>,
    name: Option<String>,
    subjects: Option<HashSet<SubjectId>>,
    roles: Option<HashSet<RoleId>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl GroupBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            subjects: None,
            roles: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn id(mut self, id: GroupId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn subjects(mut self, subjects: HashSet<SubjectId>) -> Self {
        self.subjects = Some(subjects);
        self
    }

    pub fn roles(mut self, roles: HashSet<RoleId>) -> Self {
        self.roles = Some(roles);
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

    pub fn build(self) -> Group {
        Group {
            id: self.id.unwrap(),
            name: self.name.unwrap(),
            subjects: self.subjects.unwrap(),
            roles: self.roles.unwrap(),
            created_at: self.created_at.unwrap(),
            updated_at: self.updated_at.unwrap(),
        }
    }
}