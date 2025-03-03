use std::fmt::{Debug, Display};

use crate::moves::{Category, Flag, Move, OverrideOffensivePokemon};
use crate::pokemon::Pokemon;
use crate::species::{Ability, Stat};
use crate::dex::{Dex, DexError, Identifier};
use crate::types::Type;

type CalcInt = u32;
type CalcFloat = f32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamageRange(pub [CalcInt; 16]);
impl DamageRange {
    pub fn new() -> Self {
        Self([85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100])
    }
    pub fn min(&self) -> CalcInt {
        self.0[0]
    }
    pub fn max(&self) -> CalcInt {
        self.0[15]
    }
    pub fn pokerounded_multiply(&mut self, value:CalcFloat) {
        self.0 = self.0.map(|damage| pokemon_round(damage as CalcFloat * value) as CalcInt);
    }
    pub fn rounded_multiply(&mut self, value:CalcFloat) {
        self.0 = self.0.map(|damage| (damage as CalcFloat * value).round() as CalcInt);
    }
    pub fn floored_multiply(&mut self, value:CalcFloat) {
        self.0 = self.0.map(|damage| (damage as CalcFloat * value).floor() as CalcInt);
    }
    pub fn ceiled_multiply(&mut self, value:CalcFloat) {
        self.0 = self.0.map(|damage| (damage as CalcFloat * value).ceil() as CalcInt);
    }
}
impl Default for DamageRange {
    fn default() -> Self {
        Self::new()
    }
}
impl PartialEq<[CalcInt; 16]> for DamageRange {
    fn eq(&self, other: &[CalcInt; 16]) -> bool {
        self.0 == *other   
    }    
}

const EPSILON:CalcFloat = 0.00001;
fn pokemon_round(value: CalcFloat) -> CalcFloat {
    (value - EPSILON).round()
}

const ZERO_DAMAGE: DamageRange = DamageRange([0;16]);

