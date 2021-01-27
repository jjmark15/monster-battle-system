use rust_decimal::Decimal;

use crate::type_effectiveness::{DamageMultiplier, TypeEffectivenessCalculator};
use crate::{ElementalType, PrimitiveElement};

#[derive(Default)]
pub struct TypeEffectivenessCalculatorImpl;

impl TypeEffectivenessCalculatorImpl {
    pub fn new() -> Self {
        TypeEffectivenessCalculatorImpl
    }

    fn primitive_multiplier(
        attack_type: &PrimitiveElement,
        defender_type: &PrimitiveElement,
    ) -> DamageMultiplier {
        let half = Decimal::new(5, 1);
        let single = 1.into();
        let double = 2.into();

        let value: Decimal = match attack_type {
            PrimitiveElement::Normal => match defender_type {
                PrimitiveElement::Normal
                | PrimitiveElement::Fire
                | PrimitiveElement::Grass
                | PrimitiveElement::Water => single,
            },
            PrimitiveElement::Fire => match defender_type {
                PrimitiveElement::Water | PrimitiveElement::Fire => half,
                PrimitiveElement::Normal => single,
                PrimitiveElement::Grass => double,
            },
            PrimitiveElement::Grass => match defender_type {
                PrimitiveElement::Fire | PrimitiveElement::Grass => half,
                PrimitiveElement::Normal => single,
                PrimitiveElement::Water => double,
            },
            PrimitiveElement::Water => match defender_type {
                PrimitiveElement::Grass | PrimitiveElement::Water => half,
                PrimitiveElement::Normal => single,
                PrimitiveElement::Fire => double,
            },
        };

        DamageMultiplier::new(value)
    }
}

impl TypeEffectivenessCalculator for TypeEffectivenessCalculatorImpl {
    fn calculate(
        &self,
        attack_type: &PrimitiveElement,
        defender_type: &ElementalType,
    ) -> DamageMultiplier {
        let first_multiplier =
            Self::primitive_multiplier(attack_type, defender_type.primary_primitive_type());

        match defender_type.secondary_primitive_type() {
            Some(element) => {
                let second_multiplier = Self::primitive_multiplier(attack_type, element);
                first_multiplier.combined_with(second_multiplier)
            }
            None => first_multiplier,
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    fn under_test() -> TypeEffectivenessCalculatorImpl {
        TypeEffectivenessCalculatorImpl::new()
    }

    #[test]
    fn normal_has_multiplier_1_against_normal() {
        assert_that(&under_test().calculate(
            &PrimitiveElement::Normal,
            &ElementalType::new(PrimitiveElement::Normal, None),
        ))
        .is_equal_to(&DamageMultiplier::new(1.into()));
    }

    #[test]
    fn fire_has_multiplier_0_5_against_water() {
        assert_that(&under_test().calculate(
            &PrimitiveElement::Fire,
            &ElementalType::new(PrimitiveElement::Water, None),
        ))
        .is_equal_to(&DamageMultiplier::new(Decimal::new(5, 1)));
    }

    #[test]
    fn grass_has_multiplier_2_against_water() {
        assert_that(&under_test().calculate(
            &PrimitiveElement::Grass,
            &ElementalType::new(PrimitiveElement::Water, None),
        ))
        .is_equal_to(&DamageMultiplier::new(2.into()));
    }

    #[test]
    fn fire_has_multiplier_0_25_against_water_and_fire() {
        assert_that(&under_test().calculate(
            &PrimitiveElement::Fire,
            &ElementalType::new(PrimitiveElement::Water, Some(PrimitiveElement::Fire)),
        ))
        .is_equal_to(&DamageMultiplier::new(Decimal::new(25, 2)));
    }
}
