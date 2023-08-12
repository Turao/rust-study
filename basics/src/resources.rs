#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Resource {
    Service(String),
    Endpoint(String, String),
}