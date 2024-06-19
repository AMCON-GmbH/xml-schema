#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct EnumValue {
    #[yaserde(rename = "base", attribute)]
    pub base: Option<String>,
    // TODO
}