use crate::monster::Monster;
use crate::type_effectiveness::{TypeEffectivenessCalculator, TypeEffectivenessMultiplier};
use crate::{Attack, Damage};

#[derive(Default)]
pub struct CombatService<TEC: TypeEffectivenessCalculator> {
    type_effectiveness_calculator: TEC,
}

impl<TEC: TypeEffectivenessCalculator> CombatService<TEC> {
    pub fn new(type_effectiveness_calculator: TEC) -> Self {
        CombatService {
            type_effectiveness_calculator,
        }
    }

    pub fn perform_attack(
        &self,
        attack: Attack,
        defender: &mut Monster,
    ) -> Result<(), CombatError> {
        if self.is_defeated(defender) {
            return Err(CombatError::DefenderIsAlreadyDefeated);
        }

        dbg!(attack.element(), defender.elemental_type());
        let multiplier = self
            .type_effectiveness_calculator
            .calculate(attack.element(), defender.elemental_type());

        let damage = self.damage_from_attack(&attack, multiplier);
        defender.receive_damage(damage);
        Ok(())
    }

    fn is_defeated(&self, monster: &Monster) -> bool {
        monster.health().value() == 0.into()
    }

    fn damage_from_attack(
        &self,
        attack: &Attack,
        multiplier: TypeEffectivenessMultiplier,
    ) -> Damage {
        Damage::new(attack.power().value() * multiplier.value())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CombatError {
    #[error("Defender is already defeated")]
    DefenderIsAlreadyDefeated,
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use rust_decimal::Decimal;
    use spectral::prelude::*;

    use crate::type_effectiveness::{MockTypeEffectivenessCalculator, TypeEffectivenessMultiplier};
    use crate::{AttackPower, ElementalType, Health, PrimitiveElement};

    use super::*;

    fn under_test(
        type_effectiveness_calculator: MockTypeEffectivenessCalculator,
    ) -> CombatService<MockTypeEffectivenessCalculator> {
        CombatService::new(type_effectiveness_calculator)
    }

    fn mock_type_effectiveness_calculator() -> MockTypeEffectivenessCalculator {
        MockTypeEffectivenessCalculator::default()
    }

    fn prepare_mock_type_effectiveness_calculator(
        mock_type_effectiveness_calculator: &mut MockTypeEffectivenessCalculator,
        multiplier_value: Decimal,
    ) {
        mock_type_effectiveness_calculator
            .expect_calculate()
            .with(
                eq(PrimitiveElement::Normal),
                eq(ElementalType::new(PrimitiveElement::Normal, None)),
            )
            .returning(move |_, _| TypeEffectivenessMultiplier::new(multiplier_value));
    }

    fn elemental_type() -> ElementalType {
        ElementalType::new(PrimitiveElement::Normal, None)
    }

    fn defending_monster(health_value: Decimal) -> Monster {
        Monster::new(elemental_type(), Health::new(health_value))
    }

    fn attack() -> Attack {
        Attack::new(PrimitiveElement::Normal, AttackPower::new(5.into()))
    }

    #[test]
    fn performs_attack_on_defender() {
        let mut defender = defending_monster(10.into());
        let mut calculator = mock_type_effectiveness_calculator();
        prepare_mock_type_effectiveness_calculator(&mut calculator, 1.into());

        under_test(calculator)
            .perform_attack(attack(), &mut defender)
            .unwrap();

        assert_that(defender.health()).is_equal_to(&Health::new(5.into()));
    }

    #[test]
    fn applies_type_effectiveness_multiplier_to_resultant_damage() {
        let mut defender = defending_monster(20.into());
        let mut calculator = mock_type_effectiveness_calculator();
        prepare_mock_type_effectiveness_calculator(&mut calculator, 2.into());

        under_test(calculator)
            .perform_attack(attack(), &mut defender)
            .unwrap();

        assert_that(defender.health()).is_equal_to(&Health::new(10.into()));
    }

    #[test]
    fn fails_to_perform_attack_on_already_defeated_defender() {
        assert_that(&matches!(
            under_test(mock_type_effectiveness_calculator())
                .perform_attack(attack(), &mut defending_monster(0.into())),
            Err(CombatError::DefenderIsAlreadyDefeated)
        ))
        .is_true();
    }
}
