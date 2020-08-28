use std::collections::HashMap;

const ACGT: &str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        _ if ACGT.contains(nucleotide) => {
            let mut count = 0usize;
            for c in dna.chars() {
                match c {
                    _ if ACGT.contains(c) => {
                        if c == nucleotide {
                            count += 1;
                        }
                    }
                    _ => return Err(c),
                }
            }
            Ok(count)
        }
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();

    for nucleotide in ACGT.chars() {
        counts.insert(nucleotide, count(nucleotide, dna)?);
    }

    Ok(counts)
}
