use rust_decimal::Decimal;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Health(Decimal);

impl Health {
    pub fn new(value: Decimal) -> Self {
        Health(value)
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
        let health_value = Decimal::from(10);
        assert_that(&Health::new(health_value).value()).is_equal_to(health_value);
    }
}
