use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::{generation::Generation, learnsets::Gender, moves::NonStandardReason, names::abilities::Ability, types::Type};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Species {
    pub num: i32,
    pub name: String,
    pub types: Vec<Type>,
    /// Generation of a custom pokemon
    pub gen: Option<Generation>,
    pub gender_ratio: Option<GenderRatio>,
    pub gender: Option<Gender>,
    pub base_stats: StatDistribution,
    pub abilities: Abilities,
    pub weightkg: f32,
    #[serde(default)]
    pub evos: Vec<String>,
    pub egg_groups: Vec<String>, // TODO enum,
    pub prevo: Option<String>,
    pub evo_level: Option<u8>,
    pub evo_item: Option<String>,
    #[serde(default)]
    pub other_formes: Vec<String>,
    #[serde(default)]
    pub forme_order: Vec<String>,
    pub can_gigantamax : Option<String>,
    pub base_species: Option<String>,
    pub forme: Option<String>, // TODO enum
    pub required_item: Option<String>,
    pub is_nonstandard: Option<NonStandardReason>,
    pub changes_from: Option<String>,
    pub evo_condition: Option<String>,
    pub evo_type: Option<String>, // TODO enum,
    pub evo_region: Option<String>, // TODO enum
    pub mother: Option<String>,
    #[serde(default)]
    pub can_hatch: bool,
    pub evo_move: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>, // TODO Enum,
    pub base_forme: Option<String>,
    #[serde(default)]
    pub cosmetic_formes: Vec<String>,
    #[serde(rename = "maxHP")]
    pub max_hp: Option<u8>,
    pub required_ability: Option<String>,
    #[serde(deserialize_with = "deserialize_battle_only")]
    #[serde(default)]
    pub battle_only: Option<Vec<String>>,
    pub required_move: Option<String>,
    #[serde(default)]
    pub required_items: Vec<String>,
    #[serde(default)]
    pub cannot_dynamax: bool,
    pub force_tera_type: Option<Type>,
    #[serde(default)]
    pub unrelease_hidden: bool,
    #[serde(default)]
    pub male_only_hidden: bool,
}

fn deserialize_battle_only<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error> where D: Deserializer<'de> {
    let value = serde_json::Value::deserialize(deserializer)?;
    let temp: Option<Vec<String>> = match &value {
        Value::Array(n) => n.iter().cloned().map(serde_json::from_value::<String>).collect::<Result<_, _>>().ok(),
        Value::String(s) => Some(vec![s.clone()]),
        _ => None
    };
    temp.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for battle only")))
        .map(Option::Some)
}

#[derive(Clone, Deserialize)]
#[serde(from = "HashMap<char, Ability>")]
pub struct Abilities {
    pub normal: Vec<Ability>,
    pub hidden: Option<Ability>
}
impl From<HashMap<char, Ability>> for Abilities {
    fn from(mut value: HashMap<char, Ability>) -> Self {
        let hidden= value.remove(&'h');
        let normal = value.into_iter().map(|(_, ability)| ability).collect();
        Abilities { normal, hidden }
    }
}

#[derive(Deserialize, PartialEq, Clone, Copy, Debug)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct GenderRatio {
    pub f: f32,
    pub m: f32
}

#[derive(Deserialize, Default, PartialEq, Eq, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub struct StatDistribution {
    #[serde(default)]
    pub hp: u8,
    #[serde(default, rename = "atk")]
    pub attack: u8,
    #[serde(default, rename = "def")]
    pub defence: u8,
    #[serde(default, rename = "spa")]
    pub special_attack: u8,
    #[serde(default, rename = "spd")]
    pub special_defence: u8,
    #[serde(default, rename = "spe")]
    pub speed: u8,
}
impl StatDistribution {
    pub fn get(&self, stat: Stat) -> u8 {
        match stat {
            Stat::HP => self.hp,
            Stat::Attack => self.attack,
            Stat::Defence => self.defence,
            Stat::SpecialAttack => self.special_attack,
            Stat::SpecialDefence => self.special_defence,
            Stat::Speed => self.speed,
            _ => todo!()
        }
    }
    pub fn get_mut(&mut self, stat: Stat) -> &mut u8 {
        match stat {
            Stat::HP => &mut self.hp,
            Stat::Attack => &mut self.attack,
            Stat::Defence => &mut self.defence,
            Stat::SpecialAttack => &mut self.special_attack,
            Stat::SpecialDefence => &mut self.special_defence,
            Stat::Speed => &mut self.speed,
            _ => todo!()
        }
    }
}
impl From<[u8;6]> for StatDistribution {
    fn from(value: [u8;6]) -> Self {
        Self { hp: value[0], attack: value[1], defence: value[2], special_attack: value[3], special_defence: value[4], speed: value[5] }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Deserialize, Debug)]
pub enum Stat {
    #[serde(rename = "hp")]
    HP,
    #[serde(rename = "atk")]
    Attack,
    #[serde(rename = "def")]
    Defence,
    #[serde(rename = "spa")]
    SpecialAttack,
    #[serde(rename = "spd")]
    SpecialDefence,
    #[serde(rename = "spe")]
    Speed,
    #[serde(rename = "accuracy")]
    Accuracy,
    #[serde(rename = "evasion")]
    Evasion,
}