pub use lame_geek::*;

use crate::damage::DamageMultiplier;
use crate::{Element, MonsterElement};

mod lame_geek;

#[cfg_attr(test, mockall::automock)]
pub trait TypeEffectivenessCalculator {
    fn calculate(&self, attack_type: &Element, defender_type: &MonsterElement) -> DamageMultiplier;
}
