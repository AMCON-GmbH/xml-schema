#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" })]
pub struct List {
  #[yaserde(rename = "itemType", attribute = true)]
  pub item_type: String,
}
