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
mod schema_tests {
  use std::fs;

  use pretty_assertions::assert_eq;
  use std::default::Default;
  use std::fmt::Debug;

  use crate::xsd::annotation::{Annotation, AppInfo, MetaInfo, Old, Xml};
  use crate::xsd::restriction::{
    EnumRestrictionValue, NumericRestrictionValue, Restriction, StringRestrictionValue,
  };
  use crate::xsd::schema::Schema;
  use crate::xsd::simple_type::SimpleType;

  //region helper methods
  impl SimpleType {
    fn assert_restriction<T: Debug + PartialEq>(
      &self,
      base: &str,
      actual: fn(restriction: &Restriction) -> &T,
      expected: &T,
    ) {
      let restriction = self.restriction.as_ref().unwrap();

      assert_eq!(restriction.base, Some(String::from(base)));
      assert_eq!(actual(restriction), expected);
    }

    /// Separate function as enums contain possibly multiline documentation (handling of linebreaks, extra whitespace)
    fn assert_enum_restriction(&self, base: &str, expected: Vec<EnumRestrictionValue>) {
      // trim all whitespaces to single whitespaces
      let mut trimmed: Vec<EnumRestrictionValue> = vec![];
      for value in &self.restriction.as_ref().unwrap().enumerations {
        trimmed.push(value.reduce_whitespaces());
      }

      let actual = Some(Restriction {
        // use trimmed version of the actual enum values for comparison
        enumerations: trimmed,
        // everything else from self.restriction
        ..self.restriction.as_ref().unwrap().clone()
      });

      let expected = Some(Restriction {
        base: Some(String::from(base)),
        enumerations: expected,
        ..Default::default()
      });

      assert_eq!(actual, expected);
    }

    fn assert_annotation_old_name(&self, documentation: &str, old_name: &str) {
      assert_eq!(
        self.annotation.as_ref().unwrap().reduce_whitespaces(),
        Annotation {
          id: None,
          attributes: vec![],
          documentation: Some(reduce_whitespace(documentation)),
          app_info: Some(AppInfo {
            meta_info: Some(MetaInfo {
              xml: Some(Xml {
                since: None,
                old: Some(Old {
                  name: String::from(old_name)
                }),
              })
            })
          }),
        }
      )
    }

    fn assert_annotation_since(&self, documentation: &str, since: &str) {
      assert_eq!(
        self.annotation.as_ref().unwrap().reduce_whitespaces(),
        Annotation {
          id: None,
          attributes: vec![],
          documentation: Some(reduce_whitespace(documentation)),
          app_info: Some(AppInfo {
            meta_info: Some(MetaInfo {
              xml: Some(Xml {
                since: Some(String::from(since)),
                ..Default::default()
              })
            })
          }),
        }
      )
    }
  }

