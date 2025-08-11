use tests::input_seq;
use dawnandrew100::min_strobemer;

fn main() {
    let test = input_seq();
    println!("{:?}", &test[0]);
    let sequence = &test[0];
    let kmer = 3;
    let window_size = 5;
    let seg_len = 4;

    let strobes = min_strobemer(kmer, window_size, seg_len, &sequence);
    println!("{:?}", strobes);
}
