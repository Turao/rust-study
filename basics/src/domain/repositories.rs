use async_trait::async_trait;

#[derive(Debug)]
pub enum Error {
    Simple(String)
}

#[async_trait]
pub trait Repository<Id, Entity> {
    async fn get_by_id(&self, id: Id) -> Result<Option<Entity>, Error>;
    async fn save(&self, entity: Entity) -> Result<(), Error>;
}