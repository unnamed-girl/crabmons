use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::{species::Stat, types::Type};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    pub num: i32,
    pub accuracy: Accuracy,
    pub base_power: u8,
    pub category: Category,
    pub is_non_standard: Option<NonStandardReason>,
    pub name: String,
    pub pp: u8,
    pub priority: i8,
    pub flags: FlagList,
    pub is_z: Option<String>,
    pub crit_ratio: Option<u8>,
    // secondary
    pub target: Target,
    pub type_: Type,
    pub contest_type: Option<ContestType>,
    #[serde(rename = "desc")]
    pub description: Option<String>,
    #[serde(rename = "shortDesc")]
    pub short_description: Option<String>,
    pub drain: Option<(u8, u8)>,
    pub boosts: Option<BoostsList>,
    // z_move:
    #[serde(default)] 
    pub base_power_callback: bool,
    // condition
    pub volatile_status: Option<VolatileStatus>,
    pub multihit: Option<MultiHit>,
    #[serde(default)] 
    pub calls_move: bool,
    pub side_condition: Option<SideCondition>,
    #[serde(default)] 
    pub has_crash_damage: bool,
    #[serde(default)] 
    pub stalling_move: bool,
    #[serde(default)] 
    pub self_switch: SelfSwitch,
    #[serde(default)]
    pub ignore_immunity: IgnoreImmunity,
    pub override_offensive_stat: Option<Stat>,
    // max_move
    pub recoil: Option<(u8, u8)>,
    pub weather: Option<Weather>,
    #[serde(default)] 
    pub ignore_defensive: bool,
    #[serde(default)] 
    pub ignore_evasion: bool,
    #[serde(default)] 
    pub force_switch: bool,
    // self_boost
    pub non_ghost_target: Option<NonGhostTarget>,
    pub status: Option<Status>,
    #[serde(default)] 
    pub smart_target: bool,
    pub damage: Option<AlternativeDamage>,
    pub terrain: Option<Terrain>,
    #[serde(default)] 
    pub has_sheer_force: bool,
    pub self_destruct: Option<SelfDestruct>,
    pub pseudo_weather: Option<PseudoWeather>,
    #[serde(default)] 
    pub breaks_protect: bool,
    // secondaries
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
    pub no_pp_boosts: bool,
    #[serde(default)] 
    pub sleep_usable: bool,
    #[serde(default)] 
    pub tracks_target: bool,
    #[serde(default)] 
    pub steals_boosts: bool,
    #[serde(default)] 
    pub struggle_recoil: bool,
}
impl Move {
    pub fn has_flag(&self, flag: Flag) -> bool {
        self.flags.has_flag(flag)
    }
}

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
        result.ok_or(serde::de::Error::custom(format!("{value} is not valid for ignore immunity")
    ))
    }
}

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
        result.ok_or(serde::de::Error::custom(format!("{value} is not valid for ignore immunity")
    ))
    }
}
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
        result.ok_or(serde::de::Error::custom(format!("{value} is not valid for accuracy")
    ))
    }
}
pub enum AlternativeDamage {
    Flat(u8),
    Unique(UniqueDamage)
}

#[derive(Deserialize)]
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
        result.ok_or(serde::de::Error::custom(format!("{value} is not valid for alternative damage")
    ))
    }
}

pub enum MultiHit {
    Constant(u8),
    Range(u8, u8)
}
impl<'de> Deserialize<'de> for MultiHit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let result = match &value {
            serde_json::Value::Number(n) => n.as_u64().and_then(|n| n.try_into().ok()).map(MultiHit::Constant),
            serde_json::Value::Array(a) => match &a[..] {
                [serde_json::Value::Number(a), serde_json::Value::Number(b)] =>
                    a.as_u64().and_then(|a| a.try_into().ok()).and_then(|a| 
                        b.as_u64().and_then(|b| b.try_into().ok()).map(|b| 
                            MultiHit::Range(a, b)
                        )
                    ),
                _ => None
            },
            _ => None
        };
        result.ok_or(serde::de::Error::custom(format!("{value} is not valid for accuracy")
    ))
    }
}

#[derive(Deserialize, PartialEq, Eq)]
pub enum Category {
    Physical,
    Special,
    Status,
}

#[derive(PartialEq, Eq, Hash, Deserialize, Clone, Copy)]
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
        result.ok_or(serde::de::Error::custom(format!("{value} is not a valid list of flags")))
    }
}

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
        result.ok_or(serde::de::Error::custom(format!("{value} is not a valid list of boosts")))
    }
}


#[derive(Deserialize)]
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum NonStandardReason {
    LGPE,
    Past,
    Future,
    CAP,
    Gigantamax,
    Unobtainable,
}
#[derive(Deserialize)]
pub enum ContestType {
    Clever,
    Cute,
    Tough,
    Beautiful,
    Cool,
}

#[derive(Deserialize)]
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

    // Past Volatile Statuses
    MudSport,
    Autotomize,
    WaterSport,
    Mist,
    LightScreen,
    Reflect,
}

#[derive(Deserialize)]
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
pub enum SelfSwitch {
    True,
    False,
    Unique(UniqueSelfSwitch)
}
impl Default for SelfSwitch {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Deserialize)]
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
        result.ok_or(serde::de::Error::custom(format!("{value} is not valid for a self switch")))
    }
}


#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum NonGhostTarget {
    #[serde(rename = "self")]
    Self_
}

#[derive(Deserialize)]
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
}
#[derive(Deserialize)]
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


#[derive(Deserialize)]
pub enum SelfDestruct {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "ifHit")]
    IfHit
}

#[derive(Deserialize)]
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
        result.ok_or(serde::de::Error::custom(format!("{value} is not valid for OHKO")))
    }
}
#[derive(Deserialize)]
pub enum UniqueOHKO {
    Ice
}

#[derive(Deserialize)]
pub enum OverrideOffensivePokemon {
    #[serde(rename = "target")]
    Target
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SlotCondition {
    HealingWish,
    FollowMe, // Past
    LunarDance,
    RevivalBlessing,
    #[serde(rename = "Wish")]
    Wish,
}
