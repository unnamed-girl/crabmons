use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub const LATEST_GENERATION: Generation = Generation::Nine;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[serde(try_from = "u8")] 
pub enum Generation {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

#[derive(Debug)]
pub struct NotAGeneration(String);
impl Display for NotAGeneration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a generation", self.0)
    }
}
impl TryFrom<u8> for Generation {
    type Error = NotAGeneration;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Generation::One),
            2 => Ok(Generation::Two),
            3 => Ok(Generation::Three),
            4 => Ok(Generation::Four),
            5 => Ok(Generation::Five),
            6 => Ok(Generation::Six),
            7 => Ok(Generation::Seven),
            8 => Ok(Generation::Eight),
            9 => Ok(Generation::Nine),
            _ => Err(NotAGeneration(value.to_string()))
        }
    }
}
impl TryFrom<&str> for Generation {
    type Error = NotAGeneration;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|_| NotAGeneration(value.to_string()))
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