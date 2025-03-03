use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{items::Priorities, species::Stat, types::Type};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Move {
    pub num: i32,
    pub accuracy: Accuracy,
    pub base_power: u8,
    pub category: Category,
    pub is_nonstandard: Option<NonStandardReason>,
    pub name: String,
    pub pp: u8,
    pub priority: i8,
    pub flags: FlagList,
    pub is_z: Option<String>,
    pub crit_ratio: Option<u8>,
    pub secondary: Option<Secondary>,
    pub target: Target,
    pub type_: Type,
    pub contest_type: Option<ContestType>,
    #[serde(default)]
    #[serde(rename = "desc")]
    pub description: String,
    #[serde(default)]
    #[serde(rename = "shortDesc")]
    pub short_description: String,
    pub drain: Option<(u8, u8)>,
    pub z_move: Option<ZMoveData>, // todo
    #[serde(default)] 
    pub base_power_callback: bool,
    pub condition: Option<Condition>,
    #[serde(default)]
    #[serde(rename = "multihit")]
    pub number_of_hits: NumberOfHits,
    #[serde(default)] 
    pub calls_move: bool,
    #[serde(default)] 
    pub has_crash_damage: bool,
    #[serde(default)] 
    pub stalling_move: bool,
    #[serde(default)] 
    pub self_switch: SelfSwitch,
    #[serde(default)]
    pub ignore_immunity: IgnoreImmunity,
    pub override_offensive_stat: Option<Stat>,
    pub max_move: Option<MaxMoveData>,
    pub recoil: Option<(u8, u8)>,
    #[serde(default)] 
    pub ignore_defensive: bool,
    #[serde(default)] 
    pub ignore_evasion: bool,
    #[serde(default)] 
    pub force_switch: bool,
    pub non_ghost_target: Option<NonGhostTarget>, // Curse
    #[serde(default)] 
    pub smart_target: bool,
    pub damage: Option<AlternativeDamage>,
    pub terrain: Option<Terrain>,
    #[serde(default)] 
    pub has_sheer_force: bool, // Applies sheer force but secondary effects still happen.
    pub selfdestruct: Option<SelfDestruct>,
    #[serde(default)] 
    pub breaks_protect: bool,
    pub secondaries: Option<Vec<Secondary>>,
    pub ohko: Option<OHKO>,
    #[serde(default)] 
    pub will_crit: bool,
    pub override_offensive_pokemon: Option<OverrideOffensivePokemon>,
    pub is_max: Option<MaxMove>, 
    #[serde(default)] 
    pub ignore_ability: bool,
    pub slot_condition: Option<SlotCondition>,
    pub heal: Option<(u8, u8)>,
    pub real_move: Option<String>,
    #[serde(default)] 
    pub thaws_target: bool,
    #[serde(default)] 
    pub mind_blown_recoil: bool,
    #[serde(default)] 
    pub multi_accuracy: bool,
    pub override_defensive_stat: Option<Stat>,
    #[serde(default)]
    #[serde(rename = "noPPBoosts")] 
    pub no_pp_boosts: bool,
    #[serde(default)] 
    pub sleep_usable: bool,
    #[serde(default)] 
    pub tracks_target: bool,
    #[serde(default)] 
    pub steals_boosts: bool,
    #[serde(default)] 
    pub struggle_recoil: bool,
    #[serde(default)]
    pub multiaccuracy: bool,

    #[serde(deserialize_with = "deserialize_self_boosts")]
    #[serde(default)]
    pub self_boost: Option<BoostsList>,
    
    #[serde(flatten)]
    pub priorities: Option<Priorities>,
    
    #[serde(flatten)]
    pub target_effects: Option<MoveEffects>,
    #[serde(rename="self")]
    pub self_effects: Option<MoveEffects>,
}
impl Move {
    pub fn has_flag(&self, flag: Flag) -> bool {
        self.flags.has_flag(flag)
    }
}

