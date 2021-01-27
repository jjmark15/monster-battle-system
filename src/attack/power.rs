#[derive(Debug, Eq, PartialEq)]
pub struct AttackPower(i16);

impl AttackPower {
    pub fn new(value: i16) -> Self {
        AttackPower(value)
    }

    pub fn value(&self) -> i16 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    const POWER_VALUE: i16 = 10;

    #[test]
    fn returns_integer_value() {
        assert_that(&AttackPower::new(POWER_VALUE).value()).is_equal_to(POWER_VALUE);
    }
}
