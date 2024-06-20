use crate::xsd::{
  attribute, attribute_group, complex_type, element, group, import, qualification, simple_type,
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(root = "schema"
prefix = "xs",
namespace = "xs: http://www.w3.org/2001/XMLSchema",)]
pub struct Schema {
  #[yaserde(rename = "targetNamespace", attribute)]
  pub target_namespace: Option<String>,
  #[yaserde(rename = "elementFormDefault", attribute)]
  pub element_form_default: Option<qualification::Qualification>,
  #[yaserde(rename = "attributeFormDefault", attribute)]
  pub attribute_form_default: Option<qualification::Qualification>,
  #[yaserde(rename = "import")]
  pub imports: Vec<import::Import>,
  #[yaserde(rename = "element")]
  pub elements: Vec<element::Element>,
  #[yaserde(rename = "simpleType")]
  pub simple_type: Vec<simple_type::SimpleType>,
  #[yaserde(rename = "complexType")]
  pub complex_type: Vec<complex_type::ComplexType>,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<attribute::Attribute>,
  #[yaserde(rename = "attributeGroup")]
  pub attribute_group: Vec<attribute_group::AttributeGroup>,
  #[yaserde(rename = "group")]
  pub group: Vec<group::Group>,
}

#[cfg(test)]
mod tests {
  use crate::xsd::schema::Schema;
  use std::collections::HashSet;
  use std::fs;

  #[test]
  fn de_schema_should_contain_all_simple_types_from_xsd() {
    // given
    let xsd = fs::read_to_string("fixtures/common.xsd").unwrap();

    let expected_simple_type_names = HashSet::from([
      "ZonedDateTime",
      "ZonedDate",
      "ListCycleNumber",
      "Checksum",
      "BlockingReasonText",
      "MediumId",
    ]);

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    assert!(schema
      .simple_type
      .iter()
      .all(|x| expected_simple_type_names.contains(x.name.as_str())));
  }

  #[test]
  fn de_schema_should_contain_all_complex_types_from_xsd() {
    // given
    let xsd = fs::read_to_string("fixtures/common.xsd").unwrap();

    let expected_complex_type_names = HashSet::from([
      "OrganisationalUnit",
      "SymmetricKeyId",
      "ListCycle",
      "IncrementalListCycleInformation",
      "ListVerificationInformation",
      "SamHotlistingDemand",
      "BlockingReason",
      "CaCertificateRepository",
      "CvCertificateRevocationListEntry",
      "CvCertificateRevocationList",
      "InternalError",
      "EntitlementIssuingAbortedData",
      "ActionAbortedData",
      "SignedEntitlementIssuedAttestation",
      "SignedEntitlementTerminatedAttestation",
      "SignedEntitlementBlockedAttestation",
      "SignedEntitlementUnblockedAttestation",
      "SignedEntitlementInspectedAttestation",
      "SignedEntitlementValidatedAttestation",
      "SignedCheckinAttestation",
      "SignedCheckoutAttestation",
      "SignedUserTariffParametersChangedAttestation",
      "SignedStoredValuePaymentMethodRechargedAttestation",
      "SignedStoredValuePaymentMethodReimbursedAttestation",
      "SignedStoredValuePaymentMethodDebitedAttestation",
      "SignedStoredValuePaymentMethodCreditedAttestation",
      "SignedAccountBasedPaymentMethodDebitedAttestation",
      "SignedAccountBasedPaymentMethodCreditedAttestation",
      "SignedApplicationBlockedAttestation",
      "SignedApplicationUnblockedAttestation",
      "SignedApplicationTerminatedAttestation",
      "Price",
    ]);

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    assert!(schema
      .complex_type
      .iter()
      .all(|x| expected_complex_type_names.contains(x.name.as_str())));
  }
  //endregion
}
