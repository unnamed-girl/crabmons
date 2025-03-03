use serde::Deserialize;

use crate::{generation::Generation, moves::{BoostsList, Condition, NonStandardReason, Status, VolatileStatus}, types::Type};

fn some_true() -> Option<bool> {
    Some(true)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ItemData {
    pub name: String,
    pub num: i16,
    pub is_nonstandard: Option<NonStandardReason>,
    #[serde(default)]
    #[serde(rename = "desc")]
    pub description: String,
    #[serde(default)]
    #[serde(rename = "shortDesc")]
    pub short_description: String,

    pub gen: Generation,
    pub fling: Option<FlingData>,
    pub natural_gift: Option<NaturalGiftData>,
    
    #[serde(default)]
    pub is_berry: bool,
    #[serde(default)]
    pub is_pokeball: bool,
    #[serde(default)]
    pub is_gem: bool,
    #[serde(default)]
    pub is_choice: bool,

    #[serde(rename = "itemUser")]
    pub item_users: Option<Vec<String>>,
    pub forced_forme: Option<String>,

    pub z_move: Option<ZCrystalData>,
    pub z_move_from: Option<String>,
    pub z_move_type: Option<Type>,

    pub mega_stone: Option<String>,
    pub mega_evolves: Option<String>,
    
    pub on_plate: Option<Type>,
    pub on_memory: Option<Type>,
    pub on_drive: Option<Type>,
    /// Some(true): can be stolen, even from or by one of their item_user
    /// (used by gen 4 arceus plates, where the steal prevention was part of the multitype ability).
    /// Other reasons for being unable to steal - like suction cups - still apply.
    /// 
    /// Some(false): can't be stolen no matter what.
    #[serde(default = "some_true")]
    #[serde(rename = "onTakeItem")]
    pub override_innate_stealable: Option<bool>,
    pub on_negate_immunity: Option<bool>, // todo rename and figure out default
    #[serde(default)]
    pub ignore_klutz: bool,
    pub on_eat:  Option<bool>, // todo rename and figure out default

    pub boosts: Option<BoostsList>,
    pub condition: Option<Condition>,

    #[serde(flatten)]
    pub priorities: Priorities,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FlingData {
    pub base_power: u8,
    pub volatile_status: Option<VolatileStatus>,
    pub status: Option<Status>
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NaturalGiftData {
    pub base_power: u8,
    pub type_: Type,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ZCrystalData {
    Generic,
    Unique(String)
}
impl<'de> Deserialize<'de> for ZCrystalData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Bool(true) => Some(ZCrystalData::Generic),
            serde_json::Value::String(base) => Some(ZCrystalData::Unique(base.clone())),
            _ => None
        };
        result.ok_or(serde::de::Error::custom(format!("{value} is not valid for a ZMove")))
    }
}


#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Priorities {
    // TODO handle priority with a type
    pub on_modify_atk_priority: Option<i8>,
    #[serde(rename = "onModifySpAPriority")]
    pub on_modify_spa_priority: Option<i8>,
    pub on_modify_def_priority: Option<i8>,
    #[serde(rename = "onModifySpDPriority")]
    pub on_modify_spd_priority: Option<i8>,
    
    pub on_foe_trap_pokemon_priority: Option<i8>,
    pub on_trap_pokemon_priority: Option<i8>,
    pub on_before_move_priority: Option<i8>,
    pub on_foe_before_move_priority: Option<i8>,
    pub on_modify_move_priority: Option<i8>,
    pub on_modify_accuracy_priority: Option<i8>,
    pub on_modify_type_priority: Option<i8>,
    pub on_attract_priority: Option<i8>,
    pub on_damage_priority: Option<i8>,
    pub on_accuracy_priority: Option<i8>,
    pub on_effectiveness_priority: Option<i8>,
    pub on_type_priority: Option<i8>,
    pub on_damaging_hit_order: Option<i8>,
    pub on_any_prepare_hit_priority: Option<i8>,
    pub on_try_primary_hit_priority: Option<i8>,
    pub on_try_hit_priority: Option<i8>,
    pub on_try_heal_priority: Option<i8>,
    pub on_try_boost_priority: Option<i8>,
    pub on_try_move_priority: Option<i8>,
    pub on_base_power_priority: Option<i8>,
    pub on_after_move_secondary_priority: Option<i8>,
    pub on_after_move_secondary_self_priority: Option<i8>,
    pub on_after_set_status_priority: Option<i8>,
    pub on_source_modify_accuracy_priority: Option<i8>,
    pub on_fractional_priority_priority: Option<i8>,
    pub on_fractional_priority: Option<f32>,
    pub on_field_residual_order: Option<i8>,
    pub on_field_residual_sub_order: Option<i8>,
    pub on_side_residual_order: Option<i8>,
    pub on_side_residual_sub_order: Option<i8>,
    pub on_residual_order: Option<i8>,
    pub on_residual_sub_order: Option<i8>,
    pub on_redirect_target_priority: Option<i8>,
    pub on_foe_redirect_target_priority: Option<i8>,
    pub on_source_invulnerability_priority: Option<i8>,
}