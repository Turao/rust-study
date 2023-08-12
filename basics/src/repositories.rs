use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::{Sqlite, Error, FromRow};
use sqlx::pool::Pool;

use chrono::{Utc, TimeZone};

use crate::permissions::{PermissionId, Permission};
use crate::subjects::{SubjectId, Subject};

#[async_trait]
pub trait Repository<Id, Entity> {
    async fn get_by_id(&self, id: Id) -> Result<Option<Entity>, Error>;
    async fn save(&self, entity: Entity) -> Result<(), Error>;
}


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
            operation: "".to_string(),
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
            .operation(crate::operations::Operation::Invoke(crate::resources::Resource::Service("test".to_string())))
            .created_at(Utc.timestamp_millis_opt(value.created_at).single().unwrap_or_default())
            .updated_at(Utc.timestamp_millis_opt(value.updated_at).single().unwrap_or_default())
            .build()
    }
}

pub struct SqliteSubjectRepository {
    connection_pool: Pool<Sqlite>
}

impl SqliteSubjectRepository {
    pub fn new(connection_pool: Pool<Sqlite>) -> SqliteSubjectRepository {
        SqliteSubjectRepository {
            connection_pool
        }
    }
}

#[async_trait]
impl Repository<SubjectId, Subject> for SqliteSubjectRepository {
    async fn save(&self, entity: Subject) -> Result<(), Error> {
        let model = SqliteSubjectModel::from(entity);
        let mut connection = self.connection_pool.acquire().await?;
        let query = "INSERT INTO subjects VALUES (?, ?, ?, ?, ?);";
        sqlx::query(query)
            .bind(model.id)
            .bind(model.name)
            .bind(model.roles)
            .bind(model.created_at)
            .bind(model.updated_at)
            .execute(&mut *connection).await?;
        Ok(())
    }
    
    async fn get_by_id(&self, id: SubjectId) -> Result<Option<Subject>, Error> {
        let mut connection = self.connection_pool.acquire().await?;
        let query = "SELECT * FROM subjects WHERE id = ?;";
        let subject = sqlx::query_as::<_, SqliteSubjectModel>(query)
            .bind::<String>(id.into()) // todo: map to model PK first
            .fetch_optional(&mut *connection).await?
            .map(Subject::from);
        Ok(subject)
    }
}

#[derive(Debug, FromRow)]
struct SqliteSubjectModel {
    id: String,
    name: String,
    roles: String,
    created_at: i64,
    updated_at: i64,
}

impl From<Subject> for SqliteSubjectModel {
    fn from(value: Subject) -> Self {
        Self {
            id: value.get_id().into(),
            name: value.get_name(),
            roles: "".to_string(),
            created_at: value.get_created_at().timestamp_millis(),
            updated_at: value.get_updated_at().timestamp_millis()
        }
    }
}

impl From<SqliteSubjectModel> for Subject {
    fn from(value: SqliteSubjectModel) -> Self {
        Subject::builder()
            .id(value.id.into())
            .name(value.name)
            .roles(HashMap::new())
            .created_at(Utc.timestamp_millis_opt(value.created_at).single().unwrap_or_default())
            .updated_at(Utc.timestamp_millis_opt(value.updated_at).single().unwrap_or_default())
            .build()
    }
}