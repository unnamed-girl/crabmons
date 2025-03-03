use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{items::Priorities, parsing_utils::{impl_from_either, impl_try_from_either, Either, NotImplemented, deserialize_via}, species::Stat, types::Type};

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
    #[serde(rename = "multihit")]
    #[serde(default)]
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
    #[serde(default)]
    pub is_max: IsMaxMove, 
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

    #[serde(deserialize_with = "deserialize_via::<_, Option<BoostsList>, _SelfBoost>")]
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

#[derive(Deserialize)]
struct _SelfBoost {
    boosts: BoostsList
}
impl From<_SelfBoost> for Option<BoostsList> {
    fn from(value: _SelfBoost) -> Self {
        Some(value.boosts)
    }
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

#[derive(Clone, Deserialize, Debug, Default)]
#[serde(from = "Either<bool, String>")]
pub enum IsMaxMove {
    #[default]
    NotAMaxMove,
    GenericMaxMove,
    ForPokemon(String)
}
impl_from_either!(IsMaxMove, bool, String);
impl From<bool> for IsMaxMove {
    fn from(value: bool) -> Self {
        if value {
            Self::GenericMaxMove
        } else {
            Self::NotAMaxMove
        }
    }
}
impl From<String> for IsMaxMove {
    fn from(value: String) -> Self {
        Self::ForPokemon(value)
    }
}
impl From<IsMaxMove> for bool {
    fn from(value: IsMaxMove) -> Self {
        match value {
            IsMaxMove::GenericMaxMove | IsMaxMove::ForPokemon(_) => true,
            IsMaxMove::NotAMaxMove => false,
        }
    }
}

#[derive(Clone, Deserialize, Debug, Default)]
#[serde(from = "Either<bool, HashMap<Type, bool>>")]
pub enum IgnoreImmunity {
    #[default]
    DoesntIgnoreImmunity,
    IgnoresImmunity,
    Types(Vec<Type>)
}
type IgnoreImmunityTypeMap = HashMap<Type, bool>;
impl_from_either!(IgnoreImmunity, bool, IgnoreImmunityTypeMap);

impl From<bool> for IgnoreImmunity {
    fn from(value: bool) -> Self {
        if value {
            Self::IgnoresImmunity
        } else {
            Self::DoesntIgnoreImmunity
        }
    }
}
impl From<HashMap<Type, bool>> for IgnoreImmunity {
    fn from(value: HashMap<Type, bool>) -> Self {
        Self::Types(
            value.into_iter().filter(|(_, immune)| *immune)
                .map(|(type_, _)| type_)
                .collect()
        )
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(try_from = "Either<bool, u8>")]
pub enum Accuracy {
    AlwaysHits,
    Percent(u8)
}
impl_try_from_either!(Accuracy, bool, u8, NotImplemented, Infallible);
impl TryFrom<bool> for Accuracy {
    type Error = NotImplemented;
    fn try_from(value: bool) -> Result<Self, Self::Error> {
        if value {
            Ok(Self::AlwaysHits)
        } else {
            Err(NotImplemented("False accuracy not implemented"))
        }
    }
}
impl From<u8> for Accuracy {
    fn from(value: u8) -> Self {
        Self::Percent(value)
    }
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(from = "Either<u8, UniqueDamage>")]
pub enum AlternativeDamage {
    Flat(u8),
    Unique(UniqueDamage)
}
impl_from_either!(AlternativeDamage, u8, UniqueDamage);
impl From<u8> for AlternativeDamage {
    fn from(value: u8) -> Self {
        Self::Flat(value)
    }
}
impl From<UniqueDamage> for AlternativeDamage {
    fn from(value: UniqueDamage) -> Self {
        Self::Unique(value)
    }
}


#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum UniqueDamage {
    #[serde(rename = "level")]
    Level
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[serde(from = "Either<u8, [u8; 2]>")]
pub enum NumberOfHits {
    #[default]
    Normal,
    Constant(u8),
    Range(u8, u8)
}

type NumberOfHitsRange = [u8; 2];
impl_from_either!(NumberOfHits, u8, NumberOfHitsRange);

impl From<u8> for NumberOfHits {
    fn from(value: u8) -> Self {
        Self::Constant(value)
    }
}
impl From<[u8; 2]> for NumberOfHits {
    fn from(value: [u8; 2]) -> Self {
        Self::Range(value[0], value[1])
    }
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

#[derive(Deserialize, Clone, Debug)]
#[serde(from = "HashMap<Flag, u8>")]
pub struct FlagList(pub Vec<Flag>);
impl FlagList {
    pub fn has_flag(&self, flag: Flag) -> bool {
        self.0.contains(&flag)
    }
}
impl From<HashMap<Flag, u8>> for FlagList {
    fn from(value: HashMap<Flag, u8>) -> Self {
        FlagList(value.into_iter()
            .filter(|(_, has_flag)| *has_flag == 1)
            .map(|(flag, _)| flag)
            .collect())
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(from = "HashMap<Stat, i8>")]
pub struct BoostsList(pub Vec<(Stat, i8)>);
impl From<HashMap<Stat, i8>> for BoostsList {
    fn from(value: HashMap<Stat, i8>) -> Self {
        BoostsList(value.into_iter().collect())
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

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(from = "Either<bool, UniqueSelfSwitch>")]
pub enum SelfSwitch {
    True,
    #[default]
    False,
    Unique(UniqueSelfSwitch)
}
impl_from_either!(SelfSwitch, bool, UniqueSelfSwitch);
impl From<bool> for SelfSwitch {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}
impl From<UniqueSelfSwitch> for SelfSwitch {
    fn from(value: UniqueSelfSwitch) -> Self {
        Self::Unique(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UniqueSelfSwitch {
    CopyVolatile,
    ShedTail,
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

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(try_from = "Either<bool, UniqueOHKO>")]
pub enum OHKO {
    Regular,
    Unique(UniqueOHKO)
}
impl_try_from_either!(OHKO, bool, UniqueOHKO, NotImplemented, Infallible);
impl TryFrom<bool> for OHKO {
    type Error = NotImplemented;
    fn try_from(value: bool) -> Result<Self, Self::Error> {
        if value {
            Ok(Self::Regular)
        } else {
            Err(NotImplemented("False OHKO not implemented"))
        }
    }
}
impl From<UniqueOHKO> for OHKO {
    fn from(value: UniqueOHKO) -> Self {
        Self::Unique(value)
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