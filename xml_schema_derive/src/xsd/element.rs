use crate::xsd::{
  annotation::Annotation, complex_type::ComplexType, max_occurences::MaxOccurences,
  simple_type::SimpleType,
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "element",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Element {
  #[yaserde(attribute)]
  pub name: Option<String>,
  #[yaserde(rename = "type", attribute)]
  pub r#type: Option<String>,
  #[yaserde(rename = "ref", attribute)]
  pub refers: Option<String>,
  #[yaserde(rename = "minOccurs", attribute)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename = "maxOccurs", attribute)]
  pub max_occurences: Option<MaxOccurences>,
  #[yaserde(rename = "complexType")]
  pub complex_type: Option<ComplexType>,
  #[yaserde(rename = "simpleType")]
  pub simple_type: Option<SimpleType>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
}
