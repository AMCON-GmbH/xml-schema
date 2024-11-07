use crate::xsd::sequence::Sequence;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" })]
pub struct Group {
  #[yaserde(attribute = true)]
  pub name: Option<String>,
  #[yaserde(attribute = true, rename = "ref")]
  pub reference: Option<String>,
  #[yaserde(prefix = "xs")]
  pub sequence: Option<Sequence>,
}
