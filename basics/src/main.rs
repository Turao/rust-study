use crate::repositories::{SqlitePermissionRepository, Repository, SqliteSubjectRepository};
use sqlx::sqlite::SqlitePool;
use sqlx::Result;


mod access_checker;
mod groups;
mod operations;
mod permissions;
mod repositories;
mod resources;
mod roles;
mod subjects;

#[async_std::main]
async fn main() -> Result<()> {
    let list_users_operation = operations::Operation::Invoke(
        resources::Resource::Endpoint(
            "users".to_string(),
            "get_users".to_string(),
        ),
    );
    let list_users_permission = permissions::Permission::new(
        "list users",
        list_users_operation.clone(),
    );

    let update_user_operation = operations::Operation::Invoke(
        resources::Resource::Endpoint(
            "users".to_string(),
            "update_users".to_string(),
        ),
    );
    let update_user_permission = permissions::Permission::new(
        "update user",
        update_user_operation.clone(),
    );

    let mut engineer_role = roles::Role::new("engineer");
    engineer_role.add_permission(list_users_permission.clone());
    engineer_role.add_permission(update_user_permission.clone());
    for permission in engineer_role.get_permissions() {
        println!("{:?}", permission)
    }

    let mut john = subjects::Subject::new("john");
    john.add_role(engineer_role);

    let ok = access_checker::AccessChecker::can_subject_perform_operation(&john, &list_users_operation);
    println!("ok: {:?}", ok);

    let connection_pool = SqlitePool::connect(":memory").await?;
    let mut connection = connection_pool.acquire().await?;
    sqlx::migrate!("./datastore").run(&mut connection).await?;
    
    let permission_repository = SqlitePermissionRepository::new(connection_pool.clone());
    permission_repository.save(list_users_permission.clone()).await?;
    permission_repository.save(update_user_permission.clone()).await?;

    println!("{:?}", permission_repository.get_by_id(list_users_permission.get_id()).await?.unwrap());

    let subject_repository = SqliteSubjectRepository::new(connection_pool.clone());
    let john_wick = subjects::Subject::new("john wick");
    let baba_yaga = subjects::Subject::new("baba yaga");
    subject_repository.save(john_wick.clone()).await?;
    subject_repository.save(baba_yaga.clone()).await?;

    println!("{:?}", subject_repository.get_by_id(john_wick.get_id()).await?.unwrap());
    println!("{:?}", subject_repository.get_by_id(baba_yaga.get_id()).await?.unwrap());


    // let mut group = groups::Group::new("engineering_team");
    // group.add_subject(subjects::Subject::new("homer").get_id());
    // group.add_subject(subjects::Subject::new("marge").get_id());
    // group.add_subject(subjects::Subject::new("maggie").get_id());
    // group.add_subject(subjects::Subject::new("bart").get_id());
    // group.add_subject(subjects::Subject::new("lisa").get_id());
    

    Ok(())
}