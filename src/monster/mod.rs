use std::ops::Sub;

pub use attack::*;
pub use damage::Damage;
pub use health::Health;
pub use monster_type::MonsterType;

mod attack;
mod damage;
mod health;
mod monster_type;

pub struct Monster {
    monster_type: MonsterType,
    health: Health,
}

impl Monster {
    pub fn new(monster_type: MonsterType, health: Health) -> Self {
        Monster {
            monster_type,
            health,
        }
    }

    pub fn monster_type(&self) -> &MonsterType {
        &self.monster_type
    }

    pub fn receive_damage(&mut self, damage: Damage) {
        self.health = self.health - damage;
    }

    pub fn health(&self) -> &Health {
        &self.health
    }
}

impl Sub<Damage> for Health {
    type Output = Health;

    fn sub(self, rhs: Damage) -> Self::Output {
        let damage_applied_value = self.value() - rhs.value();

        if damage_applied_value < 0.into() {
            Health::new(0.into())
        } else {
            Health::new(damage_applied_value)
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::elements::Element;

    use super::*;

    fn under_test() -> Monster {
        Monster::new(monster_type(), Health::new(10.into()))
    }

    fn monster_type() -> MonsterType {
        MonsterType::new(Element::Normal, None)
    }

    #[test]
    fn returns_its_health() {
        assert_that(&under_test().health()).is_equal_to(&Health::new(10.into()));
    }

    #[test]
    fn returns_its_monster_type() {
        assert_that(&under_test().monster_type()).is_equal_to(&monster_type());
    }

    #[test]
    fn health_is_affected_by_damage() {
        let mut monster = under_test();
        monster.receive_damage(Damage::new(5.into()));

        assert_that(&monster.health()).is_equal_to(&Health::new(5.into()));
    }

    #[test]
    fn damage_cannot_lower_health_below_zero() {
        let mut monster = under_test();
        monster.receive_damage(Damage::new(15.into()));

        assert_that(&monster.health()).is_equal_to(&Health::new(0.into()));
    }

    #[test]
    fn negative_damage_adds_to_health() {
        let mut monster = under_test();
        monster.receive_damage(Damage::new((-5).into()));

        assert_that(&monster.health()).is_equal_to(&Health::new(15.into()));
    }
}
