use rust_decimal::Decimal;

#[derive(Debug, Eq, PartialEq)]
pub struct AttackPower(Decimal);

impl AttackPower {
    pub fn new(value: Decimal) -> Self {
        AttackPower(value)
    }

    pub fn value(&self) -> Decimal {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn returns_integer_value() {
        let attack_value = 10.into();
        assert_that(&AttackPower::new(attack_value).value()).is_equal_to(&attack_value);
    }
}
