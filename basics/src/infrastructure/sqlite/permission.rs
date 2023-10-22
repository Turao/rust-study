use async_trait::async_trait;
use sqlx::{Sqlite, FromRow};
use sqlx::pool::Pool;

use chrono::{Utc, TimeZone};

use crate::domain::permissions::{PermissionId, Permission};
use crate::domain::repositories::{Error, Repository};

#[derive(Debug, FromRow)]
struct SqlitePermissionRepositoryModel {
    id: String,
    name: String,
    operation: String,
    created_at: i64,
    updated_at: i64,
}

impl From<Permission> for SqlitePermissionRepositoryModel {
    fn from(value: Permission) -> Self {      
        Self {
            id: value.get_id().into(),
            name: value.get_name(),
            operation: serde_json::to_string(&value.get_operation()).unwrap(),
            created_at: value.get_created_at().timestamp_millis(),
            updated_at: value.get_updated_at().timestamp_millis(),
        }
    }
}

impl From<SqlitePermissionRepositoryModel> for Permission {
    fn from(value: SqlitePermissionRepositoryModel) -> Self {
        Permission::builder()
            .id(PermissionId::from(value.id))
            .name(value.name)
            .operation(serde_json::from_str(&value.operation).unwrap())
            .created_at(Utc.timestamp_millis_opt(value.created_at).single().unwrap_or_default())
            .updated_at(Utc.timestamp_millis_opt(value.updated_at).single().unwrap_or_default())
            .build()
    }
}

#[derive(Debug)]
pub struct SqlitePermissionRepository {
    connection_pool: Pool<Sqlite>
}

impl SqlitePermissionRepository {
    pub fn new(connection_pool: Pool<Sqlite>) -> SqlitePermissionRepository {
        SqlitePermissionRepository {
            connection_pool
        }
    }
}

#[async_trait]
impl Repository<PermissionId, Permission> for SqlitePermissionRepository {
    async fn get_by_id(&self, id: PermissionId) -> Result<Option<Permission>, Error> {
        let mut connection = self.connection_pool.acquire().await?;
        let query = "SELECT * FROM permissions WHERE id = ?";
        let permission = sqlx::query_as::<_, SqlitePermissionRepositoryModel>(query)
            .bind::<String>(id.into()) // todo: map to model PK first
            .fetch_optional(&mut *connection).await?
            .map(Permission::from);
        Ok(permission)
    }

    async fn save(&self, entity: Permission) -> Result<(), Error> {
        let model = SqlitePermissionRepositoryModel::from(entity);
        let mut connection = self.connection_pool.acquire().await?;
        let query = "INSERT INTO permissions VALUES(?, ?, ?, ?, ?);";
        sqlx::query(query)
            .bind(model.id)
            .bind(model.name)
            .bind(model.operation)
            .bind(model.created_at)
            .bind(model.updated_at)
            .execute(&mut *connection).await?;
        Ok(())
    }
}