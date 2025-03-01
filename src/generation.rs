use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub const LATEST_GENERATION: Generation = Generation::Nine;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Generation {
    #[serde(alias = "1")]
    One,
    #[serde(alias = "2")]
    Two,
    #[serde(alias = "3")]
    Three,
    #[serde(alias = "4")]
    Four,
    #[serde(alias = "5")]
    Five,
    #[serde(alias = "6")]
    Six,
    #[serde(alias = "7")]
    Seven,
    #[serde(alias = "8")]
    Eight,
    #[serde(alias = "9")]
    Nine
}
pub struct NotAGeneration;
impl Debug for NotAGeneration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Not a generation".fmt(f)
    }
}
impl TryFrom<u8> for Generation {
    type Error = NotAGeneration;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        serde_json::from_str(&format!("\"{}\"", value)).map_err(|_| NotAGeneration)
    }
}
impl TryFrom<&str> for Generation {
    type Error = NotAGeneration;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|_| NotAGeneration)
    }
}
impl Generation {
    pub fn next_generation(self) -> Option<Self> {
        match self {
            Generation::One => Some(Generation::Two),
            Generation::Two => Some(Generation::Three),
            Generation::Three => Some(Generation::Four),
            Generation::Four => Some(Generation::Five),
            Generation::Five => Some(Generation::Six),
            Generation::Six => Some(Generation::Seven),
            Generation::Seven => Some(Generation::Eight),
            Generation::Eight => Some(Generation::Nine),
            Generation::Nine => None,
        }
    }
}