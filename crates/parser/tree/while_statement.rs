use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileStatement {
    pub location: Location,
    pub test: Rc<Expression>,
    pub body: Rc<Directive>,
}