use std::str::FromStr;

use serde::Deserialize;
use strum::EnumString;

use crate::dex::DexError;

use super::IdentifierName;

impl TryFrom<IdentifierName> for Ability {
    type Error = DexError;
    fn try_from(value: IdentifierName) -> Result<Self, Self::Error> {
        Ability::from_str(value.inner()).map_err(|_| DexError::NotFound(value.inner().to_string()))
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Debug)]
#[serde(rename_all = "lowercase", try_from = "IdentifierName")]
#[strum(serialize_all = "lowercase", use_phf)]
pub enum Ability {
    #[serde(rename = "")]
    #[strum(serialize = "")]
    NoAbility,

    Adaptability,
    Aerilate,
    Aftermath,
    AirLock,
    Analytic,
    AngerPoint,
    AngerShell,
    Anticipation,
    ArenaTrap,
    ArmorTail,
    AromaVeil,
    AsOneGlastrier,
    AsOneSpectrier,
    AuraBreak,
    BadDreams,
    BallFetch,
    Battery,
    BattleArmor,
    BattleBond,
    BeadsofRuin,
    BeastBoost,
    Berserk,
    BigPecks,
    Blaze,
    Bulletproof,
    CheekPouch,
    ChillingNeigh,
    Chlorophyll,
    ClearBody,
    CloudNine,
    ColorChange,
    Comatose,
    Commander,
    Competitive,
    CompoundEyes,
    Contrary,
    Corrosion,
    Costar,
    CottonDown,
    CudChew,
    CuriousMedicine,
    CursedBody,
    CuteCharm,
    Damp,
    Dancer,
    DarkAura,
    DauntlessShield,
    Dazzling,
    Defeatist,
    Defiant,
    DeltaStream,
    DesolateLand,
    Disguise,
    Download,
    DragonsMaw,
    Drizzle,
    Drought,
    DrySkin,
    EarlyBird,
    EarthEater,
    EffectSpore,
    ElectricSurge,
    Electromorphosis,
    EmbodyAspectCornerstone,
    EmbodyAspectHearthflame,
    EmbodyAspectTeal,
    EmbodyAspectWellspring,
    EmergencyExit,
    FairyAura,
    Filter,
    FlameBody,
    FlareBoost,
    FlashFire,
    FlowerGift,
    FlowerVeil,
    Fluffy,
    Forecast,
    Forewarn,
    FriendGuard,
    Frisk,
    FullMetalBody,
    FurCoat,
    GaleWings,
    Galvanize,
    Gluttony,
    GoodasGold,
    Gooey,
    GorillaTactics,
    GrassPelt,
    GrassySurge,
    GrimNeigh,
    GuardDog,
    GulpMissile,
    Guts,
    HadronEngine,
    Harvest,
    Healer,
    Heatproof,
    HeavyMetal,
    HoneyGather,
    Hospitality,
    HugePower,
    HungerSwitch,
    Hustle,
    Hydration,
    HyperCutter,
    IceBody,
    IceFace,
    IceScales,
    Illuminate,
    Illusion,
    Immunity,
    Imposter,
    Infiltrator,
    InnardsOut,
    InnerFocus,
    Insomnia,
    Intimidate,
    IntrepidSword,
    IronBarbs,
    IronFist,
    Justified,
    KeenEye,
    Klutz,
    LeafGuard,
    Levitate,
    Libero,
    LightMetal,
    LightningRod,
    Limber,
    LingeringAroma,
    LiquidOoze,
    LiquidVoice,
    LongReach,
    MagicBounce,
    MagicGuard,
    Magician,
    MagmaArmor,
    MagnetPull,
    MarvelScale,
    MegaLauncher,
    Merciless,
    Mimicry,
    MindsEye,
    Minus,
    MirrorArmor,
    MistySurge,
    MoldBreaker,
    Moody,
    MotorDrive,
    Mountaineer,
    Moxie,
    Multiscale,
    Multitype,
    Mummy,
    MyceliumMight,
    NaturalCure,
    Neuroforce,
    NeutralizingGas,
    NoGuard,
    Normalize,
    Oblivious,
    Opportunist,
    OrichalcumPulse,
    Overcoat,
    Overgrow,
    OwnTempo,
    ParentalBond,
    PastelVeil,
    PerishBody,
    Persistent,
    Pickpocket,
    Pickup,
    Pixilate,
    Plus,
    PoisonHeal,
    PoisonPoint,
    PoisonPuppeteer,
    PoisonTouch,
    PowerConstruct,
    PowerSpot,
    PowerofAlchemy,
    Prankster,
    Pressure,
    PrimordialSea,
    PrismArmor,
    PropellerTail,
    Protean,
    Protosynthesis,
    PsychicSurge,
    PunkRock,
    PurePower,
    PurifyingSalt,
    QuarkDrive,
    QueenlyMajesty,
    QuickDraw,
    QuickFeet,
    RKSSystem,
    RainDish,
    Rattled,
    Rebound,
    Receiver,
    Reckless,
    Refrigerate,
    Regenerator,
    Ripen,
    Rivalry,
    RockHead,
    RockyPayload,
    RoughSkin,
    RunAway,
    SandForce,
    SandRush,
    SandSpit,
    SandStream,
    SandVeil,
    SapSipper,
    Schooling,
    Scrappy,
    ScreenCleaner,
    SeedSower,
    SereneGrace,
    ShadowShield,
    ShadowTag,
    Sharpness,
    ShedSkin,
    SheerForce,
    ShellArmor,
    ShieldDust,
    ShieldsDown,
    Simple,
    SkillLink,
    SlowStart,
    SlushRush,
    Sniper,
    SnowCloak,
    SnowWarning,
    SolarPower,
    SolidRock,
    SoulHeart,
    Soundproof,
    SpeedBoost,
    Stakeout,
    Stall,
    Stalwart,
    Stamina,
    StanceChange,
    Static,
    Steadfast,
    SteamEngine,
    Steelworker,
    SteelySpirit,
    Stench,
    StickyHold,
    StormDrain,
    StrongJaw,
    Sturdy,
    SuctionCups,
    SuperLuck,
    SupersweetSyrup,
    SupremeOverlord,
    SurgeSurfer,
    Swarm,
    SweetVeil,
    SwiftSwim,
    SwordofRuin,
    Symbiosis,
    Synchronize,
    TabletsofRuin,
    TangledFeet,
    TanglingHair,
    Technician,
    Telepathy,
    TeraShell,
    TeraShift,
    TeraformZero,
    Teravolt,
    ThermalExchange,
    ThickFat,
    TintedLens,
    Torrent,
    ToughClaws,
    ToxicBoost,
    ToxicChain,
    ToxicDebris,
    Trace,
    Transistor,
    Triage,
    Truant,
    Turboblaze,
    Unaware,
    Unburden,
    Unnerve,
    UnseenFist,
    VesselofRuin,
    VictoryStar,
    VitalSpirit,
    VoltAbsorb,
    WanderingSpirit,
    WaterAbsorb,
    WaterBubble,
    WaterCompaction,
    WaterVeil,
    WeakArmor,
    WellBakedBody,
    WhiteSmoke,
    WimpOut,
    WindPower,
    WindRider,
    WonderGuard,
    WonderSkin,
    ZenMode,
    ZerotoHero,
}