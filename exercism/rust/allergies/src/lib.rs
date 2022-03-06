use enum_iterator::IntoEnumIterator;

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        self.0 & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::into_enum_iter()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
