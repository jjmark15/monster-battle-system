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

pub(crate) enum PrimitiveDamageMultiplier {
    Zero,
    Half,
    Single,
    Double,
}

impl From<PrimitiveDamageMultiplier> for DamageMultiplier {
    fn from(primitive: PrimitiveDamageMultiplier) -> Self {
        let decimal = match primitive {
            PrimitiveDamageMultiplier::Zero => 0.into(),
            PrimitiveDamageMultiplier::Half => Decimal::new(5, 1),
            PrimitiveDamageMultiplier::Single => 1.into(),
            PrimitiveDamageMultiplier::Double => 2.into(),
        };
        DamageMultiplier::new(decimal)
    }
}
