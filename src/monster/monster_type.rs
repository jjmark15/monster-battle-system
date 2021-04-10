use crate::elements::Element;

#[derive(Debug, Eq, PartialEq)]
pub struct MonsterType {
    primary_element: Element,
    secondary_element: Option<Element>,
}

impl MonsterType {
    pub fn new(primary_element: Element, secondary_element: Option<Element>) -> Self {
        MonsterType {
            primary_element,
            secondary_element,
        }
    }

    pub fn primary_element(&self) -> &Element {
        &self.primary_element
    }

    pub fn secondary_element(&self) -> Option<&Element> {
        if let Some(element) = &self.secondary_element {
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
    fn returns_a_primary_element() {
        assert_that(&MonsterType::new(Element::Normal, None).primary_element())
            .is_equal_to(&Element::Normal);
    }

    #[test]
    fn returns_present_optional_secondary_element() {
        assert_that(
            &MonsterType::new(Element::Normal, Some(Element::Normal))
                .secondary_element()
                .unwrap(),
        )
        .is_equal_to(&Element::Normal);
    }

    #[test]
    fn returns_missing_optional_secondary_element() {
        assert_that(
            &MonsterType::new(Element::Normal, None)
                .secondary_element()
                .is_none(),
        )
        .is_true();
    }
}
