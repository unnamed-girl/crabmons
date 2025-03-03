use std::collections::HashMap;

use serde::Deserialize;

use crate::{parsing_utils::{Either, NotImplemented}, generation::Generation, natures::Nature, species::StatDistribution};

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

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(try_from = "Option<Either<bool, u8>>")]
pub enum EventShiny {
    Shiny,
    CanBeEither,
    NotShiny
}
impl TryFrom<Option<Either<bool, u8>>> for EventShiny {
    type Error = NotImplemented;
    fn try_from(value: Option<Either<bool, u8>>) -> Result<Self, Self::Error> {
        match value {
            Some(Either::A(true)) => Ok(Self::Shiny),
            None | Some(Either::A(false)) => Ok(Self::NotShiny),
            Some(Either::B(1)) => Ok(Self::CanBeEither),
            Some(Either::B(_)) => Err(NotImplemented("Only 'true', 'false' or '1' are valid for shiny"))
        }
    }
}