use dex::{DexError, GenDex};
use pokemon::PokemonBuilder;
use species::Stat;

pub mod species;
pub mod types;
pub mod moves;
pub mod dex;
pub mod learnsets;
pub mod generation;
pub mod natures;
pub mod pokemon;
pub mod damage_calc;

pub(crate) const SPECIES_JSON: &str = include_str!("../data/species.json");
pub(crate) const MOVES_JSON: &str = include_str!("../data/moves.json");
pub(crate) const LEARNSETS_JSON: &str = include_str!("../data/learnsets.json");
pub(crate) const TYPES_JSON: &str = include_str!("../data/types.json");
pub(crate) const NATURES_JSON: &str = include_str!("../data/natures.json");

fn main() {
    example_calc().unwrap();        
}

fn example_calc() -> Result<(), DexError> {
    let dex = GenDex::default();

    let defender = PokemonBuilder::new(&dex, "rillaboom")?;
    let attacker = PokemonBuilder::new(&dex, "tornadus")?.ev(Stat::Attack, 252);
    let attack = dex.calc().attacker(&attacker).defender(&defender).move_("hurricane")?;
    println!("{:?}", attack.calc(true).damage_range());
    Ok(())
}

