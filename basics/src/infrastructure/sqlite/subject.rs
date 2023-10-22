use async_trait::async_trait;
use sqlx::{Sqlite, FromRow};
use sqlx::pool::Pool;

use chrono::{Utc, TimeZone};

use crate::domain::repositories::{Error, Repository};
use crate::domain::subjects::{SubjectId, Subject};

#[derive(Debug, FromRow)]
struct SqliteSubjectModel {
    id: String,
    version: i64,
    name: String,
    roles: String,
    created_at: i64,
    updated_at: i64,
}

impl From<Subject> for SqliteSubjectModel {
    fn from(value: Subject) -> Self {
        Self {
            id: value.get_id().into(),
            version: value.get_version(),
            name: value.get_name(),
            roles: serde_json::to_string(&value.get_roles()).unwrap(),
            created_at: value.get_created_at().timestamp_millis(),
            updated_at: value.get_updated_at().timestamp_millis()
        }
    }
}

impl From<SqliteSubjectModel> for Subject {
    fn from(value: SqliteSubjectModel) -> Self {
        Subject::builder()
            .id(value.id.into())
            .version(value.version)
            .name(value.name)
            .roles(serde_json::from_str(&value.roles).unwrap())
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
        let query = "
            INSERT INTO subjects VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
            version=?, name=?, roles=?, created_at=?, updated_at=?;
        ";
        sqlx::query(query)
            // insert
            .bind(model.id)
            .bind(model.version)
            .bind(model.name.clone())
            .bind(model.roles.clone())
            .bind(model.created_at)
            .bind(model.updated_at)
            // update
            .bind(model.version)
            .bind(model.name.clone())
            .bind(model.roles.clone())
            .bind(model.created_at)
            .bind(model.updated_at)
            .execute(&mut *connection).await?;
        Ok(())
    }
    
    async fn get_by_id(&self, id: SubjectId) -> Result<Option<Subject>, Error> {
        let mut connection = self.connection_pool.acquire().await?;
        let query = "SELECT * FROM subjects WHERE id = ? ORDER BY version DESC LIMIT 1;";
        let subject = sqlx::query_as::<_, SqliteSubjectModel>(query)
            .bind::<String>(id.into()) // todo: map to model PK first
            .fetch_optional(&mut *connection).await?
            .map(Subject::from);
        Ok(subject)
    }
}
