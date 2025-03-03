use std::collections::HashMap;

use serde::Deserialize;

use crate::{generation::Generation, natures::Nature, species::StatDistribution};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Learnset {
    pub learnset: Option<HashMap<String, Vec<String>>>,
    pub event_data: Option<Vec<EncounterData>>,
    #[serde(default)]
    pub encounters: Vec<EncounterData>,
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

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct EncounterData {
    pub generation: Generation,
    pub level: Option<u8>,
    pub moves: Option<Vec<String>>,
    pub pokeball: Option<String>,
    pub max_egg_moves: Option<u8>,
    pub abilities: Option<Vec<String>>,
    pub gender: Option<Gender>,
    #[serde(default)]
    pub shiny: EventShiny,
    pub nature: Option<Nature>,
    #[serde(default)]
    pub emerald_event_egg: bool,
    #[serde(default)]
    pub is_hidden:bool,
    pub ivs: Option<StatDistribution>,
    #[serde(rename = "perfectIVs")]
    pub perfect_ivs: Option<u8>
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    #[serde(rename = "F")]
    Female,
    #[serde(rename = "M")]
    Male,
    #[serde(rename = "N")]
    Unknown
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventShiny {
    Shiny,
    CanBeEither,
    #[default]
    NotShiny
}
impl<'de> Deserialize<'de> for EventShiny {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Bool(true) => Some(EventShiny::Shiny),
            serde_json::Value::Number(n) => if n.as_u64().is_some_and(|n| n == 1) {Some(EventShiny::CanBeEither)} else {None},
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for an event shiny")
    ))
    }
}