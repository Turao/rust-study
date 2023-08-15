use async_trait::async_trait;

#[derive(Debug)]
pub enum RepositoryError {
    Simple(String)
}

#[async_trait]
pub trait Repository<Id, Entity> {
    async fn get_by_id(&self, id: Id) -> Result<Option<Entity>, RepositoryError>;
    async fn save(&self, entity: Entity) -> Result<(), RepositoryError>;
}