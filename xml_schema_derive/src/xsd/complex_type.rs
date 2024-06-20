use crate::xsd::{
  annotation::Annotation, attribute::Attribute, complex_content::ComplexContent,
  sequence::Sequence, simple_content::SimpleContent,
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(rename = "complexType"
prefix = "xs",
namespace = " http://www.w3.org/2001/XMLSchema")]
pub struct ComplexType {
  #[yaserde(attribute)]
  pub name: String,
  #[yaserde(attribute)]
  pub id: String,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  pub sequence: Option<Sequence>,
  #[yaserde(rename = "simpleContent")]
  pub simple_content: Option<SimpleContent>,
  #[yaserde(rename = "complexContent")]
  pub complex_content: Option<ComplexContent>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
}

/// Tests if complex types are deserialized correctly from xsd files.
/// Each test deserializes the xsd to Schema and then finds the complex type to test, in contrast to directly deserializing ComplexType.
/// That is because isolated ComplexTypes do not exist and namespaces can only be checked in the context of a Schema.
#[cfg(test)]
mod tests {
  use crate::xsd::complex_type::ComplexType;
  use crate::xsd::element::Element;
  use crate::xsd::schema::Schema;
  use crate::xsd::sequence::Sequence;
  use std::fs;
  use crate::xsd::annotation::Annotation;

  fn get_complex_type<'a>(schema: &'a Schema, type_name: &str) -> &'a ComplexType {
    schema
      .complex_type
      .iter()
      .find(|x| x.name == type_name)
      .unwrap()
  }

  #[test]
  fn de_complex_type_with_sequence() {
    // given
    let xsd = fs::read_to_string("fixtures/common.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let org_unit = get_complex_type(&schema, "OrganisationalUnit");

    assert_eq!(org_unit.id, "_6c807dfd-b979-4be9-becf-4b798a7d7bcb");
    assert_eq!(org_unit.name, "OrganisationalUnit");
    assert_eq!(org_unit.complex_content, None);
    assert_eq!(org_unit.attributes, vec![]);
    assert_eq!(org_unit.simple_content, None);

    Annotation::assert_since(
      org_unit.annotation.as_ref().unwrap(),
      "Combination of organisation and its role in the IFM context.",
      "3.0.0",
    );

    assert_eq!(
      org_unit.sequence,
      Some(Sequence {
        elements: vec![
          Element {
            name: Some(String::from("organisationId")),
            r#type: Some(String::from("tns:OrganisationId")),
            ..Default::default()
          },
          Element {
            name: Some(String::from("role")),
            r#type: Some(String::from("tns:PartnerRoleCode")),
            ..Default::default()
          }
        ]
      })
    );
  }
}
