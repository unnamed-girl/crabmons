use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{items::Priorities, parsing_utils::{deserialize_via, impl_from_either, impl_try_from_either, Either, NotImplemented}, species::Stat, types::Type};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MoveData {
    pub num: i32,
    pub accuracy: Accuracy,
    pub base_power: u8,
    pub category: Category,
    /// `Some(reason)`: this move is typically illegal in this generation for the given reason\
    pub is_nonstandard: Option<NonStandardReason>,
    pub name: String,
    pub pp: u8,
    pub priority: i8,
    pub flags: FlagList,
    #[serde(default)]
    pub crit_ratio: CritRatio,
    #[serde(deserialize_with = "deserialize_via::<_, Option<Vec<Secondary>>, Either<Option<Secondary>, Vec<Secondary>>>")]
    #[serde(default, alias = "secondary")]
    pub secondaries: Option<Vec<Secondary>>,
    pub target: Target,
    pub type_: Type,
    pub contest_type: Option<ContestType>,
    #[serde(default, rename = "desc")]
    pub description: String,
    #[serde(default, rename = "shortDesc")]
    pub short_description: String,
    /// Percentage of the damage the move deals that the user heals.
    pub drain: Option<PokeFraction>,
    #[serde(rename = "isZ")]
    pub z_crystal: Option<String>,
    pub z_move: Option<ZMoveData>,
    pub condition: Option<Condition>, // TODO
    #[serde(default, rename = "multihit")]
    pub number_of_hits: NumberOfHits,
    /// Whether this move causes the user to use a different move. See Metronome
    #[serde(default)] 
    pub calls_move: bool,
    /// Whether the user loses 50% of their hp on a miss. See High Jump Kick
    #[serde(default)] 
    pub has_crash_damage: bool,
    /// Whether this move has a scaling chance to fail with each successful consecutive use. See Protect.
    #[serde(default)] 
    pub stalling_move: bool,
    #[serde(default)]
    pub self_switch: SelfSwitch,
    #[serde(default)]
    pub ignore_immunity: IgnoreImmunity,
    /// The offensive stat to use instead of attack/special attack. See body press.
    pub override_offensive_stat: Option<Stat>,
    #[serde(default)]
    pub is_max: IsMaxMove, 
    pub max_move: Option<MaxMoveData>,
    /// Percentage of damage dealt the user takes as recoil damage.
    pub recoil: Option<PokeFraction>,
    /// Ignores the target's defensive stat changes. See Darkiast Lariat.
    #[serde(default)] 
    pub ignore_defensive: bool,
    /// Ignore's the target's evasive stat changes. See Darkest Lariat
    #[serde(default)] 
    pub ignore_evasion: bool,
    /// Whether this move forces the opponent to switch out. See Roar.
    #[serde(default)] 
    pub force_switch: bool,
    /// `Some(target)`: this move's target when not used by a ghost type. See Curse.
    pub non_ghost_target: Option<Target>,
    /// In doubles, hit each enemy once. Avoids immune/protecting pokemon, hitting their partner twice instead. See Dragon Darts
    #[serde(default)] 
    pub smart_target: bool,
    pub damage: Option<AlternativeDamage>,
    /// Benefits from sheer force but secondary effects still happen. See Electro Shot.
    #[serde(default)]
    pub has_sheer_force: bool,
    pub selfdestruct: Option<SelfDestruct>,
    /// See feint.
    #[serde(default)] 
    pub breaks_protect: bool,
    pub ohko: Option<OHKO>,
    /// Whether the move always crits. See Surging Strikes.
    #[serde(default)] 
    pub will_crit: bool,
    /// The source to use for the offensive stat rather than the user. See Foul Play
    pub override_offensive_pokemon: Option<OverrideOffensivePokemon>,
    #[serde(default)] 
    pub ignore_ability: bool,
    pub slot_condition: Option<SlotCondition>,
    /// Percentage of their hp the target(s) recover. See Recover
    pub heal: Option<PokeFraction>,
    /// The name of the real move, if this is a derivative move. See Hidden Power Fire
    pub real_move: Option<String>,
    #[serde(default)] 
    pub thaws_target: bool,
    /// Whether the user loses half of their hp, even if the move fails. See Mind Blown
    #[serde(default)] 
    pub mind_blown_recoil: bool,
    /// The defensive stat to use rather than the expected stat for this category. See Psystrike.
    pub override_defensive_stat: Option<Stat>,
    /// Whether this move's default PP is also its max PP.
    #[serde(default, rename = "noPPBoosts")] 
    pub no_pp_boosts: bool,
    /// Whether this move can be used while sleeping. See Sleep Talk
    #[serde(default)] 
    pub sleep_usable: bool,
    /// If true, this move cannot be redirected. See Snipe Shot.
    #[serde(default)] 
    pub tracks_target: bool,
    /// If true, the target's positive stat changes are stolen before dealing damage. See Spectral Thief
    #[serde(default)] 
    pub steals_boosts: bool,
    /// Whether the move costs the user 25% of its hp if successful.\
    /// Doesn't count as recoil for the purposes of Rock Head, Reckless, Magic Guard, etc. 
    #[serde(default)] 
    pub struggle_recoil: bool,
    /// Whether the move checks for accuracy between each hit. See Population Bomb.
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
impl MoveData {
    pub fn has_flag(&self, flag: Flag) -> bool {
        self.flags.has_flag(flag)
    }
}

impl From<Either<Option<Secondary>, Vec<Secondary>>> for Option<Vec<Secondary>> {
    fn from(value: Either<Option<Secondary>, Vec<Secondary>>) -> Self {
        match value {
            Either::A(Some(a)) => Some(vec![a]),
            Either::A(None) => Some(Vec::new()),
            Either::B(b) => Some(b),
        }
    }
}

/// `Raised(1)` represents +1 crit ratio stage.\
/// The conversion from u8 assumes from Showdown in which 1 maps to Standard, 2 maps to Raised(1) etc.
#[derive(Default, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(try_from = "u8")]
pub enum CritRatio {
    #[default]
    Standard,
    Raised(u8)
}
impl TryFrom<u8> for CritRatio {
    type Error = NotImplemented;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Err(NotImplemented("CritRatio starts at 1, 0 is invalid.")),
            1 => Ok(Self::Standard),
            2.. => Ok(Self::Raised(value - 1))
        }
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
#[derive(Deserialize, Clone, Copy)]
#[serde(from = "[u8; 2]")]
pub struct PokeFraction {
    pub numerator: u8,
    pub demoninator: u8,
}
impl From<PokeFraction> for f32 {
    fn from(value: PokeFraction) -> Self {
        value.numerator as f32 / value.demoninator as f32
    }
}
impl From<[u8; 2]> for PokeFraction {
    fn from(value: [u8; 2]) -> Self {
        Self { numerator: value[0], demoninator: value[1] }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Secondary {
    #[serde(default)]
    pub dustproof: bool, // Whether this gets stopped by shield dust

    #[serde(flatten)]
    pub target_effects: MoveEffects,
    #[serde(rename = "self")]
    pub self_effects: Option<MoveEffects>,
}

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MoveEffects {
    pub chance: Option<u8>,
    pub boosts: Option<BoostsList>,
    pub volatile_status: Option<VolatileStatus>,
    pub side_condition: Option<SideCondition>,
    pub pseudo_weather: Option<PseudoWeather>,
    pub weather: Option<Weather>,
    pub terrain: Option<Terrain>,
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

#[derive(Deserialize, Default, PartialEq, Eq, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub struct BoostsList {
    #[serde(default)]
    pub hp: i8,
    #[serde(default, rename = "atk")]
    pub attack: i8,
    #[serde(default, rename = "def")]
    pub defence: i8,
    #[serde(default, rename = "spa")]
    pub special_attack: i8,
    #[serde(default, rename = "spd")]
    pub special_defence: i8,
    #[serde(default, rename = "spe")]
    pub speed: i8,
    #[serde(default)]
    pub evasion: i8,
    #[serde(default)]
    pub accuracy: i8,    
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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ZMoveData {
    pub base_power: Option<u8>,
    pub boost: Option<BoostsList>,
    pub effect: Option<String> //TODO
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaxMoveData {
    pub base_power: Option<u8>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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