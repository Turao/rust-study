use async_trait::async_trait;
use sqlx::{Sqlite, FromRow};
use sqlx::pool::Pool;

use chrono::{Utc, TimeZone};

use crate::domain::repositories::{Error, Repository};
use crate::domain::groups::{GroupId, Group};

#[derive(Debug, FromRow)]
struct SqliteGroupModel {
    id: String,
    name: String,
    subjects: String,
    roles: String,
    created_at: i64,
    updated_at: i64,
}

impl From<Group> for SqliteGroupModel {
    fn from(value: Group) -> Self {
        Self {
            id: value.get_id().into(),
            name: value.get_name(),
            subjects: serde_json::to_string(&value.get_subjects()).unwrap(),
            roles: serde_json::to_string(&value.get_roles()).unwrap(),
            created_at: value.get_created_at().timestamp_millis(),
            updated_at: value.get_updated_at().timestamp_millis()
        }
    }
}

impl From<SqliteGroupModel> for Group {
    fn from(value: SqliteGroupModel) -> Self {
        Group::builder()
            .id(value.id.into())
            .name(value.name)
            .subjects(serde_json::from_str(&value.subjects).unwrap())
            .roles(serde_json::from_str(&value.roles).unwrap())
            .created_at(Utc.timestamp_millis_opt(value.created_at).single().unwrap_or_default())
            .updated_at(Utc.timestamp_millis_opt(value.updated_at).single().unwrap_or_default())
            .build()
    }
}

pub struct SqliteGroupRepository {
    connection_pool: Pool<Sqlite>
}

impl SqliteGroupRepository {
    pub fn new(connection_pool: Pool<Sqlite>) -> SqliteGroupRepository {
        SqliteGroupRepository {
            connection_pool
        }
    }
}

#[async_trait]
impl Repository<GroupId, Group> for SqliteGroupRepository {
    async fn save(&self, entity: Group) -> Result<(), Error> {
        let model = SqliteGroupModel::from(entity);
        let mut connection = self.connection_pool.acquire().await?;
        let query = "
            INSERT INTO groups VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
            name=?, subjects=?, roles=?, created_at=?, updated_at=?;
        ";
        sqlx::query(query)
            // insert
            .bind(model.id)
            .bind(model.name.clone())
            .bind(model.subjects.clone())
            .bind(model.roles.clone())
            .bind(model.created_at)
            .bind(model.updated_at)
            // update
            .bind(model.name.clone())
            .bind(model.subjects)
            .bind(model.roles.clone())
            .bind(model.created_at)
            .bind(model.updated_at)
            .execute(&mut *connection).await?;
        Ok(())
    }
    
    async fn get_by_id(&self, id: GroupId) -> Result<Option<Group>, Error> {
        let mut connection = self.connection_pool.acquire().await?;
        let query = "SELECT * FROM groups WHERE id = ?;";
        let Group = sqlx::query_as::<_, SqliteGroupModel>(query)
            .bind::<String>(id.into()) // todo: map to model PK first
            .fetch_optional(&mut *connection).await?
            .map(Group::from);
        Ok(Group)
    }
}
