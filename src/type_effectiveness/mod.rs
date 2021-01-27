use rust_decimal::Decimal;

pub use lame_geek::*;

use crate::{ElementalType, PrimitiveElement};

mod lame_geek;

#[cfg_attr(test, mockall::automock)]
pub trait TypeEffectivenessCalculator {
    fn calculate(
        &self,
        attack_type: &PrimitiveElement,
        defender_type: &ElementalType,
    ) -> TypeEffectivenessMultiplier;
}

#[derive(Debug, Eq, PartialEq)]
pub struct TypeEffectivenessMultiplier {
    value: Decimal,
}

impl TypeEffectivenessMultiplier {
    pub fn new(value: Decimal) -> Self {
        TypeEffectivenessMultiplier { value }
    }

    pub fn value(&self) -> Decimal {
        self.value
    }
}
