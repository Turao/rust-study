use async_trait::async_trait;
use sqlx::{Sqlite, FromRow};
use sqlx::pool::Pool;

use chrono::{Utc, TimeZone};

use crate::domain::roles::{RoleId, Role};
use crate::domain::repositories::{Error, Repository};

#[derive(Debug, FromRow)]
struct SqliteRoleRepositoryModel {
    id: String,
    name: String,
    permissions: String,
    created_at: i64,
    updated_at: i64,
}

impl From<Role> for SqliteRoleRepositoryModel {
    fn from(value: Role) -> Self {      
        Self {
            id: value.get_id().into(),
            name: value.get_name(),
            permissions: serde_json::to_string(&value.get_permissions()).unwrap(),
            created_at: value.get_created_at().timestamp_millis(),
            updated_at: value.get_updated_at().timestamp_millis(),
        }
    }
}

impl From<SqliteRoleRepositoryModel> for Role {
    fn from(value: SqliteRoleRepositoryModel) -> Self {
        Role::builder()
            .id(RoleId::from(value.id))
            .name(value.name)
            .permissions(serde_json::from_str(&value.permissions).unwrap())
            .created_at(Utc.timestamp_millis_opt(value.created_at).single().unwrap_or_default())
            .updated_at(Utc.timestamp_millis_opt(value.updated_at).single().unwrap_or_default())
            .build()
    }
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
        let query = "SELECT * FROM roles WHERE id = ?";
        let role = sqlx::query_as::<_, SqliteRoleRepositoryModel>(query)
            .bind::<String>(id.into()) // todo: map to model PK first
            .fetch_optional(&mut *connection).await?
            .map(Role::from);
        Ok(role)
    }

    async fn save(&self, entity: Role) -> Result<(), Error> {
        let model = SqliteRoleRepositoryModel::from(entity);
        let mut connection = self.connection_pool.acquire().await?;
        let query = "INSERT INTO roles VALUES(?, ?, ?, ?, ?);";
        sqlx::query(query)
            .bind(model.id)
            .bind(model.name)
            .bind(model.permissions)
            .bind(model.created_at)
            .bind(model.updated_at)
            .execute(&mut *connection).await?;
        Ok(())
    }
}