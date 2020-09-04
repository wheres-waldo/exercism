use lazy_static::lazy_static;
use std::collections::BTreeMap;
use Allergen::*;

lazy_static! {
    static ref ALLERGIES: BTreeMap<Allergen, u8> = [
        (Eggs, 0),
        (Peanuts, 1),
        (Shellfish, 2),
        (Strawberries, 3),
        (Tomatoes, 4),
        (Chocolate, 5),
        (Pollen, 6),
        (Cats, 7)
    ]
    .iter()
    .cloned()
    .collect();
}

pub struct Allergies(u32);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & (1 << ALLERGIES.get(allergen).unwrap()) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGIES
            .keys()
            .cloned()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
