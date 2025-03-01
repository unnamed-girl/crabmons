use crate::{natures::NatureData, species::{Ability, Species, Stat, StatDistribution}};

#[derive(Clone, Copy)]
pub struct Pokemon<'a> {
    pub species: &'a Species,
    pub level: u8,
    pub ability: Ability,
    pub nature: Option<&'a NatureData>,
    pub evs: StatDistribution,
    pub ivs: StatDistribution
}
impl<'a> Pokemon<'a> {
    pub fn new(species: &'a Species) -> Self {
        Self { species, level: 50, ability: Ability::NoAbility, nature:None, evs: StatDistribution::default(), ivs:[31, 31, 31, 31, 31, 31].into() }
    }
    pub fn stat(&self, stat: Stat) -> u16 {
        let base_stat = self.species.base_stats.get(stat) as f32;
        let nature_multiplier = self.nature.as_ref().map_or(1.0, |nature| nature.multiplier(stat));
        let ev = self.evs.get(stat) as f32;
        let iv = self.ivs.get(stat) as f32;
        let level = self.level as f32;
        match stat {
            Stat::HP=> ((2.0*base_stat + iv + ev/4.0)*level/100.0 + level + 10.0) as u16,
            _ => (((2.0*base_stat + iv + ev/4.0)*level/100.0 + 5.0).floor() * nature_multiplier).floor() as u16,
        }
    }
}

impl<'a> Pokemon<'a> {
    pub fn ivs(mut self, ivs:impl Into<StatDistribution>) -> Self {
        self.ivs = ivs.into();
        self
    }
    pub fn evs(mut self, evs:impl Into<StatDistribution>) -> Self {
        self.evs = evs.into();
        self
    }
    pub fn iv(mut self, stat:Stat, iv:u8) -> Self {
        *self.ivs.get_mut(stat) = iv;
        self
    }
    pub fn ev(mut self, stat:Stat, ev:u8) -> Self {
        *self.evs.get_mut(stat) = ev;
        self
    }
    pub fn level(mut self, level:u8) -> Self {
        self.level = level;
        self
    }
    pub fn nature(mut self, nature: &'a NatureData) -> Self {
        self.nature = Some(nature);
        self
    }
}