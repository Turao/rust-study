use crate::domain::permissions::{PermissionId, Permission};
use crate::domain::repositories::Repository;
use crate::domain::resources::ResourceId;
use crate::domain::roles::{RoleId, Role};
use crate::domain::subjects::{Subject, SubjectId};
use crate::domain::operations::Operation::Invoke;

pub struct AccessChecker {
    subject_repository: Box<dyn Repository<SubjectId, Subject>>,
    role_repository: Box<dyn Repository<RoleId, Role>>,
    permission_repository: Box<dyn Repository<PermissionId, Permission>>,
}

impl AccessChecker {
    pub fn new(
        subject_repository: Box<dyn Repository<SubjectId, Subject>>,
        role_repository: Box<dyn Repository<RoleId, Role>>,
        permission_repository: Box<dyn Repository<PermissionId, Permission>>,
    ) -> AccessChecker {
        Self {
            subject_repository: subject_repository,
            role_repository: role_repository,
            permission_repository: permission_repository
        }
    }

    pub async fn can_invoke(&self, subject_id: SubjectId, resource_id: ResourceId) -> Result<bool, ()> {
        let subject = self.subject_repository.get_by_id(subject_id)
            .await
            .expect("failed to fetch subject")
            .expect("subject not found");

        let mut can_invoke_resource = false;

        for role_id in subject.get_roles() {
            let role = self.role_repository.get_by_id(role_id)
                .await
                .expect("failed to fetch role")
                .expect("role not found");

            for permission_id in role.get_permissions() {
                let permission = self.permission_repository.get_by_id(permission_id)
                    .await
                    .expect("failed to fetch permission")
                    .expect("permission not found");

                match permission.get_operation() {
                    Invoke(resource) => {
                        if resource.get_id() == resource_id {
                            can_invoke_resource = true;
                            break
                        }
                    },
                }
            }
        }

        Ok(can_invoke_resource)
    }
}