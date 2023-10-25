use std::collections::HashSet;

use async_trait::async_trait;
use sqlx::{Sqlite, FromRow, Connection};
use sqlx::pool::Pool;

use chrono::{Utc, TimeZone};

use crate::domain::permissions::PermissionId;
use crate::domain::roles::{RoleId, Role};
use crate::domain::repositories::{Error, Repository};

#[derive(Debug, FromRow)]
struct SqliteRoleRepositoryModel {
    id: String,
    name: String,
    created_at: i64,
    updated_at: i64,
}

impl From<SqliteRoleRepositoryModel> for Role {
    fn from(value: SqliteRoleRepositoryModel) -> Self {
        Role::builder()
            .id(RoleId::from(value.id))
            .name(value.name)
            .created_at(Utc.timestamp_millis_opt(value.created_at).single().unwrap_or_default())
            .updated_at(Utc.timestamp_millis_opt(value.updated_at).single().unwrap_or_default())
            .build()
    }
}

#[derive(Debug, FromRow)]
struct SqliteRolePermissionRepositoryModel {
    role_id: String,
    permission_id: String,
}

#[derive(Debug)]
pub struct SqliteRoleRepository {
    connection_pool: Pool<Sqlite>
}

impl SqliteRoleRepository {
    pub fn new(connection_pool: Pool<Sqlite>) -> SqliteRoleRepository {
        SqliteRoleRepository {
            connection_pool
        }
    }
}

#[async_trait]
impl Repository<RoleId, Role> for SqliteRoleRepository {
    async fn get_by_id(&self, id: RoleId) -> Result<Option<Role>, Error> {
        let mut connection = self.connection_pool.acquire().await?;
        let mut transaction = connection.begin().await?;
        
        let role_query = "SELECT * FROM roles WHERE id = ?";
        let role_model = sqlx::query_as::<_, SqliteRoleRepositoryModel>(role_query)
            .bind::<String>(id.clone().into()) // todo: map to model PK first
            .fetch_optional(&mut *transaction).await?
            .expect("role not found");
        
        let role_permissions_query = "SELECT * FROM roles_permissions WHERE role_id = ?";
        let role_permission_models = sqlx::query_as::<_, SqliteRolePermissionRepositoryModel>(role_permissions_query)
            .bind::<String>(id.clone().into()) // todo: map to model PK first
            .fetch_all(&mut *transaction).await?;

        let role_permissions = role_permission_models.into_iter()
            .map(|model| PermissionId::from(model.permission_id))
            .collect();

        let role = Role::builder()
            .id(RoleId::from(role_model.id))
            .name(role_model.name)
            .created_at(Utc.timestamp_millis_opt(role_model.created_at).single().unwrap_or_default())
            .updated_at(Utc.timestamp_millis_opt(role_model.updated_at).single().unwrap_or_default())
            .permissions(role_permissions)
            .build();


        Ok(Some(role))
    }

    async fn save(&self, entity: Role) -> Result<(), Error> {
        let mut connection = self.connection_pool.acquire().await?;
        let mut transaction = connection.begin().await?;

        let model = SqliteRoleRepositoryModel {
            id: entity.get_id().into(),
            name: entity.get_name(),
            created_at: entity.get_created_at().timestamp_millis(),
            updated_at: entity.get_updated_at().timestamp_millis(),
        };
        let role_query = "INSERT INTO roles VALUES(?, ?, ?, ?);";
        sqlx::query(role_query)
            .bind(model.id)
            .bind(model.name)
            .bind(model.created_at)
            .bind(model.updated_at)
            .execute(&mut *transaction).await?;

        let role_permissions: Vec<SqliteRolePermissionRepositoryModel> = entity.get_permissions()
            .into_iter()
            .map(|permission_id| SqliteRolePermissionRepositoryModel{
                role_id: entity.get_id().into(),
                permission_id: permission_id.into(),
            })
            .collect();

        let role_permissions_query = "INSERT INTO roles_permissions VALUES(?, ?);";
        for model in role_permissions {
            sqlx::query(role_permissions_query)
                .bind(model.role_id)
                .bind(model.permission_id)
                .execute(&mut *transaction).await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}