use sha2::{Sha256, Digest};

// Test inputs
pub fn input_seq() -> Vec<String> {
    let sequences: [&str; 2] = [
        "ATGCGTACGTTAGCTAGTCA",
        "TGCGAATCGTAGCTGACTGAGCGATGCTAGTACGTAGCTA",
    ];

    sequences.iter().map(|&s| s.to_string()).collect()
}

// Hash function for testing
pub fn sha256_kmer_hash(kmer: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(kmer.as_bytes());
    let result = hasher.finalize();
    let mut array = [0u8; 32];
    array.copy_from_slice(&result);
    array
}

pub fn min_strobemer_on_sample() -> Vec<Vec<String>> {
    // Tests for first strobemer of a sequence
    // Given k = 3, w = 5, n = 4

    let mut expected = Vec::new();
    expected.push(vec![
        "ATG".to_string(),
        "GTA".to_string(),
        "TAG".to_string(),
        "CTA".to_string(),
    ]);
    expected.push(vec![
        "TGC".to_string(),
        "CGT".to_string(),
        "TAG".to_string(),
        "TGA".to_string(),
    ]);
    expected
}