fn deserialize_self_boosts<'de, D>(deserializer: D) -> Result<Option<BoostsList>, D::Error> where D: Deserializer<'de> {
    let value = serde_json::Value::deserialize(deserializer)?;
    let temp: Option<BoostsList> = match &value {
        Value::Object(inner) => match inner.get("boosts") {
            Some(value) => serde_json::from_value(value.clone()).ok(),
            None => None
        }
        _ => None
    };
    temp.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for self boosts")))
        .map(Option::Some)
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Secondary {
    #[serde(default)]
    pub dustproof: bool, // Whether this gets stopped by shield dust

    #[serde(flatten)]
    pub target_effects: MoveEffects,
    #[serde(rename = "self")]
    pub self_effects: Option<MoveEffects>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MoveEffects {
    pub chance: Option<u8>,
    pub boosts: Option<BoostsList>,
    pub volatile_status: Option<VolatileStatus>,
    pub side_condition: Option<SideCondition>,
    pub pseudo_weather: Option<PseudoWeather>,
    pub weather: Option<Weather>,
    pub status: Option<Status>,
}

#[derive(Clone)]
pub enum MaxMove {
    TypeBased,
    ForPokemon(String)
}
impl<'de> Deserialize<'de> for MaxMove {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Bool(true) => Some(MaxMove::TypeBased),
            serde_json::Value::String(s) => Some(MaxMove::ForPokemon(s.clone())),
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for ignore immunity")
    ))
    }
}

#[derive(Clone)]
pub enum IgnoreImmunity {
    True,
    False,
    Types(Vec<Type>)
}
impl Default for IgnoreImmunity {
    fn default() -> Self {
        Self::False
    }
}
impl<'de> Deserialize<'de> for IgnoreImmunity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Bool(true) => Some(IgnoreImmunity::True),
            serde_json::Value::Bool(false) => Some(IgnoreImmunity::False),
            serde_json::Value::Object(_) => {
                    let types: Option<HashMap<Type, bool>> = serde_json::from_value(value.clone()).ok();
                    types.map(|m| IgnoreImmunity::Types(m.into_keys().collect()))
            },
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for ignore immunity")
    ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Accuracy {
    AlwaysHits,
    Percent(u8)
}
impl<'de> Deserialize<'de> for Accuracy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Number(n) => n.as_u64().and_then(|n| n.try_into().ok()).map(Accuracy::Percent),
            serde_json::Value::Bool(true) => Some(Accuracy::AlwaysHits),
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for accuracy")
    ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlternativeDamage {
    Flat(u8),
    Unique(UniqueDamage)
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum UniqueDamage {
    #[serde(rename = "level")]
    Level
}
impl<'de> Deserialize<'de> for AlternativeDamage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Number(n) => n.as_u64().and_then(|n| n.try_into().ok()).map(AlternativeDamage::Flat),
            serde_json::Value::String(_) => serde_json::from_value(value.clone()).map(AlternativeDamage::Unique).ok(),
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for alternative damage")
    ))
    }
}
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum NumberOfHits {
    #[default]
    Normal,
    Constant(u8),
    Range(u8, u8)
}
impl NumberOfHits {
    pub fn max(self) -> u8 {
        match self {
            Self::Normal => 1,
            Self::Constant(n) => n,
            Self::Range(_, max) => max,
        }
    }
    pub fn min(self) -> u8 {
        match self {
            Self::Normal => 1,
            Self::Constant(n) => n,
            Self::Range(min, _) => min,
        }
    }
}
impl<'de> Deserialize<'de> for NumberOfHits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Number(n) => n.as_u64().and_then(|n| n.try_into().ok()).map(NumberOfHits::Constant),
            serde_json::Value::Array(a) => match &a[..] {
                [serde_json::Value::Number(a), serde_json::Value::Number(b)] =>
                    a.as_u64().and_then(|a| a.try_into().ok()).and_then(|a| 
                        b.as_u64().and_then(|b| b.try_into().ok()).map(|b| 
                            NumberOfHits::Range(a, b)
                        )
                    ),
                _ => None
            },
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for accuracy")
    ))
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Category {
    Physical,
    Special,
    Status,
}

