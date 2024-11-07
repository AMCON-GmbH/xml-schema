use crate::xsd::annotation::Annotation;
use crate::xsd::simple_type::SimpleType;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "attribute",
  prefix = "xs",
  namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Attribute {
  #[yaserde(prefix = "xs", attribute = true)]
  pub name: Option<String>,
  #[yaserde(rename = "type", attribute = true)]
  pub r#type: Option<String>,
  #[yaserde(rename = "use", attribute = true)]
  pub required: Required,
  #[yaserde(rename = "ref", attribute = true)]
  pub refers: Option<String>,
  #[yaserde(rename = "simpleType", prefix = "xs")]
  pub simple_type: Option<SimpleType>,
  #[yaserde(rename = "annotation", prefix = "xs")]
  pub annotation: Option<Annotation>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize)]
pub enum Required {
  #[default]
  #[yaserde(rename = "optional")]
  Optional,
  #[yaserde(rename = "required")]
  Required,
}
