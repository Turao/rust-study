use sqlx::sqlite::SqlitePool;
use tracing::info;

use crate::domain::repositories::{Repository, Error};
use crate::domain::operations::Operation;
use crate::domain::permissions::Permission;
use crate::domain::resources::Resource;
use crate::domain::roles::Role;
use crate::domain::subjects::Subject;

use crate::infrastructure::sqlite::permission::SqlitePermissionRepository;
use crate::infrastructure::sqlite::role::SqliteRoleRepository;
use crate::infrastructure::sqlite::subject::SqliteSubjectRepository;

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
    for permission in engineer_role.get_permissions() {
        info!("{:?}", permission)
    }

    let connection_pool = SqlitePool::connect(":memory").await?;
    let mut connection = connection_pool.acquire().await?;
    sqlx::migrate!("./datastore/sqlite").run(&mut connection).await.expect("unable to migrate");

    let role_repository = SqliteRoleRepository::new(connection_pool.clone());
    role_repository.save(engineer_role.clone()).await?;
    let role = role_repository.get_by_id(engineer_role.get_id()).await?.unwrap();
    info!("{:?}", role);

    let mut john = Subject::new("john");
    john.add_role(engineer_role.get_id());
    
    let permission_repository = SqlitePermissionRepository::new(connection_pool.clone());
    permission_repository.save(list_users_permission.clone()).await?;
    permission_repository.save(update_user_permission.clone()).await?;

    info!("{:?}", permission_repository.get_by_id(list_users_permission.get_id()).await?.unwrap());

    let subject_repository = SqliteSubjectRepository::new(connection_pool.clone());
    subject_repository.save(john.clone()).await?;

    let john_wick = Subject::new("john wick");
    let baba_yaga = Subject::new("baba yaga");
    subject_repository.save(john_wick.clone()).await?;
    subject_repository.save(baba_yaga.clone()).await?;

    info!("{:?}", subject_repository.get_by_id(john.get_id()).await?.unwrap());
    info!("{:?}", subject_repository.get_by_id(john_wick.get_id()).await?.unwrap());
    info!("{:?}", subject_repository.get_by_id(baba_yaga.get_id()).await?.unwrap());

    let access_checker = application::access_checker::AccessChecker::new(
        Box::new(subject_repository),
        Box::new(role_repository),
        Box::new(permission_repository),
    );
    let can_access = access_checker.can_access(john_wick.get_id(), list_users_resource.get_id())
        .await
        .expect("failed to check if can access");
    info!("{:?}", can_access);

    Ok(())
}