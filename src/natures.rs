use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;

use crate::{dex::Identifier, species::Stat};


#[derive(Serialize)]
pub enum Nature {
    Hardy,
    Lonely,
    Brave,
    Adamant,
    Naughty,
    Bold,
    Docile,
    Relaxed,
    Impish,
    Lax,
    Timid,
    Hasty,
    Serious,
    Jolly,
    Naive,
    Modest,
    Mild,
    Quiet,
    Bashful,
    Rash,
    Calm,
    Gentle,
    Sassy,
    Careful,
    Quirky,    
}
impl Display for Nature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&to_variant_name(self).unwrap().to_lowercase(), f)
    }
}
impl Identifier for Nature {
    fn as_identifier(&self) -> String {
        self.to_string()
    }
}

#[derive(Deserialize, Clone)]
pub struct NatureData {
    pub name: String,
    pub plus: Option<Stat>,
    pub minus: Option<Stat>,
}
impl NatureData {
    pub fn multiplier(&self, stat: Stat) -> f32 {
        if self.plus.is_some_and(|plus| plus == stat) {
            1.1
        } else if self.minus.is_some_and(|plus| plus == stat) {
            0.9
        } else {
            1.0
        }
    }
}