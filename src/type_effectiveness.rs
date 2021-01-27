use crate::{ElementalType, PrimitiveElement};

#[cfg_attr(test, mockall::automock)]
pub trait TypeEffectivenessCalculator {
    fn calculate(
        &self,
        attack_type: &PrimitiveElement,
        defender_type: &ElementalType,
    ) -> TypeEffectivenessMultiplier;
}

#[derive(Debug)]
pub struct TypeEffectivenessMultiplier {
    value: u16,
}

impl TypeEffectivenessMultiplier {
    #[cfg(test)]
    pub fn new(value: u16) -> Self {
        TypeEffectivenessMultiplier { value }
    }

    pub fn value(&self) -> u16 {
        self.value
    }
}
