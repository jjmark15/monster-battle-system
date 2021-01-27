use crate::monster::Monster;
use crate::{Attack, Damage};

#[derive(Default)]
pub struct CombatService;

impl CombatService {
    pub fn new() -> Self {
        CombatService
    }

    pub fn perform_attack(
        &self,
        attack: Attack,
        defender: &mut Monster,
    ) -> Result<(), CombatError> {
        if self.is_defeated(defender) {
            return Err(CombatError::DefenderIsAlreadyDefeated);
        }

        let damage = self.damage_from_attack(&attack);
        defender.receive_damage(damage);
        Ok(())
    }

    fn is_defeated(&self, monster: &Monster) -> bool {
        monster.health().value() == 0
    }

    fn damage_from_attack(&self, attack: &Attack) -> Damage {
        Damage::new(attack.power().value())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CombatError {
    #[error("Defender is already defeated")]
    DefenderIsAlreadyDefeated,
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::{AttackPower, ElementalType, Health, PrimitiveElement};

    use super::*;

    fn under_test() -> CombatService {
        CombatService::new()
    }

    fn defending_monster(health_value: u16) -> Monster {
        Monster::new(
            ElementalType::new(PrimitiveElement::Normal, None),
            Health::new(health_value),
        )
    }

    fn attack() -> Attack {
        Attack::new(PrimitiveElement::Normal, AttackPower::new(5))
    }

    #[test]
    fn performs_attack_on_defender() {
        let mut defender = defending_monster(10);

        under_test()
            .perform_attack(attack(), &mut defender)
            .unwrap();

        assert_that(defender.health()).is_equal_to(&Health::new(5));
    }

    #[test]
    fn fails_to_perform_attack_on_already_defeated_defender() {
        assert_that(&matches!(
            under_test().perform_attack(attack(), &mut defending_monster(0)),
            Err(CombatError::DefenderIsAlreadyDefeated)
        ))
        .is_true();
    }
}
