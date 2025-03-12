pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs(u32),
    Peanuts(u32),
    Shellfish(u32),
    Strawberries(u32),
    Tomatoes(u32),
    Chocolate(u32),
    Pollen(u32),
    Cats(u32),
}

impl PartialEq<Allergen> for Allergen {
    fn eq(&self, other: &Allergen) -> bool {
        
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
        //todo!("Given the '{score}' score, construct a new Allergies struct.");
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        println!("SSS = {:?}", self.score);
        todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        todo!(
            "Return the list of allergens contained within the score with which the Allergies struct was made."
        );
    }
}
