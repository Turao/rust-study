use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::domain::{repositories::{Error, Repository}, subjects::{SubjectId, Subject}};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubjectRequest {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubjectResponse {
    pub subject_id: SubjectId,
    pub subject_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSubjectRequest {
    pub subject_id: SubjectId
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSubjectResponse {}

// todo: make SubjectServiceImpl implement this trait (somehow this does not work)
// I think the dyn Repository object does not implement the necessary traits
// to satisfy the borrow checker rules
#[async_trait]
pub trait SubjectService {
    async fn create_subject(&self, req: CreateSubjectRequest) -> Result<CreateSubjectResponse, Error>;
    async fn delete_subject(&self, req: DeleteSubjectRequest) -> Result<DeleteSubjectResponse, Error>;
}

pub struct SubjectServiceImpl {
    subject_repository: Box<dyn Repository<SubjectId, Subject>>
}

impl SubjectServiceImpl {
    pub fn new(subject_repository: Box<dyn Repository<SubjectId, Subject>>) -> Self {
        SubjectServiceImpl {
            subject_repository: subject_repository,
        }
    }

    pub async fn create_subject(&self, req: CreateSubjectRequest) -> Result<CreateSubjectResponse, Error> {
        let subject = Subject::new(&req.name);
        self.subject_repository.save(subject.clone())
            .await
            .expect("unable to save subject");
        Ok(CreateSubjectResponse {
            subject_id: subject.get_id(),
            subject_name: subject.get_name(),
        })
    }
    
    pub async fn delete_subject(&self, req: DeleteSubjectRequest) -> Result<DeleteSubjectResponse, Error> {
        let mut subject = self.subject_repository.get_by_id(req.subject_id)
            .await
            .expect("unable to fetch subject")
            .expect("subject not found");

        subject.delete();
        
        self.subject_repository.save(subject)
            .await
            .expect("unable to save subject");

        Ok(DeleteSubjectResponse {})
    }
}
