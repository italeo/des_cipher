pub fn permute(input: u64, table: &[u8], in bits: usize) -> 64 {
    // DES tables are 1-based and MSB-first.
    let mut out = 0u64;

    for (i, &pos1) in table.iter().enumerate() {
        let src = in_bits - (pos1 as usize)
    }

}