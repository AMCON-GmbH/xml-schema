use crate::xsd::annotation::Annotation;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" })]
pub struct Restriction {
  #[yaserde(rename = "base", attribute = true)]
  pub base: Option<String>,
  #[yaserde(rename = "maxLength", prefix = "xs")]
  pub max_length: Option<NumericRestrictionValue>,
  #[yaserde(rename = "minLength", prefix = "xs")]
  pub min_length: Option<NumericRestrictionValue>,
  #[yaserde(rename = "length", prefix = "xs")]
  pub length: Option<NumericRestrictionValue>,
  #[yaserde(rename = "minExclusive", prefix = "xs")]
  pub min_exclusive: Option<NumericRestrictionValue>,
  #[yaserde(rename = "maxExclusive", prefix = "xs")]
  pub max_exclusive: Option<NumericRestrictionValue>,
  #[yaserde(rename = "minInclusive", prefix = "xs")]
  pub min_inclusive: Option<NumericRestrictionValue>,
  #[yaserde(rename = "maxInclusive", prefix = "xs")]
  pub max_inclusive: Option<NumericRestrictionValue>,
  #[yaserde(rename = "totalDigits", prefix = "xs")]
  pub total_digits: Option<NumericRestrictionValue>,
  #[yaserde(rename = "pattern", prefix = "xs")]
  pub patterns: Vec<StringRestrictionValue>,
  #[yaserde(rename = "enumeration", prefix = "xs")]
  pub enumerations: Vec<EnumRestrictionValue>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" })]
pub struct NumericRestrictionValue {
  #[yaserde(rename = "value", attribute = true)]
  pub value: u64,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" })]
pub struct StringRestrictionValue {
  #[yaserde(rename = "value", attribute = true)]
  pub value: String,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = { "xs" = "http://www.w3.org/2001/XMLSchema" })]
pub struct EnumRestrictionValue {
  #[yaserde(rename = "value", attribute = true)]
  pub value: String,
  #[yaserde(rename = "annotation", prefix = "xs")]
  pub annotation: Option<Annotation>,
}
