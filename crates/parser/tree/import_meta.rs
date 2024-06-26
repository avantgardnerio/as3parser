use crate::ns::*;
use serde::{Serialize, Deserialize};

/// The `import.meta` expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportMeta {
    pub location: Location,
}