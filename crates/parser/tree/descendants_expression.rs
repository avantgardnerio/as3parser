use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescendantsExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    pub identifier: QualifiedIdentifier,
}