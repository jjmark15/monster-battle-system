use rust_decimal::Decimal;

use crate::combat::{DamageMultiplier, TypeEffectivenessCalculator};
use crate::monster::MonsterElements;
use crate::Element;

#[derive(Default)]
pub struct TypeEffectivenessCalculatorImpl;

impl TypeEffectivenessCalculatorImpl {
    pub fn new() -> Self {
        TypeEffectivenessCalculatorImpl
    }

    fn primitive_multiplier(attack_type: &Element, defender_type: &Element) -> DamageMultiplier {
        let half = Decimal::new(5, 1);
        let single = 1.into();
        let double = 2.into();

        let value: Decimal = match attack_type {
            Element::Normal => match defender_type {
                Element::Normal | Element::Fire | Element::Grass | Element::Water => single,
            },
            Element::Fire => match defender_type {
                Element::Water | Element::Fire => half,
                Element::Normal => single,
                Element::Grass => double,
            },
            Element::Grass => match defender_type {
                Element::Fire | Element::Grass => half,
                Element::Normal => single,
                Element::Water => double,
            },
            Element::Water => match defender_type {
                Element::Grass | Element::Water => half,
                Element::Normal => single,
                Element::Fire => double,
            },
        };

        DamageMultiplier::new(value)
    }
}

impl TypeEffectivenessCalculator for TypeEffectivenessCalculatorImpl {
    fn calculate(
        &self,
        attack_type: &Element,
        defender_type: &MonsterElements,
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
            &Element::Normal,
            &MonsterElements::new(Element::Normal, None),
        ))
        .is_equal_to(&DamageMultiplier::new(1.into()));
    }

    #[test]
    fn fire_has_multiplier_0_5_against_water() {
        assert_that(
            &under_test().calculate(&Element::Fire, &MonsterElements::new(Element::Water, None)),
        )
        .is_equal_to(&DamageMultiplier::new(Decimal::new(5, 1)));
    }

    #[test]
    fn grass_has_multiplier_2_against_water() {
        assert_that(
            &under_test().calculate(&Element::Grass, &MonsterElements::new(Element::Water, None)),
        )
        .is_equal_to(&DamageMultiplier::new(2.into()));
    }

    #[test]
    fn fire_has_multiplier_0_25_against_water_and_fire() {
        assert_that(&under_test().calculate(
            &Element::Fire,
            &MonsterElements::new(Element::Water, Some(Element::Fire)),
        ))
        .is_equal_to(&DamageMultiplier::new(Decimal::new(25, 2)));
    }
}
