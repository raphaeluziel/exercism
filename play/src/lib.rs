use std::collections::{hash_map::Entry, HashMap};

const AMINOS: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    todo!("How much of nucleotide type '{nucleotide}' is contained inside DNA string '{dna}'?");
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut h:HashMap<char, usize> = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    let dna = "DCGATCGATTTTTTTTTTXSUR";
    for ch in dna.chars() {
        let ggg = h.entry(ch);//.and_modify(|n| *n += 1);
        if ggg == Vacant
        println!("GGG = {:?}", ggg);
    }

    println!("YOY {:?}", dna);
    println!("HEY {:?}", h);
    //Ok(h)
    todo!()
}
