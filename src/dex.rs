use std::collections::HashMap;

use crate::{learnsets::Learnset, moves::Move, natures::NatureData, pokemon::Pokemon, species::Species, types::TypeData};

pub trait Identifier {
    fn as_identifier(&self) -> String;
}
impl<T: AsRef<str>> Identifier for T {
    fn as_identifier(&self) -> String {
        self.as_ref().to_lowercase().replace("-", "").replace(" ", "")
    }
}

pub struct Dex {
    moves: HashMap<String, Move>,
    species: HashMap<String, Species>,
    types: HashMap<String, TypeData>,
    learnsets: HashMap<String, Learnset>,
    natures: HashMap<String, NatureData>
}

#[derive(Debug)]
pub enum DexError {
    NotFound(String)
}
impl Dex {
    pub fn new(moves: HashMap<String, Move>, species: HashMap<String, Species>, types: HashMap<String, TypeData>, learnsets: HashMap<String, Learnset>, natures: HashMap<String, NatureData>) -> Self {
        Self { moves, species, types, learnsets, natures }
    }
    
    pub fn move_<Id: Identifier>(&self, identifier:Id) -> Result<&Move, DexError> {
        self.moves.get(&identifier.as_identifier()).ok_or(DexError::NotFound(identifier.as_identifier()))
    }
    pub fn species<Id: Identifier>(&self, identifier:Id) -> Result<&Species, DexError> {
        self.species.get(&identifier.as_identifier()).ok_or(DexError::NotFound(identifier.as_identifier()))
    }
    pub fn type_<Id: Identifier>(&self, identifier:Id) -> Result<&TypeData, DexError> {
        self.types.get(&identifier.as_identifier()).ok_or(DexError::NotFound(identifier.as_identifier()))
    }
    pub fn learnset<Id: Identifier>(&self, identifier:Id) -> Result<&Learnset, DexError> {
        self.learnsets.get(&identifier.as_identifier()).ok_or(DexError::NotFound(identifier.as_identifier()))
    }
    pub fn nature<Id: Identifier>(&self, identifier:Id) -> Result<&NatureData, DexError> {
        self.natures.get(&identifier.as_identifier()).ok_or(DexError::NotFound(identifier.as_identifier()))
    }
    pub fn pokemon<Id: Identifier>(&self, identifier:Id) -> Result<Pokemon, DexError> {
        Ok(Pokemon::new(self.species(identifier)?))
    }
}
