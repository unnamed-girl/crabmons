use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Learnset {
    pub learnset: Option<HashMap<String, Vec<String>>>,
    // event_data: Option<>
    #[serde(default)]
    pub encounters: Vec<Encounter>,
    #[serde(default)]
    pub event_only: bool,
}
impl Learnset {
    pub fn all_moves(&self) -> Vec<&String> {
        self.learnset
            .as_ref()
            .map_or_else(Vec::new, |learnset| learnset.keys().collect())
    }
}

#[derive(Deserialize)]
pub struct Encounter {
    pub generation: u8,
    pub level: u8
}