fn damage_calc(dex: &Dex, attacker: &Pokemon, defender: &Pokemon, move_: &Move, doubles: bool) -> DamageRange {
    if move_.category == Category::Status {
        return ZERO_DAMAGE;
    }

    let ignore_defender_abilities = 
        move_.ignore_ability || 
        [Ability::MoldBreaker, Ability::Turboblaze, Ability::Teravolt].contains(&attacker.ability);

    let attacker_stat_source = match move_.override_offensive_pokemon {
        None => attacker,
        Some(OverrideOffensivePokemon::Target) => defender
    };

    let (offence_stat, defence_stat) = match move_.category {
        Category::Physical => (Stat::Attack, Stat::Defence),
        Category::Special => (Stat::SpecialAttack, Stat::SpecialDefence),
        Category::Status => panic!("Status moves exit early")
    };

    let offence_stat = move_.override_defensive_stat.unwrap_or(offence_stat);
    let defence_stat = move_.override_defensive_stat.unwrap_or(defence_stat);

    let mut attack = attacker_stat_source.stat(offence_stat) as CalcFloat;
    let mut defence = defender.stat(defence_stat) as CalcFloat;
    let mut power = move_.base_power as CalcFloat;
    let mut other_modifications = 1.0;

    let multi_target = move_.target.is_multi_target();

    let mut current_move_type = move_.type_;

    // --- Type Modifications ---
    if Type::Normal == current_move_type {
        let (multiplier, alter_move_type) = match attacker.ability {
            Ability::Aerilate => {(1.2, Type::Flying)},
            Ability::Galvanize => {(1.2, Type::Electric)},
            Ability::Pixilate => {(1.2,Type::Fairy)},
            Ability::Refrigerate => {(1.2,Type::Ice)},
            _ => {(1.0, current_move_type)}
        };
        power *= multiplier;
        current_move_type = alter_move_type;
    };
    if move_.has_flag(Flag::Sound) && attacker.ability == Ability::LiquidVoice {
        current_move_type = Type::Water
    };
    if attacker.ability == Ability::Normalize && move_.is_z.is_none() && !["hiddenpower", "weatherball", "naturalgift", "technoblast", "judgment", "multiattack", "terrainpulse"].contains(&move_.name.as_str()) {
        current_move_type = Type::Normal;
    }

    // --- STAT MODIFICATIONS ---
    // TODO Blaze
    // TODO Overgrow
    // TODO Swarm
    // TODO Torrent
    attack *= match (current_move_type, attacker.ability) {
        (Type::Dragon, Ability::DragonsMaw) => 1.5,
        (Type::Rock, Ability::RockyPayload) => 1.5,
        (Type::Steel, Ability::Steelworker) => 1.5,
        (Type::Electric, Ability::Transistor) => 1.5,
        _ => 1.0,
    };


    // --- Power Modifications ---
    // Note power modifications that also modify type are in Type Modifications
    if attacker.ability == Ability::Technician && move_.base_power < 60 {power *= 1.5};
    if attacker.ability == Ability::WaterBubble && current_move_type == Type::Water {power *= 1.5};
    //TODO Analytic
    if attacker.ability == Ability::IronFist && move_.has_flag(Flag::Punch) {power *= 1.2};
    if attacker.ability == Ability::MegaLauncher && move_.has_flag(Flag::Pulse) {power *= 1.5};
    if attacker.ability == Ability::PunkRock && move_.has_flag(Flag::Sound) {power *= 1.3};
    if attacker.ability == Ability::Reckless && (move_.has_crash_damage || move_.recoil.is_some()) {power *= 1.2}; // Mind blown and struggle recoil don't count
    //TODO Rivalry
    //TODO Sand Force
    if attacker.ability == Ability::Sharpness && move_.has_flag(Flag::Slicing) {power *= 1.5};
    if attacker.ability == Ability::SheerForce && move_.has_sheer_force {power *= 1.3};
    //TODO Stakeout
    if attacker.ability == Ability::SteelySpirit && current_move_type == Type::Steel {power *= 1.5};
    if attacker.ability == Ability::StrongJaw && move_.has_flag(Flag::Bite) {power *= 1.5};
    //TODO Supreme Overlord
    if attacker.ability == Ability::ToughClaws && move_.has_flag(Flag::Contact) {power *= 1.3};
    //TODO Toxic Boost

    // --- Ally Abilities ---
    //TODO Battery
    //TODO Power Spot
    //TODO Steely Spirit
    //TODO Friend Guard
    //TODO Flower Gift
    //TODO Minus

    // --- Get Stat Ability ---
    //TODO Chlorophyll
    //TODO Flower Gift
    //TODO Fur Coat
    //TODO Gorilla Tactics
    //TODO Grass Pelt
    //TODO Guts
    //TODO Hadron Engine
    //TODO Huge Power
    //TODO Hustle
    //TODO Marvel Scale
    //TODO Orichalcum Pulse
    //TODO Plus
    //TODO Protosynthesis
    //TODO Pure Power
    //TODO Quark Drive
    //TODO Quick Feet
    //TODO Sand Rush
    //TODO Slush Rush
    //TODO Solar Power
    //TODO Surge Surfer
    //TODO Swift Swim
    //TODO Unburden

    // --- Defender Abilities ---
    if !ignore_defender_abilities {
        let immune = match (defender.ability, current_move_type) {
            (Ability::EarthEater, Type::Ground) => true,
            (Ability::FlashFire, Type::Fire) => true,
            (Ability::DrySkin, Type::Water) => true,
            (Ability::Levitate, Type::Ground) => true,
            (Ability::LightningRod, Type::Electric) => true,
            (Ability::MotorDrive, Type::Electric) => true,
            (Ability::SapSipper, Type::Grass) => true,
            (Ability::StormDrain, Type::Water) => true,
            (Ability::VoltAbsorb, Type::Electric) => true,
            (Ability::WaterAbsorb, Type::Water) => true,
            (Ability::WellBakedBody, Type::Fire) => true,
            _ => false
        };
        if immune {
            return  ZERO_DAMAGE
        }

        //TODO Disguise
        if defender.ability == Ability::DrySkin && current_move_type == Type::Fire  {other_modifications *= 0.8};
        if defender.ability == Ability::Fluffy  && current_move_type == Type::Fire {other_modifications *= 0.5};
        if defender.ability == Ability::Fluffy && move_.has_flag(Flag::Contact) {other_modifications *= 2.0};
        if defender.ability == Ability::Heatproof && current_move_type == Type::Fire {attack *= 0.5};
        //TODO Multiscale
        //TODO Shadow Shield (NOT INGORABLE)
        if defender.ability == Ability::PunkRock && move_.has_flag(Flag::Contact) {other_modifications *= 2.0};
        if defender.ability == Ability::PurifyingSalt && current_move_type == Type::Ghost {attack *= 0.5};
        if defender.ability == Ability::ThickFat && (current_move_type == Type::Fire || current_move_type == Type::Ice) {attack *= 0.5};
        if defender.ability == Ability::WaterBubble && current_move_type == Type::Fire {other_modifications *= 2.0};
        if defender.ability == Ability::IceScales && move_.category == Category::Special {other_modifications *= 2.0};
    }
        
    let target_multiplier = match (doubles, multi_target) {
        (true, true) => 0.75,
        _ => 1.0
    };
    let stab_multiplier = match (attacker.species.types.contains(&current_move_type), attacker.ability) {
        (true, Ability::Adaptability) => 2.0,
        (true, _) => 1.5,
        _ => 1.0,
    };
    // --- Type Effectiveness ---
    //TODO Scrappy
    //TODO Filter
    //TODO Prism Armor (NOT IGNORABLE)
    //TODO Solid Rock
    //TODO Wonder GUard

    let mut type_multiplier = 1.0;
    for type_ in &defender.species.types {
        let type_ = dex.type_(type_).expect("Dex to handle all types");
        type_multiplier *= type_.damage_taken(current_move_type).to_multiplier();
    }

    let n_hits = move_.number_of_hits.max();

    let level = attacker.level as CalcFloat;
    let attack = pokemon_round(attack);
    let power = pokemon_round(power);
    let defence = pokemon_round(defence);
    let mut damage = (((level*2.0/5.0 + 2.0).floor()*power*attack/defence).floor()/50.0 + 2.0).floor(); // From showdown, diverges from bulbapedia

    damage = pokemon_round(damage * target_multiplier);
    // Parental Bond
    // Weather
    // Glaive Rush
    if move_.will_crit {damage = (damage * 1.5).floor()};
    let mut random = DamageRange::new();
    random.floored_multiply(damage/100.0); // From showdown, diverges from bulbapedia
    let mut damage = random;
    damage.floored_multiply(stab_multiplier); // From showdown, diverges from bulbapedia
    damage.floored_multiply(type_multiplier);
    // BURN
    damage.pokerounded_multiply(other_modifications);
    // ZMOVE
    // TERA SHIELD
    damage.pokerounded_multiply(n_hits as f32);
    damage
}


