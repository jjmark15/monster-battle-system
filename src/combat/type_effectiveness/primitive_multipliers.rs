use rust_decimal::Decimal;

use crate::combat::DamageMultiplier;
use crate::Element;

pub(super) enum PrimitiveDamageMultiplier {
    Zero,
    Half,
    Single,
    Double,
}

pub(super) fn bug_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fire
        | Element::Fighting
        | Element::Poison
        | Element::Flying
        | Element::Ghost
        | Element::Steel
        | Element::Fairy => PrimitiveDamageMultiplier::Half,
        Element::Grass | Element::Psychic | Element::Dark => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn dark_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fighting | Element::Dark | Element::Fairy => PrimitiveDamageMultiplier::Half,
        Element::Psychic | Element::Ghost => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn dragon_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fairy => PrimitiveDamageMultiplier::Zero,
        Element::Steel => PrimitiveDamageMultiplier::Half,
        Element::Dragon => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn electric_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Ground => PrimitiveDamageMultiplier::Zero,
        Element::Electric | Element::Grass | Element::Dragon => PrimitiveDamageMultiplier::Half,
        Element::Water | Element::Flying => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn fairy_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fire | Element::Poison | Element::Steel => PrimitiveDamageMultiplier::Half,
        Element::Fighting | Element::Dragon | Element::Dark => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn fighting_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Ghost => PrimitiveDamageMultiplier::Zero,
        Element::Poison | Element::Flying | Element::Psychic | Element::Bug | Element::Fairy => {
            PrimitiveDamageMultiplier::Half
        }
        Element::Normal | Element::Ice | Element::Rock | Element::Dark | Element::Steel => {
            PrimitiveDamageMultiplier::Double
        }
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn fire_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fire | Element::Water | Element::Rock | Element::Dragon => {
            PrimitiveDamageMultiplier::Half
        }
        Element::Grass | Element::Ice | Element::Bug | Element::Steel => {
            PrimitiveDamageMultiplier::Double
        }
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn flying_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Electric | Element::Rock | Element::Steel => PrimitiveDamageMultiplier::Half,
        Element::Grass | Element::Fighting | Element::Bug => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn ghost_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Normal => PrimitiveDamageMultiplier::Zero,
        Element::Dark => PrimitiveDamageMultiplier::Half,
        Element::Psychic | Element::Ghost => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn grass_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fire
        | Element::Grass
        | Element::Poison
        | Element::Flying
        | Element::Bug
        | Element::Dragon
        | Element::Steel => PrimitiveDamageMultiplier::Half,
        Element::Water | Element::Ground | Element::Rock => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn ground_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Flying => PrimitiveDamageMultiplier::Zero,
        Element::Grass | Element::Bug => PrimitiveDamageMultiplier::Half,
        Element::Fire | Element::Electric | Element::Poison | Element::Rock | Element::Steel => {
            PrimitiveDamageMultiplier::Double
        }
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn ice_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fire | Element::Water | Element::Ice | Element::Steel => {
            PrimitiveDamageMultiplier::Half
        }
        Element::Grass | Element::Ground | Element::Flying | Element::Dragon => {
            PrimitiveDamageMultiplier::Double
        }
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn normal_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Ghost => PrimitiveDamageMultiplier::Zero,
        Element::Rock | Element::Steel => PrimitiveDamageMultiplier::Half,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn poison_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Steel => PrimitiveDamageMultiplier::Zero,
        Element::Poison | Element::Ground | Element::Rock | Element::Ghost => {
            PrimitiveDamageMultiplier::Half
        }
        Element::Grass | Element::Fairy => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn psychic_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Dark => PrimitiveDamageMultiplier::Zero,
        Element::Psychic | Element::Steel => PrimitiveDamageMultiplier::Half,
        Element::Fighting | Element::Poison => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn rock_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fighting | Element::Ground | Element::Steel => PrimitiveDamageMultiplier::Half,
        Element::Fire | Element::Ice | Element::Flying | Element::Bug => {
            PrimitiveDamageMultiplier::Double
        }
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn steel_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Fire | Element::Water | Element::Electric | Element::Steel => {
            PrimitiveDamageMultiplier::Half
        }
        Element::Ice | Element::Rock | Element::Fairy => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
}

pub(super) fn water_damage_multiplier(defender_type: &Element) -> PrimitiveDamageMultiplier {
    match defender_type {
        Element::Water | Element::Grass | Element::Dragon => PrimitiveDamageMultiplier::Half,
        Element::Fire | Element::Ground | Element::Rock => PrimitiveDamageMultiplier::Double,
        _ => PrimitiveDamageMultiplier::Single,
    }
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
