use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::{generation::Generation, learnsets::Gender, moves::NonStandardReason, types::Type};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Species {
    pub num: i32,
    pub name: String,
    pub types: Vec<Type>,
    /// Generation of a custom pokemon
    pub gen: Option<Generation>,
    pub gender_ratio: Option<GenderRatio>,
    pub gender: Option<Gender>,
    pub base_stats: StatDistribution,
    pub abilities: HashMap<char, Ability>,
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

#[derive(Deserialize, PartialEq, Clone, Copy, Debug)]
#[serde(rename_all = "UPPERCASE")]
#[serde(deny_unknown_fields)]
pub struct GenderRatio {
    pub f: f32,
    pub m: f32
}

#[derive(Deserialize, Default, PartialEq, Eq, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub struct StatDistribution {
    #[serde(default)]
    pub hp: u8,
    #[serde(default)]
    pub atk: u8,
    #[serde(default)]
    pub def: u8,
    #[serde(default)]
    pub spa: u8,
    #[serde(default)]
    pub spd: u8,
    #[serde(default)]
    pub spe: u8
}
impl StatDistribution {
    pub fn get(&self, stat: Stat) -> u8 {
        match stat {
            Stat::HP => self.hp,
            Stat::Attack => self.atk,
            Stat::Defence => self.def,
            Stat::SpecialAttack => self.spa,
            Stat::SpecialDefence => self.spd,
            Stat::Speed => self.spe,
            _ => todo!()
        }
    }
    pub fn get_mut(&mut self, stat: Stat) -> &mut u8 {
        match stat {
            Stat::HP => &mut self.hp,
            Stat::Attack => &mut self.atk,
            Stat::Defence => &mut self.def,
            Stat::SpecialAttack => &mut self.spa,
            Stat::SpecialDefence => &mut self.spd,
            Stat::Speed => &mut self.spe,
            _ => todo!()
        }
    }
}
impl From<[u8;6]> for StatDistribution {
    fn from(value: [u8;6]) -> Self {
        Self { hp: value[0], atk: value[1], def: value[2], spa: value[3], spd: value[4], spe: value[5] }
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

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ability {
    #[serde(rename = "")]
    NoAbility,

    Adaptability,
    Aerilate,
    Aftermath,
    #[serde(rename = "Air Lock")]
    AirLock,
    Analytic,
    #[serde(rename = "Anger Point")]
    AngerPoint,
    #[serde(rename = "Anger Shell")]
    AngerShell,
    Anticipation,
    #[serde(rename = "Arena Trap")]
    ArenaTrap,
    #[serde(rename = "Armor Tail")]
    ArmorTail,
    #[serde(rename = "Aroma Veil")]
    AromaVeil,
    #[serde(rename = "As One (Glastrier)")]
    AsOneGlastrier,
    #[serde(rename = "As One (Spectrier)")]
    AsOneSpectrier,
    #[serde(rename = "Aura Break")]
    AuraBreak,
    #[serde(rename = "Bad Dreams")]
    BadDreams,
    #[serde(rename = "Ball Fetch")]
    BallFetch,
    Battery,
    #[serde(rename = "Battle Armor")]
    BattleArmor,
    #[serde(rename = "Battle Bond")]
    BattleBond,
    #[serde(rename = "Beads of Ruin")]
    BeadsofRuin,
    #[serde(rename = "Beast Boost")]
    BeastBoost,
    Berserk,
    #[serde(rename = "Big Pecks")]
    BigPecks,
    Blaze,
    Bulletproof,
    #[serde(rename = "Cheek Pouch")]
    CheekPouch,
    #[serde(rename = "Chilling Neigh")]
    ChillingNeigh,
    Chlorophyll,
    #[serde(rename = "Clear Body")]
    ClearBody,
    #[serde(rename = "Cloud Nine")]
    CloudNine,
    #[serde(rename = "Color Change")]
    ColorChange,
    Comatose,
    Commander,
    Competitive,
    #[serde(rename = "Compound Eyes")]
    CompoundEyes,
    Contrary,
    Corrosion,
    Costar,
    #[serde(rename = "Cotton Down")]
    CottonDown,
    #[serde(rename = "Cud Chew")]
    CudChew,
    #[serde(rename = "Curious Medicine")]
    CuriousMedicine,
    #[serde(rename = "Cursed Body")]
    CursedBody,
    #[serde(rename = "Cute Charm")]
    CuteCharm,
    Damp,
    Dancer,
    #[serde(rename = "Dark Aura")]
    DarkAura,
    #[serde(rename = "Dauntless Shield")]
    DauntlessShield,
    Dazzling,
    Defeatist,
    Defiant,
    #[serde(rename = "Delta Stream")]
    DeltaStream,
    #[serde(rename = "Desolate Land")]
    DesolateLand,
    Disguise,
    Download,
    #[serde(rename = "Dragon's Maw")]
    DragonsMaw,
    Drizzle,
    Drought,
    #[serde(rename = "Dry Skin")]
    DrySkin,
    #[serde(rename = "Early Bird")]
    EarlyBird,
    #[serde(rename = "Earth Eater")]
    EarthEater,
    #[serde(rename = "Effect Spore")]
    EffectSpore,
    #[serde(rename = "Electric Surge")]
    ElectricSurge,
    Electromorphosis,
    #[serde(rename = "Embody Aspect (Cornerstone)")]
    EmbodyAspectCornerstone,
    #[serde(rename = "Embody Aspect (Hearthflame)")]
    EmbodyAspectHearthflame,
    #[serde(rename = "Embody Aspect (Teal)")]
    EmbodyAspectTeal,
    #[serde(rename = "Embody Aspect (Wellspring)")]
    EmbodyAspectWellspring,
    #[serde(rename = "Emergency Exit")]
    EmergencyExit,
    #[serde(rename = "Fairy Aura")]
    FairyAura,
    Filter,
    #[serde(rename = "Flame Body")]
    FlameBody,
    #[serde(rename = "Flare Boost")]
    FlareBoost,
    #[serde(rename = "Flash Fire")]
    FlashFire,
    #[serde(rename = "Flower Gift")]
    FlowerGift,
    #[serde(rename = "Flower Veil")]
    FlowerVeil,
    Fluffy,
    Forecast,
    Forewarn,
    #[serde(rename = "Friend Guard")]
    FriendGuard,
    Frisk,
    #[serde(rename = "Full Metal Body")]
    FullMetalBody,
    #[serde(rename = "Fur Coat")]
    FurCoat,
    #[serde(rename = "Gale Wings")]
    GaleWings,
    Galvanize,
    Gluttony,
    #[serde(rename = "Good as Gold")]
    GoodasGold,
    Gooey,
    #[serde(rename = "Gorilla Tactics")]
    GorillaTactics,
    #[serde(rename = "Grass Pelt")]
    GrassPelt,
    #[serde(rename = "Grassy Surge")]
    GrassySurge,
    #[serde(rename = "Grim Neigh")]
    GrimNeigh,
    #[serde(rename = "Guard Dog")]
    GuardDog,
    #[serde(rename = "Gulp Missile")]
    GulpMissile,
    Guts,
    #[serde(rename = "Hadron Engine")]
    HadronEngine,
    Harvest,
    Healer,
    Heatproof,
    #[serde(rename = "Heavy Metal")]
    HeavyMetal,
    #[serde(rename = "Honey Gather")]
    HoneyGather,
    Hospitality,
    #[serde(rename = "Huge Power")]
    HugePower,
    #[serde(rename = "Hunger Switch")]
    HungerSwitch,
    Hustle,
    Hydration,
    #[serde(rename = "Hyper Cutter")]
    HyperCutter,
    #[serde(rename = "Ice Body")]
    IceBody,
    #[serde(rename = "Ice Face")]
    IceFace,
    #[serde(rename = "Ice Scales")]
    IceScales,
    Illuminate,
    Illusion,
    Immunity,
    Imposter,
    Infiltrator,
    #[serde(rename = "Innards Out")]
    InnardsOut,
    #[serde(rename = "Inner Focus")]
    InnerFocus,
    Insomnia,
    Intimidate,
    #[serde(rename = "Intrepid Sword")]
    IntrepidSword,
    #[serde(rename = "Iron Barbs")]
    IronBarbs,
    #[serde(rename = "Iron Fist")]
    IronFist,
    Justified,
    #[serde(rename = "Keen Eye")]
    KeenEye,
    Klutz,
    #[serde(rename = "Leaf Guard")]
    LeafGuard,
    Levitate,
    Libero,
    #[serde(rename = "Light Metal")]
    LightMetal,
    #[serde(rename = "Lightning Rod")]
    LightningRod,
    Limber,
    #[serde(rename = "Lingering Aroma")]
    LingeringAroma,
    #[serde(rename = "Liquid Ooze")]
    LiquidOoze,
    #[serde(rename = "Liquid Voice")]
    LiquidVoice,
    #[serde(rename = "Long Reach")]
    LongReach,
    #[serde(rename = "Magic Bounce")]
    MagicBounce,
    #[serde(rename = "Magic Guard")]
    MagicGuard,
    Magician,
    #[serde(rename = "Magma Armor")]
    MagmaArmor,
    #[serde(rename = "Magnet Pull")]
    MagnetPull,
    #[serde(rename = "Marvel Scale")]
    MarvelScale,
    #[serde(rename = "Mega Launcher")]
    MegaLauncher,
    Merciless,
    Mimicry,
    #[serde(rename = "Mind's Eye")]
    MindsEye,
    Minus,
    #[serde(rename = "Mirror Armor")]
    MirrorArmor,
    #[serde(rename = "Misty Surge")]
    MistySurge,
    #[serde(rename = "Mold Breaker")]
    MoldBreaker,
    Moody,
    #[serde(rename = "Motor Drive")]
    MotorDrive,
    Mountaineer,
    Moxie,
    Multiscale,
    Multitype,
    Mummy,
    #[serde(rename = "Mycelium Might")]
    MyceliumMight,
    #[serde(rename = "Natural Cure")]
    NaturalCure,
    Neuroforce,
    #[serde(rename = "Neutralizing Gas")]
    NeutralizingGas,
    #[serde(rename = "No Guard")]
    NoGuard,
    Normalize,
    Oblivious,
    Opportunist,
    #[serde(rename = "Orichalcum Pulse")]
    OrichalcumPulse,
    Overcoat,
    Overgrow,
    #[serde(rename = "Own Tempo")]
    OwnTempo,
    #[serde(rename = "Parental Bond")]
    ParentalBond,
    #[serde(rename = "Pastel Veil")]
    PastelVeil,
    #[serde(rename = "Perish Body")]
    PerishBody,
    Persistent,
    Pickpocket,
    Pickup,
    Pixilate,
    Plus,
    #[serde(rename = "Poison Heal")]
    PoisonHeal,
    #[serde(rename = "Poison Point")]
    PoisonPoint,
    #[serde(rename = "Poison Puppeteer")]
    PoisonPuppeteer,
    #[serde(rename = "Poison Touch")]
    PoisonTouch,
    #[serde(rename = "Power Construct")]
    PowerConstruct,
    #[serde(rename = "Power Spot")]
    PowerSpot,
    #[serde(rename = "Power of Alchemy")]
    PowerofAlchemy,
    Prankster,
    Pressure,
    #[serde(rename = "Primordial Sea")]
    PrimordialSea,
    #[serde(rename = "Prism Armor")]
    PrismArmor,
    #[serde(rename = "Propeller Tail")]
    PropellerTail,
    Protean,
    Protosynthesis,
    #[serde(rename = "Psychic Surge")]
    PsychicSurge,
    #[serde(rename = "Punk Rock")]
    PunkRock,
    #[serde(rename = "Pure Power")]
    PurePower,
    #[serde(rename = "Purifying Salt")]
    PurifyingSalt,
    #[serde(rename = "Quark Drive")]
    QuarkDrive,
    #[serde(rename = "Queenly Majesty")]
    QueenlyMajesty,
    #[serde(rename = "Quick Draw")]
    QuickDraw,
    #[serde(rename = "Quick Feet")]
    QuickFeet,
    #[serde(rename = "RKS System")]
    RKSSystem,
    #[serde(rename = "Rain Dish")]
    RainDish,
    Rattled,
    Rebound,
    Receiver,
    Reckless,
    Refrigerate,
    Regenerator,
    Ripen,
    Rivalry,
    #[serde(rename = "Rock Head")]
    RockHead,
    #[serde(rename = "Rocky Payload")]
    RockyPayload,
    #[serde(rename = "Rough Skin")]
    RoughSkin,
    #[serde(rename = "Run Away")]
    RunAway,
    #[serde(rename = "Sand Force")]
    SandForce,
    #[serde(rename = "Sand Rush")]
    SandRush,
    #[serde(rename = "Sand Spit")]
    SandSpit,
    #[serde(rename = "Sand Stream")]
    SandStream,
    #[serde(rename = "Sand Veil")]
    SandVeil,
    #[serde(rename = "Sap Sipper")]
    SapSipper,
    Schooling,
    Scrappy,
    #[serde(rename = "Screen Cleaner")]
    ScreenCleaner,
    #[serde(rename = "Seed Sower")]
    SeedSower,
    #[serde(rename = "Serene Grace")]
    SereneGrace,
    #[serde(rename = "Shadow Shield")]
    ShadowShield,
    #[serde(rename = "Shadow Tag")]
    ShadowTag,
    Sharpness,
    #[serde(rename = "Shed Skin")]
    ShedSkin,
    #[serde(rename = "Sheer Force")]
    SheerForce,
    #[serde(rename = "Shell Armor")]
    ShellArmor,
    #[serde(rename = "Shield Dust")]
    ShieldDust,
    #[serde(rename = "Shields Down")]
    ShieldsDown,
    Simple,
    #[serde(rename = "Skill Link")]
    SkillLink,
    #[serde(rename = "Slow Start")]
    SlowStart,
    #[serde(rename = "Slush Rush")]
    SlushRush,
    Sniper,
    #[serde(rename = "Snow Cloak")]
    SnowCloak,
    #[serde(rename = "Snow Warning")]
    SnowWarning,
    #[serde(rename = "Solar Power")]
    SolarPower,
    #[serde(rename = "Solid Rock")]
    SolidRock,
    #[serde(rename = "Soul-Heart")]
    SoulHeart,
    Soundproof,
    #[serde(rename = "Speed Boost")]
    SpeedBoost,
    Stakeout,
    Stall,
    Stalwart,
    Stamina,
    #[serde(rename = "Stance Change")]
    StanceChange,
    Static,
    Steadfast,
    #[serde(rename = "Steam Engine")]
    SteamEngine,
    Steelworker,
    #[serde(rename = "Steely Spirit")]
    SteelySpirit,
    Stench,
    #[serde(rename = "Sticky Hold")]
    StickyHold,
    #[serde(rename = "Storm Drain")]
    StormDrain,
    #[serde(rename = "Strong Jaw")]
    StrongJaw,
    Sturdy,
    #[serde(rename = "Suction Cups")]
    SuctionCups,
    #[serde(rename = "Super Luck")]
    SuperLuck,
    #[serde(rename = "Supersweet Syrup")]
    SupersweetSyrup,
    #[serde(rename = "Supreme Overlord")]
    SupremeOverlord,
    #[serde(rename = "Surge Surfer")]
    SurgeSurfer,
    Swarm,
    #[serde(rename = "Sweet Veil")]
    SweetVeil,
    #[serde(rename = "Swift Swim")]
    SwiftSwim,
    #[serde(rename = "Sword of Ruin")]
    SwordofRuin,
    Symbiosis,
    Synchronize,
    #[serde(rename = "Tablets of Ruin")]
    TabletsofRuin,
    #[serde(rename = "Tangled Feet")]
    TangledFeet,
    #[serde(rename = "Tangling Hair")]
    TanglingHair,
    Technician,
    Telepathy,
    #[serde(rename = "Tera Shell")]
    TeraShell,
    #[serde(rename = "Tera Shift")]
    TeraShift,
    #[serde(rename = "Teraform Zero")]
    TeraformZero,
    Teravolt,
    #[serde(rename = "Thermal Exchange")]
    ThermalExchange,
    #[serde(rename = "Thick Fat")]
    ThickFat,
    #[serde(rename = "Tinted Lens")]
    TintedLens,
    Torrent,
    #[serde(rename = "Tough Claws")]
    ToughClaws,
    #[serde(rename = "Toxic Boost")]
    ToxicBoost,
    #[serde(rename = "Toxic Chain")]
    ToxicChain,
    #[serde(rename = "Toxic Debris")]
    ToxicDebris,
    Trace,
    Transistor,
    Triage,
    Truant,
    Turboblaze,
    Unaware,
    Unburden,
    Unnerve,
    #[serde(rename = "Unseen Fist")]
    UnseenFist,
    #[serde(rename = "Vessel of Ruin")]
    VesselofRuin,
    #[serde(rename = "Victory Star")]
    VictoryStar,
    #[serde(rename = "Vital Spirit")]
    VitalSpirit,
    #[serde(rename = "Volt Absorb")]
    VoltAbsorb,
    #[serde(rename = "Wandering Spirit")]
    WanderingSpirit,
    #[serde(rename = "Water Absorb")]
    WaterAbsorb,
    #[serde(rename = "Water Bubble")]
    WaterBubble,
    #[serde(rename = "Water Compaction")]
    WaterCompaction,
    #[serde(rename = "Water Veil")]
    WaterVeil,
    #[serde(rename = "Weak Armor")]
    WeakArmor,
    #[serde(rename = "Well-Baked Body")]
    WellBakedBody,
    #[serde(rename = "White Smoke")]
    WhiteSmoke,
    #[serde(rename = "Wimp Out")]
    WimpOut,
    #[serde(rename = "Wind Power")]
    WindPower,
    #[serde(rename = "Wind Rider")]
    WindRider,
    #[serde(rename = "Wonder Guard")]
    WonderGuard,
    #[serde(rename = "Wonder Skin")]
    WonderSkin,
    #[serde(rename = "Zen Mode")]
    ZenMode,
    #[serde(rename = "Zero to Hero")]
    ZerotoHero,
}