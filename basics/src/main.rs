use sqlx::sqlite::SqlitePool;
use tracing::info;

use crate::domain::repositories::{Repository, Error};
use crate::domain::operations::Operation;
use crate::domain::permissions::Permission;
use crate::domain::resources::Resource;
use crate::domain::roles::Role;
use crate::domain::subjects::Subject;
use crate::domain::groups::Group;

use crate::infrastructure::sqlite::group::SqliteGroupRepository;
use crate::infrastructure::sqlite::permission::SqlitePermissionRepository;
use crate::infrastructure::sqlite::role::SqliteRoleRepository;
use crate::infrastructure::sqlite::subject::SqliteSubjectRepository;

use crate::application::subjects::{SubjectServiceImpl, CreateSubjectRequest, DeleteSubjectRequest};

mod domain;
mod application;
mod infrastructure;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let subscriber = tracing_subscriber::FmtSubscriber::default();
    tracing::subscriber::set_global_default(subscriber).expect("unable to set global tracing subscriber");

    let list_users_resource = Resource::new("users/get_users");
    let list_users_operation = Operation::Invoke(list_users_resource.clone());
    let list_users_permission = Permission::new(
        "list users",
        list_users_operation.clone(),
    );

    let update_user_resource = Resource::new("users/update_user");
    let update_user_operation = Operation::Invoke(update_user_resource);
    let update_user_permission = Permission::new(
        "update user",
        update_user_operation.clone(),
    );

    let mut engineer_role = Role::new("engineer");
    engineer_role.add_permission(list_users_permission.get_id());
    engineer_role.add_permission(update_user_permission.get_id());

    let connection_pool = SqlitePool::connect("datastore/memory").await?;
    let mut connection = connection_pool.acquire().await?;
    sqlx::migrate!("./datastore/sqlite").run(&mut connection).await.expect("unable to migrate");

    let role_repository = SqliteRoleRepository::new(connection_pool.clone());
    role_repository.save(engineer_role.clone()).await?;
    let role = role_repository.get_by_id(engineer_role.get_id()).await?.unwrap();
    info!("{:?}", role);

    let permission_repository = SqlitePermissionRepository::new(connection_pool.clone());
    permission_repository.save(list_users_permission.clone()).await?;
    permission_repository.save(update_user_permission.clone()).await?;

    info!("{:?}", permission_repository.get_by_id(list_users_permission.get_id()).await?.unwrap());

    let subject_repository = SqliteSubjectRepository::new(connection_pool.clone());
    let subject_service = SubjectServiceImpl::new(Box::new(subject_repository));
    let john_wick_id = subject_service.create_subject(
        CreateSubjectRequest { name: "john wick".to_string() }
    ).await?.subject_id;

    subject_service.delete_subject(
        DeleteSubjectRequest { subject_id: john_wick_id.clone() }
    ).await?;

    let alec_leamas_id = subject_service.create_subject(
        CreateSubjectRequest { name: "alec leamas".to_string() }
    ).await?.subject_id;

    let mut employees_group = Group::new("employees");
    employees_group.add_subject(john_wick_id.clone());
    employees_group.add_subject(alec_leamas_id.clone());
    employees_group.add_role(engineer_role.get_id());

    let group_repository = SqliteGroupRepository::new(connection_pool.clone());
    group_repository.save(employees_group).await.expect("failed to save employees group");

    let subject_repository_2 = SqliteSubjectRepository::new(connection_pool.clone());
    let access_checker = application::access_checker::AccessChecker::new(
        Box::new(subject_repository_2),
        Box::new(role_repository),
        Box::new(permission_repository),
    );
    let can_invoke = access_checker.can_invoke(john_wick_id.clone(), list_users_resource.get_id())
        .await
        .expect("failed to check if can invoke");
    info!("{:?}", can_invoke);

    

    Ok(())
}