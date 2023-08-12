use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

use crate::{subjects::SubjectId, roles::{RoleId, Role}};

#[derive(Debug)]
pub struct GroupId(String);

impl Default for GroupId {
    fn default() -> Self {
        GroupId(Uuid::new_v4().to_string())
    }
}

#[derive(Debug)]
pub struct Group {
    id: GroupId,
    name: String,
    subjects: HashSet<SubjectId>,
    roles: HashMap<RoleId, Role>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Group {
    pub fn new(name: &str) -> Group {
        Group {
            id: GroupId::default(),
            name: name.to_string(),
            subjects: HashSet::new(),
            roles: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
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
}