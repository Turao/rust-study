use crate::repositories::{SqlitePermissionRepository, Repository, SqliteSubjectRepository};
use sqlx::sqlite::SqlitePool;
use tracing::info;

mod domain;

mod access_checker;
mod repositories;

#[async_std::main]
async fn main() -> Result<(), repositories::RepositoryError> {
    let subscriber = tracing_subscriber::FmtSubscriber::default();
    tracing::subscriber::set_global_default(subscriber).expect("unable to set global tracing subscriber");

    let list_users_operation = domain::operations::Operation::Invoke(
        domain::resources::Resource::Endpoint(
            "users".to_string(),
            "get_users".to_string(),
        ),
    );
    let list_users_permission = domain::permissions::Permission::new(
        "list users",
        list_users_operation.clone(),
    );

    let update_user_operation = domain::operations::Operation::Invoke(
        domain::resources::Resource::Endpoint(
            "users".to_string(),
            "update_users".to_string(),
        ),
    );
    let update_user_permission = domain::permissions::Permission::new(
        "update user",
        update_user_operation.clone(),
    );

    let mut engineer_role = domain::roles::Role::new("engineer");
    engineer_role.add_permission(list_users_permission.clone());
    engineer_role.add_permission(update_user_permission.clone());
    for permission in engineer_role.get_permissions() {
        info!("{:?}", permission)
    }

    let mut john = domain::subjects::Subject::new("john");
    john.add_role(engineer_role);

    let ok = access_checker::AccessChecker::can_subject_perform_operation(&john, &list_users_operation);
    info!("ok: {:?}", ok);

    let connection_pool = SqlitePool::connect(":memory").await?;
    let mut connection = connection_pool.acquire().await?;
    sqlx::migrate!("./datastore/sqlite").run(&mut connection).await.expect("unable to migrate");
    
    let permission_repository = SqlitePermissionRepository::new(connection_pool.clone());
    permission_repository.save(list_users_permission.clone()).await?;
    permission_repository.save(update_user_permission.clone()).await?;

    info!("{:?}", permission_repository.get_by_id(list_users_permission.get_id()).await?.unwrap());

    let subject_repository = SqliteSubjectRepository::new(connection_pool.clone());
    let john_wick = domain::subjects::Subject::new("john wick");
    let baba_yaga = domain::subjects::Subject::new("baba yaga");
    subject_repository.save(john_wick.clone()).await?;
    subject_repository.save(baba_yaga.clone()).await?;

    info!("{:?}", subject_repository.get_by_id(john_wick.get_id()).await?.unwrap());
    info!("{:?}", subject_repository.get_by_id(baba_yaga.get_id()).await?.unwrap());

    // let mut group = groups::Group::new("engineering_team");
    // group.add_subject(subjects::Subject::new("homer").get_id());
    // group.add_subject(subjects::Subject::new("marge").get_id());
    // group.add_subject(subjects::Subject::new("maggie").get_id());
    // group.add_subject(subjects::Subject::new("bart").get_id());
    // group.add_subject(subjects::Subject::new("lisa").get_id());
    

    Ok(())
}