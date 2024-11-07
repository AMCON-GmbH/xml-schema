use crate::xsd::annotation::Annotation;
use crate::xsd::element::Element;
use crate::xsd::max_occurences::MaxOccurences;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "choice",
  prefix = "xs",
  namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Choice {
  #[yaserde(rename = "element", prefix = "xs")]
  pub elements: Vec<Element>,
  #[yaserde(rename = "annotation", prefix = "xs")]
  pub annotation: Option<Annotation>,
  #[yaserde(rename = "minOccurs", attribute = true)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename = "maxOccurs", attribute = true)]
  pub max_occurences: Option<MaxOccurences>,
}
