use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDirective {
    pub location: Location,
    pub alias: Option<(String, Location)>,
    pub package_name: Vec<(String, Location)>,
    pub import_specifier: ImportSpecifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportSpecifier {
    Wildcard(Location),
    Recursive(Location),
    Identifier((String, Location)),
}

impl ImportSpecifier {
    pub fn location(&self) -> Location {
        match self {
            Self::Wildcard(l) |
            Self::Recursive(l) => l.clone(),
            Self::Identifier((_, l)) => l.clone(),
        }
    }
}