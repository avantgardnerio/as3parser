use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyStatement {
    pub location: Location,
}