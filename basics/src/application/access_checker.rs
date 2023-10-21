use crate::domain::{repositories::Repository, subjects::{Subject, SubjectId}, resources::ResourceId};

pub struct AccessChecker {
    subject_repository: Box<dyn Repository<SubjectId, Subject>>
}

impl AccessChecker {
    pub fn new(subject_repository: Box<dyn Repository<SubjectId, Subject>>) -> AccessChecker {
        Self {
            subject_repository: subject_repository,
        }
    }

    pub async fn can_access(&self, subject_id: SubjectId, resource_id: ResourceId) -> Result<bool, ()> {
        let subject = self.subject_repository.get_by_id(subject_id)
            .await
            .expect("failed to fetch subject")
            .expect("subject not found");


        Ok(false)
    }
}