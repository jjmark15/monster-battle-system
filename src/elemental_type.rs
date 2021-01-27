use crate::primitive_element::PrimitiveElement;

#[derive(Debug, Eq, PartialEq)]
pub struct ElementalType {
    primary_primitive_type: PrimitiveElement,
    secondary_primitive_type: Option<PrimitiveElement>,
}

impl ElementalType {
    pub fn new(
        primary_primitive_type: PrimitiveElement,
        secondary_primitive_type: Option<PrimitiveElement>,
    ) -> Self {
        ElementalType {
            primary_primitive_type,
            secondary_primitive_type,
        }
    }

    pub fn primary_primitive_type(&self) -> &PrimitiveElement {
        &self.primary_primitive_type
    }

    pub fn secondary_primitive_type(&self) -> Option<&PrimitiveElement> {
        if let Some(element) = &self.secondary_primitive_type {
            return Some(element);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn returns_a_primary_primitive_type() {
        assert_that(&ElementalType::new(PrimitiveElement::Normal, None).primary_primitive_type())
            .is_equal_to(&PrimitiveElement::Normal);
    }

    #[test]
    fn returns_present_optional_secondary_primitive_type() {
        assert_that(
            &ElementalType::new(PrimitiveElement::Normal, Some(PrimitiveElement::Normal))
                .secondary_primitive_type()
                .unwrap(),
        )
        .is_equal_to(&PrimitiveElement::Normal);
    }

    #[test]
    fn returns_missing_optional_secondary_primitive_type() {
        assert_that(
            &ElementalType::new(PrimitiveElement::Normal, None)
                .secondary_primitive_type()
                .is_none(),
        )
        .is_true();
    }
}
