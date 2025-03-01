use std::{collections::HashMap, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{Map, Value};

use crate::{dex::Dex, generation::{Generation, LATEST_GENERATION}, learnsets::Learnset, moves::{Move, NonStandardReason}, natures::NatureData, species::Species, types::TypeData};

const SPECIES_JSON: &str = include_str!("../data/species.json");
const MOVES_JSON: &str = include_str!("../data/moves.json");
const LEARNSETS_JSON: &str = include_str!("../data/learnsets.json");
const TYPES_JSON: &str = include_str!("../data/types.json");
const NATURES_JSON: &str = include_str!("../data/natures.json");

trait Dexable {
    fn set_future(&mut self, gen: Generation); // Usually just check if this is a Future move.
    fn get_json() -> &'static str;
}

impl Default for Dex {
    fn default() -> Self {
        Self::generation_dex(LATEST_GENERATION)
    }
}
impl Dex {
    pub fn generation_dex(gen: Generation) -> Self {
        Self::new(gen_data(gen), gen_data(gen), gen_data(gen), gen_data(gen), gen_data(gen))
    }
}

fn gen_data<T: Dexable + DeserializeOwned>(gen: Generation) -> HashMap<String, T> {
    let raw_data = RawData::<T>::default();
    raw_data.0.get(&LATEST_GENERATION).unwrap().iter().map(|(name, _)|
        (name.clone(), raw_data.get(gen, name).unwrap())
    ).collect()
}

#[derive(Deserialize)]
#[serde(transparent)] 
struct RawData<T>(HashMap<Generation, HashMap<String, Map<String, Value>>>, PhantomData<T>);
impl<T:Dexable> Default for RawData<T> {
    fn default() -> Self {
        serde_json::from_str(T::get_json()).unwrap()
    }
}
impl<T:Dexable + DeserializeOwned> RawData<T> {
    fn get(&self, gen:Generation, identifier: &str) -> Option<T> {
        let data = self.get_recur(gen, identifier);
        let mut result: T = serde_json::from_value(Value::Object(data)).unwrap();
        result.set_future(gen);
        Some(result)
    }
    fn get_recur(&self, gen:Generation, identifier: &str) -> Map<String, Value> {
        let mut result = if let Some(parent_gen) = gen.next_generation() {
            self.get_recur(parent_gen, identifier)
        } else {
            Map::new()
        };
        let mut next = self.0.get(&gen).and_then(|map| map.get(identifier).cloned()).unwrap_or_default();
        result.append(&mut next);
        result
    }
}

impl Dexable for TypeData {
    fn set_future(&mut self, _gen: Generation) {}
    fn get_json() -> &'static str {
        TYPES_JSON
    }
}

impl Dexable for Move {
    fn get_json() -> &'static str {
        MOVES_JSON
    }
    fn set_future(&mut self, gen: Generation) {
        if self.num <= 0 {
            return
        }

        let first_generation = if self.is_max.is_some() {
            Generation::Eight
        } else {
            match self.num {
                827.. => Generation::Nine,
                743.. => Generation::Eight,
                622.. => Generation::Seven,
                560.. => Generation::Six,
                468.. => Generation::Five,
                355.. => Generation::Four,
                252.. => Generation::Three,
                166.. => Generation::Two,
                1.. => Generation::One,
                _ => panic!("Unreachable")
            }
        };
        if first_generation > gen {
            self.is_non_standard = Some(NonStandardReason::Future)
        }
    }
}

impl Dexable for Species {
    fn get_json() -> &'static str {
        SPECIES_JSON
    }
    fn set_future(&mut self, gen: Generation) {
        if self.num <= 0 {
            return
        }
        let mut generation = None;
        if let Some(forme) = &self.forme {
            if forme.contains("Paldea") {
                generation = Some(Generation::Nine)
            } else if let "Gmax" | "Galar" | "Galar-Zen" | "Hisui" = forme.as_str() {
                generation = Some(Generation::Eight)
            } else if forme.contains("Alola") || forme == "Starter" {
                generation = Some(Generation::Seven)
            }
        };
        let generation = generation.unwrap_or_else(|| match self.num {
                906.. => Generation::Nine,
                810.. => Generation::Eight,
                722.. => Generation::Seven,
                650.. => Generation::Six,
                494.. => Generation::Five,
                387.. => Generation::Four,
                252.. => Generation::Three,
                152.. => Generation::Two,
                1.. => Generation::One,
                _ => panic!("Unreachable")
        });
        if generation > gen {
            self.is_non_standard = Some(NonStandardReason::Future)
        }
    }
}

impl Dexable for Learnset {
    fn set_future(&mut self, _gen: Generation) {}
    fn get_json() -> &'static str {
        LEARNSETS_JSON
    }
}

impl Dexable for NatureData {
    fn set_future(&mut self, _gen: Generation) {}
    fn get_json() -> &'static str {
        NATURES_JSON
    }
}