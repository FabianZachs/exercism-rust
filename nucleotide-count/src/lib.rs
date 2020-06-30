use std::collections::HashMap;

const VALID_NUCLEOTIDE: [char; 4] = ['A', 'C', 'G', 'T'];
// Valid types A, C,  G, T
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDE.contains(&nucleotide) {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if !VALID_NUCLEOTIDE.contains(&c) {
            return Err(c);
        }
    }
    return Ok(dna.chars().filter(|&x| x == nucleotide).count());
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    for &c in VALID_NUCLEOTIDE.iter() {
        counts.insert(c, 0);
    }

    for c in dna.chars() {
        if !VALID_NUCLEOTIDE.contains(&c) {
            return Err(c);
        }
        let reference = counts.get_mut(&c).unwrap();
        *reference += 1;
    }

    Ok(counts)
    // "ACGT".chars().map(|c| (c,count(c,dna))).collect()
}
