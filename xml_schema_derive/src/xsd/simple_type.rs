use crate::xsd::{list::List, restriction::Restriction, union::Union};
use crate::xsd::annotation::Annotation;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleType {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute)]
    pub id: String,
    pub annotation: Option<Annotation>,
    pub restriction: Option<Restriction>,
    pub list: Option<List>,
    pub union: Option<Union>,
}