#[derive(Hash, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Flag {
    AllyAnim,
    Bite,
    Bullet,
    BypassSub,
    CantUseTwice,
    Charge,
    Contact,
    Dance,
    Defrost,
    Distance,
    FailCopycat,
    Failencore,
    FailInstruct,
    FailMeFirst,
    FailMimic,
    FutureMove,
    Gravity,
    Heal,
    Metronome,
    Mirror,
    MustPressure,
    NoAssist,
    NonSky,
    NoParentalBond,
    NoSketch,
    NoSleepTalk,
    PledgeCombo,
    Powder,
    Protect,
    Pulse,
    Punch,
    Recharge,
    Reflectable,
    Slicing,
    Snatch,
    Sound,
    Wind,
}

#[derive(Clone)]
pub struct FlagList(pub Vec<Flag>);
impl FlagList {
    pub fn has_flag(&self, flag: Flag) -> bool {
        self.0.contains(&flag)
    }
}
impl<'de> Deserialize<'de> for FlagList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Object(_) => {
                let flags: Option<HashMap<Flag, Value>> = serde_json::from_value(value.clone()).ok();
                flags.map(|m| FlagList(m.into_keys().collect()))
            },
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not a valid list of flags")))
    }
}

#[derive(Clone, Debug)]
pub struct BoostsList(pub Vec<(Stat, i8)>);
impl<'de> Deserialize<'de> for BoostsList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Object(_) => {
                let flags: Option<HashMap<Stat, i8>> = serde_json::from_value(value.clone()).ok();
                flags.map(|m| BoostsList(m.into_iter().collect()))
            },
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not a valid list of boosts")))
    }
}


