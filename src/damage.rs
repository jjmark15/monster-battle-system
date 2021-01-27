pub struct Damage(i16);

impl Damage {
    pub fn new(value: i16) -> Self {
        Damage(value)
    }

    pub fn value(&self) -> i16 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    const DAMAGE_VALUE: i16 = 10;

    #[test]
    fn returns_integer_value() {
        assert_that(&Damage::new(DAMAGE_VALUE).value()).is_equal_to(DAMAGE_VALUE);
    }
}
