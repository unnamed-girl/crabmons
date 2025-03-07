pub mod species;
pub mod types;
pub mod moves;
pub mod dex;
pub mod learnsets;
pub mod generation;
pub mod natures;
pub mod pokemon;
pub mod damage_calc;
pub mod items;

pub mod names;

pub(crate) mod parsing_utils;

#[cfg(feature = "real_data")]
pub mod real_data;