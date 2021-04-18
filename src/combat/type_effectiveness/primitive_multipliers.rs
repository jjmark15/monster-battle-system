use rust_decimal::Decimal;

use crate::combat::DamageMultiplier;
use crate::Element;

pub(super) enum PrimitiveDamageMultiplier {
    Half,
    Single,
    Double,
}

pub(super) fn normal_damage_multiplier(_defender_type: &Element) -> PrimitiveDamageMultiplier {
    PrimitiveDamageMultiplier::Single
}

pub(super) fn fire_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Water | Element::Fire => PrimitiveDamageMultiplier::Half,
        Element::Grass => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn grass_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fire | Element::Grass | Element::Flying => PrimitiveDamageMultiplier::Half,
        Element::Water => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn water_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Grass | Element::Water | Element::Electric => PrimitiveDamageMultiplier::Half,
        Element::Fire => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn electric_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Water | Element::Flying => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn flying_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Grass | Element::Fighting => PrimitiveDamageMultiplier::Double,
        Element::Electric => PrimitiveDamageMultiplier::Half,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn fighting_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Normal => PrimitiveDamageMultiplier::Double,
        Element::Flying => PrimitiveDamageMultiplier::Half,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

impl From<PrimitiveDamageMultiplier> for DamageMultiplier {
    fn from(primitive: PrimitiveDamageMultiplier) -> Self {
        let decimal = match primitive {
            PrimitiveDamageMultiplier::Half => Decimal::new(5, 1),
            PrimitiveDamageMultiplier::Single => 1.into(),
            PrimitiveDamageMultiplier::Double => 2.into(),
        };
        DamageMultiplier::new(decimal)
    }
}
