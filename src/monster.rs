use std::ops::Sub;

use crate::damage::Damage;
use crate::elemental_type::ElementalType;
use crate::health::Health;

pub struct Monster {
    elemental_type: ElementalType,
    health: Health,
}

impl Monster {
    pub fn new(elemental_type: ElementalType, health: Health) -> Self {
        Monster {
            elemental_type,
            health,
        }
    }

    pub fn elemental_type(&self) -> &ElementalType {
        &self.elemental_type
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

    use crate::primitive_element::PrimitiveElement;

    use super::*;

    fn under_test() -> Monster {
        Monster::new(elemental_type(), Health::new(10.into()))
    }

    fn elemental_type() -> ElementalType {
        ElementalType::new(PrimitiveElement::Normal, None)
    }

    #[test]
    fn returns_its_health() {
        assert_that(&under_test().health()).is_equal_to(&Health::new(10.into()));
    }

    #[test]
    fn returns_its_elemental_type() {
        assert_that(&under_test().elemental_type()).is_equal_to(&elemental_type());
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
