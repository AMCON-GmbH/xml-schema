use crate::xsd::{attribute::Attribute, group::Group, sequence::Sequence};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "extension",
  prefix = "xs",
  namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Extension {
  #[yaserde(attribute = true)]
  pub base: String,
  #[yaserde(rename = "attribute", prefix = "xs")]
  pub attributes: Vec<Attribute>,
  #[yaserde(rename = "sequence", prefix = "xs")]
  pub sequences: Vec<Sequence>,
  #[yaserde(rename = "group", prefix = "xs")]
  pub group: Option<Group>,
}
