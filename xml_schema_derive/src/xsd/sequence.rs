use crate::xsd::choice::Choice;
use crate::xsd::element::Element;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" })]
pub struct Sequence {
  #[yaserde(rename = "element", prefix = "xs")]
  pub elements: Vec<Element>,
  #[yaserde(rename = "choice", prefix = "xs")]
  pub choices: Vec<Choice>
}
