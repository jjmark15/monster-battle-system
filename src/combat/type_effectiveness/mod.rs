pub use lame_geek::*;

use crate::combat::DamageMultiplier;
use crate::monster::MonsterType;
use crate::Element;

mod lame_geek;

#[cfg_attr(test, mockall::automock)]
pub trait TypeEffectivenessCalculator {
    fn calculate(&self, attack_type: &Element, defender_type: &MonsterType) -> DamageMultiplier;
}
