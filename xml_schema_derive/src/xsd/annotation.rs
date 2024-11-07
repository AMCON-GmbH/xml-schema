use crate::xsd::attribute::Attribute;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "annotation",
  prefix = "xs",
  namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Annotation {
  #[yaserde(attribute = true)]
  pub id: Option<String>,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(rename = "documentation", prefix = "xs")]
  pub documentation: Option<String>,
  #[yaserde(rename = "appinfo", prefix = "xs")]
  pub app_info: Option<AppInfo>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "appinfo",
  prefix = "xs",
  namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema", "meta" = "urn:ets:metainfo" }
)]
pub struct AppInfo {
  #[yaserde(rename = "metaInfo", prefix = "meta")]
  pub meta_info: Option<MetaInfo>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "metaInfo",
  prefix = "meta",
  namespaces = { "meta" = "urn:ets:metainfo" }
)]
pub struct MetaInfo {
  #[yaserde(rename = "xml", prefix = "meta")]
  pub xml: Option<Xml>,
  #[yaserde(rename = "binary", prefix = "meta")]
  pub binary: Option<Binary>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "metaInfo",
  prefix = "meta",
  namespaces = { "meta" = "urn:ets:metainfo" }
)]
pub struct Binary {
  #[yaserde(rename = "asn1tag", prefix = "meta")]
  pub asn1_tag: Option<String>,
  #[yaserde(rename = "asn1Identifier", prefix = "meta")]
  pub asn1_identifier: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(rename = "xml", prefix = "meta", namespaces = { "meta" = "urn:ets:metainfo" })]
pub struct Xml {
  #[yaserde(rename = "old", prefix = "meta")]
  pub old: Vec<Old>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(rename = "old", prefix = "meta", namespaces = { "meta" = "urn:ets:metainfo" })]
pub struct Old {
  #[yaserde(rename = "name", prefix = "meta")]
  pub name: String,
}
