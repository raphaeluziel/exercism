#[derive(Debug)]
pub struct Allergies<'a> {
    allergies: Vec<&'a Allergen>,
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

const ALLERGIES: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies<'_> {
    pub fn new(score: u32) -> Self {
        let allergies: Vec<&Allergen> = format!("{score:b}")
            .chars()
            .rev()
            .enumerate()
            .filter(|x| x.1 == '1')
            .map(|x| &ALLERGIES[x.0])
            .collect();

        Allergies { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_num = match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        };

        let allergy = format!("{allergen_num:b}")
            .chars()
            .into_iter()
            .rev()
            .position(|x| x == '1')
            .unwrap();

        //self.allergies.get(allergy).unwrap() == &'1'
        todo!("HEY")
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let output: Vec<Allergen> = Vec::new();
        println!("Self {:?}", self);

        output
    }
}
