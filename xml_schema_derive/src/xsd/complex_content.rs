use crate::xsd::extension::Extension;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" })]
pub struct ComplexContent {
  #[yaserde(prefix = "xs")]
  pub extension: Option<Extension>,
}
