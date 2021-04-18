use primitive_multipliers::*;

use crate::combat::DamageMultiplier;
use crate::monster::MonsterType;
use crate::Element;

mod primitive_multipliers;

#[cfg_attr(test, mockall::automock)]
pub trait TypeEffectivenessCalculator {
    fn calculate(&self, attack_type: &Element, defender_type: &MonsterType) -> DamageMultiplier;
}

#[derive(Default)]
pub struct TypeEffectivenessCalculatorImpl;

impl TypeEffectivenessCalculatorImpl {
    pub fn new() -> Self {
        TypeEffectivenessCalculatorImpl
    }

    fn primitive_multiplier(attack_type: &Element, defender_type: &Element) -> DamageMultiplier {
        match attack_type {
            Element::Bug => bug_damage_multiplier(defender_type),
            Element::Dark => dark_damage_multiplier(defender_type),
            Element::Dragon => dragon_damage_multiplier(defender_type),
            Element::Electric => electric_damage_multiplier(defender_type),
            Element::Fairy => fairy_damage_multiplier(defender_type),
            Element::Fighting => fighting_damage_multiplier(defender_type),
            Element::Fire => fire_damage_multiplier(defender_type),
            Element::Flying => flying_damage_multiplier(defender_type),
            Element::Ghost => ghost_damage_multiplier(defender_type),
            Element::Grass => grass_damage_multiplier(defender_type),
            Element::Ground => ground_damage_multiplier(defender_type),
            Element::Ice => ice_damage_multiplier(defender_type),
            Element::Normal => normal_damage_multiplier(defender_type),
            Element::Poison => poison_damage_multiplier(defender_type),
            Element::Psychic => psychic_damage_multiplier(defender_type),
            Element::Rock => rock_damage_multiplier(defender_type),
            Element::Steel => steel_damage_multiplier(defender_type),
            Element::Water => water_damage_multiplier(defender_type),
        }
        .into()
    }
}

impl TypeEffectivenessCalculator for TypeEffectivenessCalculatorImpl {
    fn calculate(&self, attack_type: &Element, defender_type: &MonsterType) -> DamageMultiplier {
        let first_multiplier =
            Self::primitive_multiplier(attack_type, defender_type.primary_element());

        match defender_type.secondary_element() {
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
    use rust_decimal::Decimal;
    use spectral::prelude::*;

    use super::*;

    fn under_test() -> TypeEffectivenessCalculatorImpl {
        TypeEffectivenessCalculatorImpl::new()
    }

    #[test]
    fn normal_has_multiplier_1_against_normal() {
        assert_that(
            &under_test().calculate(&Element::Normal, &MonsterType::new(Element::Normal, None)),
        )
        .is_equal_to(&DamageMultiplier::new(1.into()));
    }

    #[test]
    fn fire_has_multiplier_0_5_against_water() {
        assert_that(
            &under_test().calculate(&Element::Fire, &MonsterType::new(Element::Water, None)),
        )
        .is_equal_to(&DamageMultiplier::new(Decimal::new(5, 1)));
    }

    #[test]
    fn grass_has_multiplier_2_against_water() {
        assert_that(
            &under_test().calculate(&Element::Grass, &MonsterType::new(Element::Water, None)),
        )
        .is_equal_to(&DamageMultiplier::new(2.into()));
    }

    #[test]
    fn fire_has_multiplier_0_25_against_water_and_fire() {
        assert_that(&under_test().calculate(
            &Element::Fire,
            &MonsterType::new(Element::Water, Some(Element::Fire)),
        ))
        .is_equal_to(&DamageMultiplier::new(Decimal::new(25, 2)));
    }
}
