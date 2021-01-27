use rust_decimal::Decimal;

#[derive(Debug, Eq, PartialEq)]
pub struct DamageMultiplier {
    value: Decimal,
}

impl DamageMultiplier {
    pub fn new(value: Decimal) -> Self {
        DamageMultiplier { value }
    }

    pub fn value(&self) -> Decimal {
        self.value
    }

    pub fn combined_with(self, other: Self) -> Self {
        DamageMultiplier::new(self.value * other.value())
    }
}
