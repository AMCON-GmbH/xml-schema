use crate::xsd::annotation::Annotation;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Restriction {
  #[yaserde(rename = "base", attribute)]
  pub base: Option<String>,
  #[yaserde(rename = "maxLength")]
  pub max_length: Option<NumericRestrictionValue>,
  #[yaserde(rename = "minLength")]
  pub min_length: Option<NumericRestrictionValue>,
  #[yaserde(rename = "length")]
  pub length: Option<NumericRestrictionValue>,
  #[yaserde(rename = "minExclusive")]
  pub min_exclusive: Option<NumericRestrictionValue>,
  #[yaserde(rename = "maxExclusive")]
  pub max_exclusive: Option<NumericRestrictionValue>,
  #[yaserde(rename = "minInclusive")]
  pub min_inclusive: Option<NumericRestrictionValue>,
  #[yaserde(rename = "maxInclusive")]
  pub max_inclusive: Option<NumericRestrictionValue>,
  #[yaserde(rename = "totalDigits")]
  pub total_digits: Option<NumericRestrictionValue>,
  #[yaserde(rename = "pattern")]
  pub patterns: Vec<StringRestrictionValue>,
  #[yaserde(rename = "enumeration")]
  pub enumerations: Vec<EnumRestrictionValue>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct NumericRestrictionValue {
  #[yaserde(rename = "value", attribute)]
  pub value: u64,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct StringRestrictionValue {
  #[yaserde(rename = "value", attribute)]
  pub value: String,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct EnumRestrictionValue {
  #[yaserde(rename = "value", attribute)]
  pub value: String,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
}
