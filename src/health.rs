#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Health(u16);

impl Health {
    pub fn new(value: u16) -> Self {
        Health(value)
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    const HEALTH_VALUE: u16 = 10;

    #[test]
    fn returns_integer_value() {
        assert_that(&Health::new(HEALTH_VALUE).value()).is_equal_to(HEALTH_VALUE);
    }
}
