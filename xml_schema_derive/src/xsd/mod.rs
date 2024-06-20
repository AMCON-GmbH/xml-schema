mod annotation;
mod attribute;
mod attribute_group;
mod complex_content;
mod complex_type;
mod element;
mod extension;
mod group;
mod import;
mod list;
mod max_occurences;
mod qualification;
mod restriction;
pub mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod union;
mod choice;

/// Common test utility functions.
#[cfg(test)]
pub(crate) mod tests {
  use crate::xsd::annotation::{Annotation, AppInfo, Binary, MetaInfo, Xml};
  use std::collections::HashSet;

  ///
  /// Trims down all kinds of whitespaces to single whitespaces.
  /// Used to compare multiline strings without accounting for differences in consecutive spaces and linebreaks.
  ///
  pub fn reduce_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
  }

  impl Annotation {
    pub(crate) fn reduce_whitespaces(&self) -> Annotation {
      let mut clone = self.clone();

      if let Some(d) = &self.documentation {
        clone.documentation = Some(reduce_whitespace(d.as_str()));
      }

      clone
    }

    pub fn check(annotation: &Annotation, expected_doc: &str, expected_asn1_tag: Option<&str>) {
      let actual = annotation.reduce_whitespaces();

      let expected = Annotation {
        id: None,
        attributes: vec![],
        documentation: Some(reduce_whitespace(expected_doc)),
        app_info: Some(AppInfo {
          meta_info: Some(MetaInfo {
            xml: Some(Xml { old: vec![] }),
            binary: expected_asn1_tag.map(|tag| Binary {
                asn1_tag: Some(String::from(tag)),
              }),
          }),
        }),
      };

      assert_eq!(actual, expected);
    }

    pub fn assert_old_name(annotation: &Annotation, documentation: &str, old_names: Vec<&str>) {
      let actual = annotation.reduce_whitespaces();
      let actual_xml = actual.app_info.unwrap().meta_info.unwrap().xml.unwrap();

      // HashSet because order does not matter
      let expected_old_names: HashSet<&str> = old_names.iter().copied().collect();

      assert_eq!(actual.id, None);
      assert_eq!(actual.attributes, vec![]);
      assert_eq!(actual.documentation, Some(reduce_whitespace(documentation)));
      assert!(actual_xml
        .old
        .iter()
        .all(|x| expected_old_names.contains(x.name.as_str())));
    }
  }
}
