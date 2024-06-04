use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlExpression {
    pub location: Location,
    pub element: Rc<XmlElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlMarkupExpression {
    pub location: Location,
    pub markup: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlListExpression {
    pub location: Location,
    pub content: Vec<Rc<XmlContent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlElement {
    pub location: Location,
    pub name: XmlTagName,
    pub attributes: Vec<Rc<XmlAttribute>>,
    pub attribute_expression: Option<Rc<Expression>>,
    pub content: Option<Vec<Rc<XmlContent>>>,
    pub closing_name: Option<XmlTagName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum XmlTagName {
    Name((String, Location)),
    Expression(Rc<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlAttribute {
    pub location: Location,
    pub name: (String, Location),
    pub value: XmlAttributeValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum XmlAttributeValue {
    Value((String, Location)),
    Expression(Rc<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum XmlContent {
    Characters((String, Location)),
    Markup((String, Location)),
    Element(Rc<XmlElement>),
    Expression(Rc<Expression>),
}