use crate::xsd::choice::Choice;
use crate::xsd::element::Element;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Sequence {
  #[yaserde(rename = "element")]
  pub elements: Vec<Element>,
  #[yaserde(rename = "choice")]
  pub choices: Vec<Choice>
}
