use tests::sha256_kmer_hash;

pub fn min_strobemer(k: usize, w: usize, n: usize, seq: &String) -> Vec<String>{
    // Validate inputs
    if k >= seq.len() {
        panic!("k-mer length must be less than sequence length");
    }
    if w >= seq.len() {
        panic!("Window size must be less than sequence length");
    }
    if n > seq.len() / w {
        panic!("Numer of segments must not exceed maximum number of possible windows");
    }

    let mut segments = Vec::with_capacity(n);

    // First segment is length of k-mer
    segments.push(seq[0..k].to_string());

    let mut start_idx = k;
    let mut kmers = Vec::new();

    // Remaining segments are calculated using minimisers
    for _ in 1..n {
        for i in 0..w {
            let start = start_idx + i;
            let end = start + k;
            kmers.push(&seq[start..end]);
        }
        let minimiser = kmers.iter().min_by_key(|&&kmer| sha256_kmer_hash(kmer)).unwrap();
        segments.push(minimiser.to_string());
        kmers.clear();
        start_idx += w;
    }
    segments
}

#[cfg(test)]
use tests::{input_seq, min_strobemer_on_sample};

#[test]
fn test_min_strobemer_on_sample_seq() {
    let seqs = input_seq();
    let k = 3;
    let w = 5;
    let n = 4;
    let mut result = min_strobemer(k, w, n, &seqs[0]);
    let expected = min_strobemer_on_sample();

    assert_eq!(result, expected[0]);

    result = min_strobemer(k, w, n, &seqs[1]);

    assert_eq!(result, expected[1]);
}

#[test]
fn test_min_strobemer_on_common_input() {
    let sequences = input_seq();

    for seq in sequences {
        let k = 3;
        let w = 5;
        let n = 4;

        let strobemer = min_strobemer(k, w, n, &seq);
        assert_eq!(strobemer.len(), n);
        for segment in &strobemer {
            assert_eq!(segment.len(), k);
        }
    }
}