pub trait MaybeAPokemon {}
impl MaybeAPokemon for Pokemon<'_> {}
impl MaybeAPokemon for () {}

pub trait MaybeAMove {}
impl MaybeAMove for &Move {}
impl MaybeAMove for () {}

#[derive(Clone, Copy)]
pub struct CalcBuilder<'a, Attacker: MaybeAPokemon, Defender: MaybeAPokemon, Move: MaybeAMove>(&'a Dex, Attacker, Defender, Move);
type ReadyCalc<'a> = CalcBuilder<'a, Pokemon<'a>, Pokemon<'a>, &'a Move>;

impl Display for ReadyCalc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} -> {}", self.1.species.name, self.3.name, self.2.species.name)
    }
}

pub struct CalcOutcome<'a>(DamageRange, ReadyCalc<'a>);
impl CalcOutcome<'_> {
    pub fn damage_range(&self) -> DamageRange {
        self.0
    }
    pub fn calc_details(&self) -> &ReadyCalc {
        &self.1
    }
}

impl Dex {
    pub fn calc(&self) -> CalcBuilder<(), (), ()> {
        CalcBuilder(self, (), (), ())
    }
}
impl<'a, D: MaybeAPokemon, M: MaybeAMove> CalcBuilder<'a, (), D, M> {
    pub fn attacker(self, attacker: Pokemon<'a>) -> CalcBuilder<'a, Pokemon<'a>, D, M> {
        CalcBuilder(self.0, attacker, self.2, self.3)
    }
}
impl<'a, A: MaybeAPokemon, B: MaybeAMove> CalcBuilder<'a, A, (), B> {
    pub fn defender(self, defender: Pokemon<'a>) -> CalcBuilder<'a, A, Pokemon<'a>, B> {
        CalcBuilder(self.0, self.1, defender, self.3)
    }
}
impl<'a, A: MaybeAPokemon, D: MaybeAPokemon> CalcBuilder<'a, A, D, ()> {
    pub fn move_<Id: Identifier>(self, move_:Id) -> Result<CalcBuilder<'a, A, D, &'a Move>, DexError> {
        let move_ = self.0.move_(move_)?;
        Ok(CalcBuilder(self.0, self.1, self.2, move_))
    }
}
impl<'a> CalcBuilder<'a, Pokemon<'a>, Pokemon<'a>, ()> {
    pub fn all_possible_attacks(self) -> Result<Vec<CalcBuilder<'a, Pokemon<'a>, Pokemon<'a>, &'a Move>>, DexError> {
        let learnset = self.0.learnset(&self.1.species.name)?;
        Ok(learnset.all_moves().iter()
            .flat_map(|move_| self.move_(move_))
            .collect()
        )  
    }
}
impl<'a> ReadyCalc<'a>{
    pub fn calc(self, doubles: bool) -> CalcOutcome<'a> {
        let result = damage_calc(self.0, &self.1, &self.2, self.3, doubles);
        CalcOutcome(result, self)
    }
}


