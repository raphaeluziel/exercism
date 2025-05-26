#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.find(|ch| !['A', 'C', 'G', 'T'].contains(&ch)) {
            Some(i) => Err(i),
            None => Ok(Dna { dna: dna.to_string() })
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna { rna: self.dna.chars().map(|d| 
            match d {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => '\0'
            }
        ).collect() }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.find(|ch| !['A', 'C', 'G', 'U'].contains(&ch)) {
            Some(i) => Err(i),
            None => Ok(Rna { rna: rna.to_string() })
        }
    }
}
