pub use power::AttackPower;

use crate::Element;

mod power;

pub struct Attack {
    element: Element,
    power: AttackPower,
}

impl Attack {
    pub fn new(element: Element, power: AttackPower) -> Self {
        Attack { element, power }
    }

    pub fn element(&self) -> &Element {
        &self.element
    }

    pub fn power(&self) -> &AttackPower {
        &self.power
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    fn under_test() -> Attack {
        Attack::new(Element::Normal, AttackPower::new(10.into()))
    }

    #[test]
    fn returns_its_element() {
        assert_that(&under_test().element()).is_equal_to(&Element::Normal);
    }

    #[test]
    fn returns_its_power() {
        assert_that(&under_test().power()).is_equal_to(&AttackPower::new(10.into()));
    }
}
