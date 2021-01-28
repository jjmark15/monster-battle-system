use crate::elements::Element;

#[derive(Debug, Eq, PartialEq)]
pub struct MonsterElements {
    primary_primitive_type: Element,
    secondary_primitive_type: Option<Element>,
}

impl MonsterElements {
    pub fn new(primary_primitive_type: Element, secondary_primitive_type: Option<Element>) -> Self {
        MonsterElements {
            primary_primitive_type,
            secondary_primitive_type,
        }
    }

    pub fn primary_primitive_type(&self) -> &Element {
        &self.primary_primitive_type
    }

    pub fn secondary_primitive_type(&self) -> Option<&Element> {
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
        assert_that(&MonsterElements::new(Element::Normal, None).primary_primitive_type())
            .is_equal_to(&Element::Normal);
    }

    #[test]
    fn returns_present_optional_secondary_primitive_type() {
        assert_that(
            &MonsterElements::new(Element::Normal, Some(Element::Normal))
                .secondary_primitive_type()
                .unwrap(),
        )
        .is_equal_to(&Element::Normal);
    }

    #[test]
    fn returns_missing_optional_secondary_primitive_type() {
        assert_that(
            &MonsterElements::new(Element::Normal, None)
                .secondary_primitive_type()
                .is_none(),
        )
        .is_true();
    }
}
