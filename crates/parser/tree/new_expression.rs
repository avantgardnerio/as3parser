use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    pub arguments: Option<Vec<Rc<Expression>>>,
}