#[cfg(test)]
#[cfg(feature = "real_data")]
mod tests {
    use std::fmt::Debug;

    use crate::{dex::{Dex, DexError}, natures::Nature, species::Stat};

    use super::{CalcOutcome, DamageRange};

    impl CalcOutcome<'_> {
        pub fn assert<T>(&self, value: T) where DamageRange: PartialEq<T>, T: Debug {
            assert_eq!(self.0, value, "{}", self.1)
        }
    }

    #[test]
    fn damage_calc_tests() -> Result<(), DexError> {
        let dex = Dex::default();

        let punching_bag = dex.pokemon("flareon")?
            .ev(Stat::Defence, 99)
            .ev(Stat::SpecialDefence, 99)
            .nature(dex.nature(Nature::Bold)?);

        // EVS
        let rillaboom = dex.pokemon("rillaboom")?
            .ev(Stat::Attack, 184)
            .nature(dex.nature(Nature::Adamant)?);
        dex.calc().attacker(rillaboom).defender(punching_bag).move_("stompingtantrum")?.calc(true).assert([104, 106, 106, 108, 110, 110, 112, 114, 114, 116, 116, 118, 120, 120, 122, 124]);

        // Resisted attack
        let swampert = dex.pokemon("swampert")?;
        dex.calc().attacker(swampert).defender(swampert).move_("hydropump")?.calc(true).assert([60, 61, 61, 63, 63, 64, 64, 66, 66, 67, 67, 69, 69, 70, 70, 72]);
        
        // AutoCrit Moves
        let urshifu_rs = dex.pokemon("urshifurapidstrike")?;
        dex.calc().attacker(urshifu_rs).defender(punching_bag).move_("surgingstrikes")?.calc(true).assert([198, 204, 204, 204, 216, 216, 216, 216, 222, 222, 222, 222, 234, 234, 234, 240]);
        
        let swampert = dex.pokemon("swampert")?
            .ev(Stat::Attack, 252);
        dex.calc().attacker(swampert).defender(punching_bag).move_("wickedblow")?.calc(true).assert([68, 69, 70, 71, 72, 72, 73, 74, 75, 76, 76, 77, 78, 79, 80, 81]);
        
        Ok(())
    }
}