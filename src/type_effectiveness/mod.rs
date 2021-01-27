pub use lame_geek::*;

use crate::damage::DamageMultiplier;
use crate::{ElementalType, PrimitiveElement};

mod lame_geek;

#[cfg_attr(test, mockall::automock)]
pub trait TypeEffectivenessCalculator {
    fn calculate(
        &self,
        attack_type: &PrimitiveElement,
        defender_type: &ElementalType,
    ) -> DamageMultiplier;
}
