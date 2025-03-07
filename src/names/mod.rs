use serde::Deserialize;

pub mod moves;
pub mod abilities;

#[derive(Deserialize)]
#[serde(from = "String")]
pub struct IdentifierName(String);
impl IdentifierName {
    pub fn new(value: String) -> Self {
        Self(value.chars()
            .filter(|c| !['\'', ' ', '-', '(', ')'].contains(c))
            .map(|ch| ch.to_ascii_lowercase())
            .collect()
        )
    }
    pub fn inner(&self) -> &str {
        &self.0
    }
}
impl From<String> for IdentifierName {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}