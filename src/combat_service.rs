use rust_decimal::Decimal;

use crate::monster::Monster;
use crate::type_effectiveness::TypeEffectivenessCalculator;
use crate::{Attack, Damage, DamageMultiplier};

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

    fn stab_multiplier(attacker: &Monster, attack: &Attack) -> DamageMultiplier {
        if attacker.elemental_type().primary_primitive_type() == attack.element() {
            return DamageMultiplier::new(Decimal::new(15, 1));
        }
        DamageMultiplier::new(1.into())
    }

    pub fn perform_attack(
        &self,
        attacker: &Monster,
        attack: Attack,
        defender: &mut Monster,
    ) -> Result<(), CombatError> {
        if self.is_defeated(defender) {
            return Err(CombatError::DefenderIsAlreadyDefeated);
        }

        let stab_multiplier = Self::stab_multiplier(attacker, &attack);
        let type_effectiveness_multiplier = self
            .type_effectiveness_calculator
            .calculate(attack.element(), defender.elemental_type());
        let combined_damage_multiplier =
            stab_multiplier.combined_with(type_effectiveness_multiplier);

        let damage = self.damage_from_attack(&attack, combined_damage_multiplier);
        defender.receive_damage(damage);
        Ok(())
    }

    fn is_defeated(&self, monster: &Monster) -> bool {
        monster.health().value() == 0.into()
    }

    fn damage_from_attack(&self, attack: &Attack, multiplier: DamageMultiplier) -> Damage {
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

    use crate::type_effectiveness::MockTypeEffectivenessCalculator;
    use crate::{AttackPower, DamageMultiplier, ElementalType, Health, PrimitiveElement};

    use super::*;

    const NON_STAB_ELEMENT: PrimitiveElement = PrimitiveElement::Water;

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
            .returning(move |_, _| DamageMultiplier::new(multiplier_value));
    }

    fn elemental_type() -> ElementalType {
        ElementalType::new(PrimitiveElement::Normal, None)
    }

    fn attacking_monster(primary_element: PrimitiveElement) -> Monster {
        Monster::new(
            ElementalType::new(primary_element, None),
            Health::new(10.into()),
        )
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
            .perform_attack(
                &attacking_monster(NON_STAB_ELEMENT),
                attack(),
                &mut defender,
            )
            .unwrap();

        assert_that(defender.health()).is_equal_to(&Health::new(5.into()));
    }

    #[test]
    fn applies_type_effectiveness_multiplier_to_resultant_damage() {
        let mut defender = defending_monster(20.into());
        let mut calculator = mock_type_effectiveness_calculator();
        prepare_mock_type_effectiveness_calculator(&mut calculator, 2.into());

        under_test(calculator)
            .perform_attack(
                &attacking_monster(NON_STAB_ELEMENT),
                attack(),
                &mut defender,
            )
            .unwrap();

        assert_that(defender.health()).is_equal_to(&Health::new(10.into()));
    }

    #[test]
    fn applies_stab_multiplier_bonus_when_attack_type_matches_attacker_primary_type() {
        let mut defender = defending_monster(10.into());
        let mut calculator = mock_type_effectiveness_calculator();
        prepare_mock_type_effectiveness_calculator(&mut calculator, 1.into());

        under_test(calculator)
            .perform_attack(
                &attacking_monster(PrimitiveElement::Normal),
                attack(),
                &mut defender,
            )
            .unwrap();

        assert_that(defender.health()).is_equal_to(&Health::new(Decimal::new(25, 1)));
    }

    #[test]
    fn fails_to_perform_attack_on_already_defeated_defender() {
        assert_that(&matches!(
            under_test(mock_type_effectiveness_calculator()).perform_attack(
                &attacking_monster(NON_STAB_ELEMENT),
                attack(),
                &mut defending_monster(0.into())
            ),
            Err(CombatError::DefenderIsAlreadyDefeated)
        ))
        .is_true();
    }
}
