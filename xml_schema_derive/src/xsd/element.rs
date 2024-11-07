use crate::xsd::{
  annotation::Annotation, complex_type::ComplexType, max_occurences::MaxOccurences,
  simple_type::SimpleType,
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "element",
  prefix = "xs",
  namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Element {
  #[yaserde(attribute = true)]
  pub name: Option<String>,
  #[yaserde(rename = "type", attribute = true)]
  pub r#type: Option<String>,
  #[yaserde(rename = "ref", attribute = true)]
  pub refers: Option<String>,
  #[yaserde(rename = "minOccurs", attribute = true)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename = "maxOccurs", attribute = true)]
  pub max_occurences: Option<MaxOccurences>,
  #[yaserde(rename = "complexType", prefix = "xs")]
  pub complex_type: Option<ComplexType>,
  #[yaserde(rename = "simpleType", prefix = "xs")]
  pub simple_type: Option<SimpleType>,
  #[yaserde(rename = "annotation", prefix = "xs")]
  pub annotation: Option<Annotation>,
}
