use std::{collections::HashMap, fmt::{Debug, Display}};

use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;

use crate::{dex::Identifier, moves::NonStandardReason, species::Stat};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Type {    
    Normal,
    Fire,
    Water,
    Grass,
    Flying,
    Fighting,
    Poison,
    Electric,
    Ground,
    Rock,
    Psychic,
    Ice,
    Bug,
    Ghost,
    Steel,
    Dragon,
    Dark,
    Fairy,
    Stellar,

    #[serde(rename = "???", other)] // default if type not recognised)
    Unknown,
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&to_variant_name(self).unwrap().to_lowercase(), f)
    }
}
impl Identifier for Type {
    fn as_identifier(&self) -> String {
        self.to_string()
    }
}
impl Identifier for &Type {
    fn as_identifier(&self) -> String {
        self.to_string()
    }
}

#[derive(Deserialize)]
pub struct TypeData {
    #[serde(rename = "damageTaken")]
    damage_taken: HashMap<Type, DamageRelation>,
    #[serde(rename = "HPivs")]
    pub hpivs: Option<HashMap<Stat, u8>>,
    #[serde(rename = "HPdvs")]
    pub hpdvs: Option<HashMap<Stat, u8>>,
    #[serde(rename = "isNonstandard")]
    pub is_non_standard: Option<NonStandardReason>,
}
impl TypeData {
    pub fn damage_taken(&self, type_: Type) -> DamageRelation {
        self.damage_taken.get(&type_)
            .copied()
            .unwrap_or(DamageRelation::Neutral)
    }
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(try_from = "u8")]
pub enum DamageRelation {
    Neutral,
    SuperEffective,
    NotVeryEffective,
    Immune
}

pub struct TryIntoDamageRelationError;
impl Display for TryIntoDamageRelationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&"Given value was not between 0 and 3 inclusive", f)
    }
}
impl TryFrom<u8> for DamageRelation {
    type Error = TryIntoDamageRelationError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Neutral),
            1 => Ok(Self::SuperEffective),
            2 => Ok(Self::NotVeryEffective),
            3 => Ok(Self::Immune),
            _ => Err(TryIntoDamageRelationError)
        }
    }
}