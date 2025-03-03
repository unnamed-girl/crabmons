use pokemon::dex::{Dex, DexError};
use pokemon::species::Stat;

fn main() -> Result<(), DexError> {
    let dex = Dex::default();

    let defender = dex.pokemon("rillaboom")?;
    let attacker = dex.pokemon("tornadus")?.ev(Stat::Attack, 252);
    let attack = dex.calc().attacker(attacker).defender(defender).move_("hurricane")?;
    let mut calc = attack.calc(true).damage_range();
    println!("{:?}", calc);
    calc.ceiled_multiply(100.0/(defender.stat(Stat::HP) as f32));
    println!("{:?}", calc);

    println!("{:?}", dex.move_("clangingscales")?.self_boost);
    println!("{:?}", dex.item("kingsrock")?.fling);
    Ok(())
}