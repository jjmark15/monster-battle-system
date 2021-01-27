pub use power::AttackPower;

use crate::PrimitiveElement;

mod power;

pub struct Attack {
    element: PrimitiveElement,
    power: AttackPower,
}

impl Attack {
    pub fn new(element: PrimitiveElement, power: AttackPower) -> Self {
        Attack { element, power }
    }

    pub fn element(&self) -> &PrimitiveElement {
        &self.element
    }

    pub fn power(&self) -> &AttackPower {
        &self.power
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    fn under_test() -> Attack {
        Attack::new(PrimitiveElement::Normal, AttackPower::new(10))
    }

    #[test]
    fn returns_its_element() {
        assert_that(&under_test().element()).is_equal_to(&PrimitiveElement::Normal);
    }

    #[test]
    fn returns_its_power() {
        assert_that(&under_test().power()).is_equal_to(&AttackPower::new(10));
    }
}
