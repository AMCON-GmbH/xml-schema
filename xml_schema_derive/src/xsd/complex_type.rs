use crate::xsd::choice::Choice;
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
  #[yaserde(rename = "sequence")]
  pub sequence: Option<Sequence>,
  #[yaserde(rename = "choice")]
  pub choice: Option<Choice>,
  #[yaserde(rename = "simpleContent")]
  pub simple_content: Option<SimpleContent>,
  #[yaserde(rename = "complexContent")]
  pub complex_content: Option<ComplexContent>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
  #[yaserde(rename = "abstract", attribute)]
  /// should default to false
  pub is_abstract: Option<bool>,
}

/// Tests if complex types are deserialized correctly from xsd files.
/// Each test deserializes the xsd to Schema and then finds the complex type to test, in contrast to directly deserializing ComplexType.
/// That is because isolated ComplexTypes do not exist and namespaces can only be checked in the context of a Schema.
#[cfg(test)]
mod tests {
  use crate::xsd::annotation::{Annotation, AppInfo, MetaInfo, Old, Xml};
  use crate::xsd::choice::Choice;
  use crate::xsd::complex_type::ComplexType;
  use crate::xsd::element::Element;
  use crate::xsd::schema::Schema;
  use crate::xsd::sequence::Sequence;
  use std::fs;

  use crate::xsd::max_occurences::MaxOccurences;
  use pretty_assertions::assert_eq;

