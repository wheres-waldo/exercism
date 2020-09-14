const ACGT: &str = "ACGT";
const ACGU: &str = "ACGU";

#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        Ok(DNA {
            0: dna
                .char_indices()
                .map(|(i, c)| if ACGT.contains(c) { Ok(c) } else { Err(i) })
                .collect::<Result<String, usize>>()?,
        })
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            0: self.0.chars().map(self::dna_to_rna).collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        Ok(RNA {
            0: rna
                .char_indices()
                .map(|(i, c)| if ACGU.contains(c) { Ok(c) } else { Err(i) })
                .collect::<Result<String, usize>>()?,
        })
    }
}

fn dna_to_rna(nucleotide: char) -> char {
    match nucleotide {
        'A' => 'U',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => unreachable!(),
    }
}
