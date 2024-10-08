use crate::xsd::attribute::Attribute;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(rename = "annotation"
prefix = "xs",
namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Annotation {
  #[yaserde(attribute)]
  pub id: Option<String>,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(rename = "documentation"
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema")]
  pub documentation: Option<String>,
  #[yaserde(rename = "appinfo")]
  pub app_info: Option<AppInfo>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(rename = "appinfo"
prefix = "xs",
namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct AppInfo {
  #[yaserde(rename = "metaInfo")]
  pub meta_info: Option<MetaInfo>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(rename = "metaInfo"
prefix = "meta",
namespace = "meta: urn:ets:metainfo")]
pub struct MetaInfo {
  #[yaserde(rename = "xml")]
  pub xml: Option<Xml>,
  #[yaserde(rename = "binary")]
  pub binary: Option<Binary>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(rename = "metaInfo"
prefix = "meta",
namespace = "meta: urn:ets:metainfo")]
pub struct Binary {
  #[yaserde(rename = "asn1tag"
  prefix = "meta",
  namespace = "meta: urn:ets:metainfo")]
  pub asn1_tag: Option<String>,
  #[yaserde(rename = "asn1Identifier"
  prefix = "meta",
  namespace = "meta: urn:ets:metainfo")]
  pub asn1_identifier: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(rename = "xml"
prefix = "meta",
namespace = "meta: urn:ets:metainfo")]
pub struct Xml {
  #[yaserde(rename = "old")]
  pub old: Vec<Old>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(rename = "old"
prefix = "meta",
namespace = "meta: urn:ets:metainfo")]
pub struct Old {
  #[yaserde(rename = "name"
    prefix = "meta",
    namespace = "meta: urn:ets:metainfo")]
  pub name: String,
}
