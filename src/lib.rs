#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let invalid_index = dna.chars().position(|c| match c {
            'G' | 'C' | 'T' | 'A' => false,
            _ => true,
        });
        
        match invalid_index {
            Some(idx) => Err(idx),
            None => Ok(Dna(dna.to_string())),
        }
    }

    pub fn into_rna(self) -> Rna {
        let rna_string = self.0.chars().map(|c| match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => c,
        }).collect();
        Rna(rna_string)
    }

    pub fn sequence(&self) -> &str {
        &self.0
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let invalid_index = rna.chars().position(|c| match c {
            'G' | 'C' | 'A' | 'U' => false,
            _ => true,
        });
        
        match invalid_index {
            Some(idx) => Err(idx),
            None => Ok(Rna(rna.to_string())),
        }
    }

    pub fn sequence(&self) -> &str {
        &self.0
    }
}
