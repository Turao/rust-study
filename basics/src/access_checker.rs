use crate::groups::Group;
use crate::operations::Operation;
use crate::subjects::Subject;

pub struct AccessChecker {}

impl AccessChecker {
    pub fn can_group_perform_operation(group: &Group, operation: &Operation) -> bool {
        group.get_roles()
            .into_iter()
            .flat_map(|role| role.get_permissions())
            .map(|role_permission| role_permission.get_operation())
            .any(|op| &op == operation)
    }

    pub fn can_subject_perform_operation(subject: &Subject, operation: &Operation) -> bool {
        subject.get_roles()
            .into_iter()
            .flat_map(|role| role.get_permissions())
            .map(|role_permission| role_permission.get_operation())
            .any(|op| &op == operation)
    }
}