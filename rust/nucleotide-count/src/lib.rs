use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => {
            let mut count = 0usize;
            for c in dna.chars() {
                match c {
                    'A' | 'C' | 'G' | 'T' => {
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

    for nucleotide in "ACGT".chars() {
        counts.insert(nucleotide, count(nucleotide, dna)?);
    }

    Ok(counts)


}