#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Target {
    AllySide,
    AdjacentAlly,
    FoeSide,
    AllAdjacent,
    Scripted,
    All,
    Normal,
    AllAdjacentFoes,
    AdjacentAllyOrSelf,
    AllyTeam,
    #[serde(rename = "self")]
    Self_,
    Any,
    Allies,
    AdjacentFoe,
    RandomNormal,
}
impl Target {
    pub fn is_multi_target(&self) -> bool {
        match self {
            Target::AllAdjacent | Target::AllAdjacentFoes | Target::All | Target::FoeSide | Target::AllySide | Target::Allies => true,
            Target::Self_ | Target::AdjacentAlly | Target::AdjacentAllyOrSelf | Target::AdjacentFoe | Target::Any | Target::Normal | Target::RandomNormal | Target::AllyTeam | Target::Scripted => false
        }
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum NonStandardReason {
    LGPE,
    Past,
    Future,
    CAP,
    Gigantamax,
    Unobtainable,
}
#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContestType {
    Clever,
    Cute,
    Tough,
    Beautiful,
    Cool,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VolatileStatus {
    Minimize,
    HealBlock,
    MiracleEye,
    Torment,
    BanefulBunker,
    LaserFocus,
    Grudge,
    Bide,
    KingsShield,
    Magnetrise,
    Charge,
    Protect,
    MaxGuard,
    BurningBulwark,
    Curse,
    PartiallyTrapped,
    Spotlight,
    Stockpile,
    Electrify,
    Tarshot,
    Obstruct,
    Smackdown,
    FocusEnergy,
    Foresight,
    Disable,
    Confusion,
    Snatch,
    NoRetreat,
    Octolock,
    DefenseCurl,
    LeechSeed,
    MagicCoat,
    Ingrain,
    DragonCheer,
    FollowMe,
    Nightmare,
    Taunt,
    Telekinesis,
    Endure,
    Attract,
    DestinyBond,
    Powder,
    Substitute,
    AquaRing,
    SilkTrap,
    Imprison,
    PowerShift,
    PowerTrick,
    SpikyShield,
    HelpingHand,
    GastroAcid,
    Embargo,
    Encore,
    RagePowder,
    Yawn,
    Flinch,
    MustRecharge,
    Roost,
    LockedMove,
    Rage,
    GlaiveRush,
    UpRoar,
    SaltCure,
    SparklingAria,
    SyrupBomb,

    // Past Volatile Statuses
    MudSport,
    Autotomize,
    WaterSport,
    Mist,
    LightScreen,
    Reflect,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SideCondition {
    Reflect,
    Mist,
    ToxicSpikes,
    CraftyShield,
    AuroraVeil,
    MatBlock,
    LuckyChant,
    SafeGuard,
    StickyWeb,
    Spikes,
    Tailwind,
    WideGuard,
    QuickGuard,
    StealthRock,
    LightScreen,
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelfSwitch {
    True,
    #[default]
    False,
    Unique(UniqueSelfSwitch)
}
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UniqueSelfSwitch {
    CopyVolatile,
    ShedTail,
}
impl<'de> Deserialize<'de> for SelfSwitch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Bool(true) => Some(SelfSwitch::True),
            serde_json::Value::Bool(false) => Some(SelfSwitch::False),
            serde_json::Value::String(_) => serde_json::from_value(value.clone()).map(SelfSwitch::Unique).ok(),
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for a self switch")))
    }
}


#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Weather {
    Sandstorm,
    #[serde(rename = "sunnyday")]
    SunnyDay,
    RainDance,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "hail")]
    Hail,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum NonGhostTarget {
    #[serde(rename = "self")]
    Self_
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    #[serde(rename = "tox")]
    Toxic,
    #[serde(rename = "brn")]
    Burn,
    #[serde(rename = "par")]
    Paralysis,
    #[serde(rename = "slp")]
    Sleep,
    #[serde(rename = "psn")]
    Poison,
    #[serde(rename = "frz")]
    Frozen,
}
#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Terrain {
    #[serde(rename = "mistyterrain")]
    Misty,
    #[serde(rename = "psychicterrain")]
    Psychic,
    #[serde(rename = "grassyterrain")]
    Grassy,
    #[serde(rename = "electricterrain")]
    Electric,
}


#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelfDestruct {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "ifHit")]
    IfHit
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PseudoWeather {
    IonDeluge,
    MagicRoom,
    WaterSport,
    FairyLock,
    WonderRoom,
    TrickRoom,
    Gravity,
    MudSport,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OHKO {
    Regular,
    Unique(UniqueOHKO)
}
impl<'de> Deserialize<'de> for OHKO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Bool(true) => Some(OHKO::Regular),
            serde_json::Value::String(_) => serde_json::from_value(value.clone()).map(OHKO::Unique).ok(),
            _ => None
        };
        result.ok_or_else(|| serde::de::Error::custom(format!("{value} is not valid for OHKO")))
    }
}
#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum UniqueOHKO {
    Ice
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum OverrideOffensivePokemon {
    #[serde(rename = "target")]
    Target
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SlotCondition {
    HealingWish,
    FollowMe, // Past
    LunarDance,
    RevivalBlessing,
    #[serde(rename = "Wish")]
    Wish,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ZMoveData {
    pub base_power: Option<u8>,
    pub boost: Option<BoostsList>,
    pub effect: Option<String> //TODO
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MaxMoveData {
    pub base_power: Option<u8>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    #[serde(default)]
    pub no_copy: bool,
    pub duration: Option<u8>,
    pub counter_max: Option<u16>,

    pub on_invulnerability: Option<bool>,
    pub on_critical_hit: Option<bool>,
    pub on_lock_move: Option<String>,

    #[serde(flatten)]
    pub priorities: Priorities,
}