  fn get_simple_type<'a>(schema: &'a Schema, type_name: &str) -> &'a SimpleType {
    schema
      .simple_type
      .iter()
      .find(|x| x.name == type_name)
      .unwrap()
  }

  ///
  /// Trims down all kinds of whitespaces to single whitespaces.
  /// Used to compare multiline strings without accounting for differences in consecutive spaces and linebreaks.
  ///
  pub fn reduce_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
  }

  impl Annotation {
    fn reduce_whitespaces(&self) -> Annotation {
      let mut clone = self.clone();

      if let Some(d) = &self.documentation {
        clone.documentation = Some(reduce_whitespace(d.as_str()));
      }

      clone
    }
  }

  impl EnumRestrictionValue {
    fn reduce_whitespaces(&self) -> EnumRestrictionValue {
      let mut clone = self.clone();

      if let Some(a) = &self.annotation {
        clone.annotation = Some(a.reduce_whitespaces())
      }

      clone
    }
  }

  fn get_enum_value(value: &str, doc: &str) -> EnumRestrictionValue {
    EnumRestrictionValue {
      value: String::from(value),
      annotation: Some(Annotation {
        id: None,
        attributes: vec![],
        documentation: Some(String::from(doc)),
        app_info: Some(AppInfo {
          meta_info: Some(MetaInfo {
            xml: Some(Xml {
              ..Default::default()
            }),
          }),
        }),
      }),
    }
  }
  //endregion

  #[test]
  fn de_simple_type_blocking_reason_text() {
    // given
    let xsd = fs::read_to_string("fixtures/common.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let blocking_reason_text: &SimpleType = get_simple_type(&schema, "BlockingReasonText");

    assert_eq!(
      blocking_reason_text.id,
      "_8e4d8bc8-c1fa-4a73-b932-55931892ffda"
    );
    assert_eq!(blocking_reason_text.name.as_str(), "BlockingReasonText");

    blocking_reason_text.assert_restriction(
      "xs:string",
      |x| &x.max_length,
      &Some(NumericRestrictionValue { value: 1024 }),
    );

    blocking_reason_text.assert_annotation_since(
      "Defines a free text for blocking reason description with a maximum length of 1024 bytes.",
      "3.0.0",
    );
  }

  #[test]
  fn de_simple_type_list_cycle_number_with_multiline_documentation() {
    // given
    let xsd = fs::read_to_string("fixtures/common.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let zoned_date_time: &SimpleType = get_simple_type(&schema, "ListCycleNumber");

    let actual = zoned_date_time
      .annotation
      .as_ref()
      .unwrap()
      .documentation
      .as_ref()
      .unwrap();

    let expected = String::from("Strictly monotonically ascending sequence number which identifies the list cycle.
                The sequence number is increased at a certain time and remains active for a certain timespan
                (e.g. once a day at 01:00 AM).
                This system wide cycle number does not depend on organisations or roles.
                The cycle number references all list objects, configuration objects,
                etc. with their states for the active phase of the cycle.
                In case of a request for an incremental list, the last used cycle number has to be passed
                to the list service. Then all list content changes between this last cycle number and the current
                active cycle will be returned to the caller.");

    assert_eq!(
      reduce_whitespace(actual.as_str()),
      reduce_whitespace(expected.as_str())
    );
  }

  #[test]
  fn de_simple_type_zoned_datetime() {
    // given
    let xsd = fs::read_to_string("fixtures/common.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let zoned_date_time: &SimpleType = get_simple_type(&schema, "ZonedDateTime");

    assert_eq!(zoned_date_time.id, "_e382223e-33b9-41e8-ae79-efe5dd9cb28c");
    assert_eq!(zoned_date_time.name.as_str(), "ZonedDateTime");

    zoned_date_time.assert_restriction(
      "xs:dateTime",
      |x| &x.patterns,
      &vec![
        StringRestrictionValue {
          value: String::from(".*Z"),
        },
        StringRestrictionValue {
          value: String::from(r#".*[+\-]\d\d:\d\d"#),
        },
      ],
    );

    zoned_date_time.assert_annotation_old_name(
      "Derivative of xs:dateTime with required timezone component.",
      "DateTime",
    );
  }

  #[test]
  fn de_simple_type_enum() {
    // given
    let xsd = fs::read_to_string("fixtures/common-enums.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let hotlisting_demand_result: &SimpleType =
      get_simple_type(&schema, "HotlistingDemandResultEnum");

    assert_eq!(
      hotlisting_demand_result.id,
      "_c3157b6a-3e65-442e-af65-d9b72ecd7851"
    );
    assert_eq!(
      hotlisting_demand_result.name.as_str(),
      "HotlistingDemandResultEnum"
    );

    hotlisting_demand_result.assert_annotation_old_name(
            "Code enumeration of possible responses for a previous hotlisting demand. See also HotlistingDemandResultCode.",
            "Mitteilung_CODE_Type"
        );

    hotlisting_demand_result.assert_enum_restriction(
      "tns:HotlistingDemandResultCode",
      vec![get_enum_value(
      "1",
      "Accepted (if block reason can be dropped on requestor's side, the entity owner expects an hotlisting demand revocation with the same reason code)."
    ), get_enum_value(
        "3",
        "Not accepted (no further action from the requesting system expected)."
      )],
    );
  }
}
