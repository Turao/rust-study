use serde::{Serialize, Deserialize};

use super::resources::Resource;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    Invoke(Resource)
}
