
// input: The original bits (upto 64) packed into a u64
// table: The permutation table (like IP/FP/E/P) each entry is a 1-based bit position from the input. MSB first
// in_bits: How many of the top bits of input are valid
pub fn permute(input: u64, table: &[u8], in bits: usize) -> 64 {
    // DES tables are 1-based and MSB-first.
    let mut out = 0u64;

    for (i, &pos1) in table.iter().enumerate() {
        let src = in_bits - (pos1 as usize);
        let bit = (input >> src) & 1;
        out |= bit << (table.len() - 1 - i)
    }
    out
}