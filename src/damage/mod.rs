mod multiplier;

pub use multiplier::DamageMultiplier;
use rust_decimal::Decimal;

pub struct Damage(Decimal);

impl Damage {
    pub fn new(value: Decimal) -> Self {
        Damage(value)
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
        let damage_value = 10.into();
        assert_that(&Damage::new(damage_value).value()).is_equal_to(damage_value);
    }
}
