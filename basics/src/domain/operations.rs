use super::resources::Resource;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operation {
    Invoke(Resource)
}
