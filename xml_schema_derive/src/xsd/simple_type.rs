use crate::xsd::annotation::Annotation;
use crate::xsd::{list::List, restriction::Restriction, union::Union};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleType {
  #[yaserde(attribute)]
  pub name: String,
  #[yaserde(attribute)]
  pub id: String,
  pub annotation: Option<Annotation>,
  pub restriction: Option<Restriction>,
  pub list: Option<List>,
  pub union: Option<Union>,
}

/// Tests if simple types are deserialized correctly from xsd files.
/// Each test deserializes the xsd to Schema and then finds the simple type to test, in contrast to directly deserializing SimpleType.
/// That is because isolated SimpleTypes do not exist and namespaces can only be checked in the context of a Schema.
#[cfg(test)]
pub(crate) mod tests {
  use std::default::Default;
  use std::fmt::Debug;
  use std::fs;

  use assert_eq;

  use crate::xsd::annotation::{Annotation, AppInfo, MetaInfo, Xml};
  use crate::xsd::restriction::{
    EnumRestrictionValue, NumericRestrictionValue, Restriction, StringRestrictionValue,
  };
  use crate::xsd::schema::Schema;
  use crate::xsd::simple_type::SimpleType;
  pub use crate::xsd::tests::reduce_whitespace;
  use crate::xsd::union::Union;

  //region helpers
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

    fn assert_annotation_old_name(&self, documentation: &str, old_names: Vec<&str>) {
      Annotation::assert_old_name(self.annotation.as_ref().unwrap(), documentation, old_names)
    }
  }

  fn get_simple_type<'a>(schema: &'a Schema, type_name: &str) -> &'a SimpleType {
    schema
      .simple_type
      .iter()
      .find(|x| x.name == type_name)
      .unwrap()
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
            binary: None,
          }),
        }),
      }),
    }
  }
  //endregion

  //region tests
  #[test]
  fn de_type_with_max_length_restriction_should_be_parsed_correctly() {
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

    Annotation::check(
      blocking_reason_text.annotation.as_ref().unwrap(),
      "Defines a free text for blocking reason description with a maximum length of 1024 bytes.",
      None,
    );
  }

  #[test]
  fn de_type_with_multiline_documentation_should_be_parsed_correctly() {
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
  fn de_type_with_pattern_restriction_should_be_parsed_correctly() {
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
      vec!["DateTime"],
    );
  }

  #[test]
  fn de_int_enum_should_be_parsed_correctly() {
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
      vec!["Mitteilung_CODE_Type"]
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

  #[test]
  fn de_string_enum_should_be_parsed_correctly() {
    // given
    let xsd = fs::read_to_string("fixtures/hotlist-enums.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let unclaimed_list_type_enum: &SimpleType = get_simple_type(&schema, "UnclaimedListTypeEnum");

    assert_eq!(
      unclaimed_list_type_enum.id,
      "_ed07d0a9-ca8c-45cc-86a8-9fb3a78fd58a"
    );

    assert_eq!(
      unclaimed_list_type_enum.name.as_str(),
      "UnclaimedListTypeEnum"
    );

    unclaimed_list_type_enum.assert_annotation_old_name(
      "Types of lists that might be unclaimed from the hotlist service. \
      Does not distinguish, e.g., between full or incremental hotlists variants.",
      vec![
        "ListenTypDNMEnumType",
        "ListenTypEnumType",
        "ListenGrundTypEnumType",
      ],
    );

    unclaimed_list_type_enum.assert_enum_restriction(
      "co:StringEnumMarker",
      vec![
        get_enum_value("APP", "Hotlist for applications."),
        get_enum_value("ENT", "Hotlist for entitlements."),
        get_enum_value("SAM", "Hotlist for SAMs."),
        get_enum_value("ORG", "Hotlist for organisations."),
        get_enum_value("SYMKEY", "Hotlist for symmetric authentication keys."),
      ],
    );
  }

  #[test]
  fn de_union_should_contain_correct_member_types() {
    // given
    let xsd = fs::read_to_string("fixtures/individualisation-common-enums.xsd").unwrap();

    // when
    let schema: Schema = yaserde::de::from_str(xsd.as_str()).unwrap();

    // then
    let media_enum: &SimpleType = get_simple_type(&schema, "MediaEnvironmentEnum");
    assert_eq!(
      media_enum.union,
      Some(
        Union {
          member_types: String::from("tns:SamFullEnvironmentEnum tns:SamLimitedEnvironmentEnum tns:UmFullEnvironmentEnum tns:UmLimitedEnvironmentEnum")
        }
      )
    );
  }
}
