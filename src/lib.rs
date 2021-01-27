pub use attack::{Attack, AttackPower};
pub use combat_service::{CombatError, CombatService};
pub use damage::{Damage, DamageMultiplier};
pub use elemental_type::MonsterElement;
pub use health::Health;
pub use monster::Monster;
pub use primitive_element::Element;
pub use type_effectiveness::{TypeEffectivenessCalculator, TypeEffectivenessCalculatorImpl};

mod attack;
mod combat_service;
mod damage;
mod elemental_type;
mod health;
mod monster;
mod primitive_element;
mod type_effectiveness;
