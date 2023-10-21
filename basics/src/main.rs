use sqlx::sqlite::SqlitePool;
use tracing::info;

use crate::domain::repositories::{Repository, Error};

mod domain;
mod application;
mod infrastructure;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let subscriber = tracing_subscriber::FmtSubscriber::default();
    tracing::subscriber::set_global_default(subscriber).expect("unable to set global tracing subscriber");

    let list_users_resource = domain::resources::Resource::new("users/get_users");
    let list_users_operation = domain::operations::Operation::Invoke(list_users_resource.clone());
    let list_users_permission = domain::permissions::Permission::new(
        "list users",
        list_users_operation.clone(),
    );

    let update_user_resource = domain::resources::Resource::new("users/update_user");
    let update_user_operation = domain::operations::Operation::Invoke(update_user_resource);
    let update_user_permission = domain::permissions::Permission::new(
        "update user",
        update_user_operation.clone(),
    );

    let mut engineer_role = domain::roles::Role::new("engineer");
    engineer_role.add_permission(list_users_permission.get_id());
    engineer_role.add_permission(update_user_permission.get_id());
    for permission in engineer_role.get_permissions() {
        info!("{:?}", permission)
    }

    let mut john = domain::subjects::Subject::new("john");
    john.add_role(engineer_role.get_id());

    // let ok = application::access_checker::AccessChecker::can_subject_perform_operation(&john, &list_users_operation);
    // info!("ok: {:?}", ok);

    let connection_pool = SqlitePool::connect(":memory").await?;
    let mut connection = connection_pool.acquire().await?;
    sqlx::migrate!("./datastore/sqlite").run(&mut connection).await.expect("unable to migrate");
    
    let permission_repository = infrastructure::repositories::SqlitePermissionRepository::new(connection_pool.clone());
    permission_repository.save(list_users_permission.clone()).await?;
    permission_repository.save(update_user_permission.clone()).await?;

    info!("{:?}", permission_repository.get_by_id(list_users_permission.get_id()).await?.unwrap());

    let subject_repository = infrastructure::repositories::SqliteSubjectRepository::new(connection_pool.clone());
    subject_repository.save(john.clone()).await?;

    let john_wick = domain::subjects::Subject::new("john wick");
    let baba_yaga = domain::subjects::Subject::new("baba yaga");
    subject_repository.save(john_wick.clone()).await?;
    subject_repository.save(baba_yaga.clone()).await?;

    info!("{:?}", subject_repository.get_by_id(john.get_id()).await?.unwrap());
    info!("{:?}", subject_repository.get_by_id(john_wick.get_id()).await?.unwrap());
    info!("{:?}", subject_repository.get_by_id(baba_yaga.get_id()).await?.unwrap());

    // let mut group = groups::Group::new("engineering_team");
    // group.add_subject(subjects::Subject::new("homer").get_id());
    // group.add_subject(subjects::Subject::new("marge").get_id());
    // group.add_subject(subjects::Subject::new("maggie").get_id());
    // group.add_subject(subjects::Subject::new("bart").get_id());
    // group.add_subject(subjects::Subject::new("lisa").get_id());
    

    let access_checker = application::access_checker::AccessChecker::new(Box::new(subject_repository));
    let can_access = access_checker.can_access(john.get_id(), list_users_resource.get_id())
        .await
        .expect("failed to check if can access");
    info!("{:?}", can_access);

    Ok(())
}