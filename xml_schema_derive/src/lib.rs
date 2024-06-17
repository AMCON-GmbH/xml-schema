extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate yaserde_derive;

use crate::attribute::XmlSchemaAttributes;
use darling::FromDeriveInput;
use syn::DeriveInput;

mod attribute;
mod expander;
pub mod xsd;

pub fn xml_schema_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: DeriveInput = syn::parse2(proc_macro2::TokenStream::from(input)).unwrap();

  let attributes = XmlSchemaAttributes::from_derive_input(&input).unwrap();

  match expander::expand_derive(&attributes) {
    Ok(expanded) => expanded.into(),
    Err(msg) => panic!("{}", msg),
  }
}