  fn get_complex_type<'a>(schema: &'a Schema, type_name: &str) -> &'a ComplexType {
    schema
      .complex_type
      .iter()
      .find(|x| x.name == type_name)
      .unwrap()
  }

  #[test]
  fn de_type_with_sequence_of_elements_should_be_parsed_correctly() {
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
    assert_eq!(org_unit.is_abstract, None);
    assert_eq!(org_unit.choice, None);

    Annotation::check(
      org_unit.annotation.as_ref().unwrap(),
      "Combination of organisation and its role in the IFM context.",
      None,
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
        ],
        choices: vec![],
      })
    );
  }

  #[test]
  fn de_abstract_type_should_be_parsed_correctly() {
    // given
    let xsd = fs::read_to_string("fixtures/ion.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let org_unit = get_complex_type(&schema, "CompactMessage");

    assert!(org_unit.is_abstract.unwrap());
  }

  #[test]
  fn de_type_with_asn1tag_should_be_parsed_correctly() {
    // given
    let xsd = fs::read_to_string("fixtures/sam-generated.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let po_token = get_complex_type(&schema, "ProductOwnerToken");

    Annotation::check(
      po_token.annotation.as_ref().unwrap(),
      "Indicates that the SAM may issue products for the contained organisation. Also contains a sequence number for the number of issuances already authorised for products of this organisation.",
      Some("0x70"))
  }

  #[test]
  fn de_type_with_choice_should_be_parsed_correctly() {
    // given
    let xsd = fs::read_to_string("fixtures/terminal.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let variant = get_complex_type(&schema, "EntitlementHotlistVariant");

    assert_eq!(
      variant.choice.as_ref().unwrap(),
      &Choice {
        elements: vec![
          Element {
            name: Some(String::from("entitlementHotlist")),
            r#type: Some(String::from("hla:EntitlementHotlist")),
            ..Default::default()
          },
          Element {
            name: Some(String::from("incrementalEntitlementHotlist")),
            r#type: Some(String::from("hla:IncrementalEntitlementHotlist")),
            ..Default::default()
          },
          Element {
            name: Some(String::from("entitlementWithProductHotlist")),
            r#type: Some(String::from("hla:EntitlementWithProductHotlist")),
            ..Default::default()
          }
        ],
        ..Default::default()
      }
    );
  }

  #[test]
  fn de_type_with_sequence_of_elements_and_choices_should_be_parsed_correctly() {
    // given
    let xsd = fs::read_to_string("fixtures/po-oa-management-attachments.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let action_list = get_complex_type(&schema, "IncrementalActionList");

    let expected_elements = get_expected_elements();
    let expected_choices = get_expected_choice();

    assert_eq!(
      action_list.sequence,
      Some(Sequence {
        elements: expected_elements,
        choices: expected_choices,
      })
    );
  }

  fn get_expected_elements() -> Vec<Element> {
    vec![Element {
      name: Some(String::from("incrementalListCycleInformation")),
      r#type: Some(String::from("co:IncrementalListCycleInformation")),
      annotation: Some(Annotation {
        id: None,
        attributes: vec![],
        documentation: None,
        app_info: Some(AppInfo {
          meta_info: Some(MetaInfo {
            xml: Some(Xml {
              old: vec![
                Old {
                  name: String::from("amListeNummer"),
                },
                Old {
                  name: String::from("amListeZeitstempel"),
                },
                Old {
                  name: String::from("amListePruefsumme"),
                },
              ],
            }),
            binary: None,
          }),
        }),
      }),
      ..Default::default()
    }]
  }

  fn get_expected_choice() -> Vec<Choice> {
    vec![Choice {
            elements: vec![
                Element {
                    name: Some(String::from("entitlementIssuanceActionListEntry")),
                    r#type: Some(String::from(
                        "po-oa-management:EntitlementIssuanceActionListEntry",
                    )),
                    annotation: Some(
                        Annotation {
                            id: None,
                            attributes: vec![],
                            documentation: Some(
                                String::from( "Action list entry for an entitlement issuance."),
                            ),
                            app_info: Some(
                                AppInfo {
                                    meta_info: Some(
                                        MetaInfo {
                                            xml: Some(
                                                Xml {
                                                    old: vec![
                                                        Old {
                                                            name: String::from("txalisber"),
                                                        },
                                                    ],
                                                },
                                            ),
                                            binary: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    ..Default::default()
                },
                Element {
                    name: Some(String::from("entitlementTerminationActionListEntry")),
                    r#type: Some(String::from(
                        "po-oa-management:EntitlementTerminationActionListEntry",
                    )),
                    annotation: Some(Annotation {
                        id: None,
                        attributes: vec![],
                        documentation: Some(String::from(
                            "Action list entry for an entitlement termination.",
                        )),
                        app_info: Some(AppInfo {
                            meta_info: Some(MetaInfo {
                                xml: Some(Xml {
                                    old: vec![Old {
                                        name: String::from("txrlisber"),
                                    }],
                                }),
                                binary: None,
                            }),
                        }),
                    }),
                    ..Default::default()
                },
                Element {
                    name: Some(String::from("entitlementUnblockingActionListEntry")),
                    r#type: Some(String::from(
                        "po-oa-management:EntitlementUnblockingActionListEntry",
                    )),
                    annotation: Some(
                        Annotation {
                            id: None,
                            attributes: vec![],
                            documentation: Some(
                                String::from("Action list entry for an entitlement unblocking."),
                            ),
                            app_info: Some(
                                AppInfo {
                                    meta_info: Some(
                                        MetaInfo {
                                            xml: Some(
                                                Xml {
                                                    old: vec![
                                                        Old {
                                                            name: String::from("txelisber"),
                                                        },
                                                    ],
                                                },
                                            ),
                                            binary: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),

                    ..Default::default()
                },
                Element {
                    name: Some(String::from("entitlementBlockingActionListEntry")),
                    r#type: Some(String::from(
                        "po-oa-management:EntitlementBlockingActionListEntry",
                    )),
                    annotation: Some(
                        Annotation {
                            id: None,
                            attributes: vec![],
                            documentation: Some(
                                String::from("Action list entry for an entitlement blocking."),
                            ),
                            app_info: Some(
                                AppInfo {
                                    meta_info: Some(
                                        MetaInfo {
                                            xml: Some(
                                                Xml {
                                                    old: vec![],
                                                },
                                            ),
                                            binary: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),

                    ..Default::default()
                },
                Element {
                    name: Some(String::from("removeOrder")),
                    r#type: Some(String::from("po-oa-management:OrderId")),
                    annotation: Some(
                        Annotation {
                            id: None,
                            attributes: vec![],
                            documentation: Some(
                                String::from("Indicates that the corresponding action list entry has been removed from the action list."),
                            ),
                            app_info: Some(
                                AppInfo {
                                    meta_info: Some(
                                        MetaInfo {
                                            xml: Some(
                                                Xml {
                                                    old: vec![
                                                        Old {
                                                            name: String::from("txllisber"),
                                                        },
                                                    ],
                                                },
                                            ),
                                            binary: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),

                    ..Default::default()
                },
            ],
            annotation: None,
            min_occurences: Some(0),
            max_occurences: Some(MaxOccurences::Unbounded),
        }]
  }
}
