use std::task::Poll;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
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
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        println!("Allergen = {:?}", allergen);
        println!("SSS = {:?}", self.score);
        let x = 11;
        let st = format!("{x:b}");
        let xo:Vec<_> = st.chars().map(|x| x.to_digit(2).unwrap_or_default()).collect();
        println!("CH = {:?}", xo);
        println!("ST = {st}");
        let allergen_num = match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128
        };
        println!("")

        todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        todo!(
            "Return the list of allergens contained within the score with which the Allergies struct was made."
        );
    }
}
