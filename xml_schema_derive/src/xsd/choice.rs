use crate::xsd::annotation::Annotation;
use crate::xsd::element::Element;
use crate::xsd::max_occurences::MaxOccurences;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "choice",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Choice {
  #[yaserde(rename = "element")]
  pub elements: Vec<Element>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
  #[yaserde(rename = "minOccurs", attribute)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename = "maxOccurs", attribute)]
  pub max_occurences: Option<MaxOccurences>,
}
