use rust_decimal::Decimal;

use crate::combat::{DamageMultiplier, TypeEffectivenessCalculator};
use crate::monster::{Attack, Damage, Monster};

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
        if attacker.monster_type().primary_element() == attack.element() {
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
            .calculate(attack.element(), defender.monster_type());
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

    use crate::combat::MockTypeEffectivenessCalculator;
    use crate::monster::{AttackPower, Health, MonsterType};
    use crate::Element;

    use super::*;

    const NON_STAB_ELEMENT: Element = Element::Water;

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
                eq(Element::Normal),
                eq(MonsterType::new(Element::Normal, None)),
            )
            .returning(move |_, _| DamageMultiplier::new(multiplier_value));
    }

    fn monster_type() -> MonsterType {
        MonsterType::new(Element::Normal, None)
    }

    fn attacking_monster(primary_element: Element) -> Monster {
        Monster::new(
            MonsterType::new(primary_element, None),
            Health::new(10.into()),
        )
    }

    fn defending_monster(health_value: Decimal) -> Monster {
        Monster::new(monster_type(), Health::new(health_value))
    }

    fn attack() -> Attack {
        Attack::new(Element::Normal, AttackPower::new(5.into()))
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
    fn applies_stab_multiplier_bonus_when_attack_type_matches_attacker_primary_element() {
        let mut defender = defending_monster(10.into());
        let mut calculator = mock_type_effectiveness_calculator();
        prepare_mock_type_effectiveness_calculator(&mut calculator, 1.into());

        under_test(calculator)
            .perform_attack(&attacking_monster(Element::Normal), attack(), &mut defender)
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
