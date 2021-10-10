const ORDER: [u64; 32] = [
    15, 6, 19, 20, 28, 11, 27, 16, 0, 14, 22, 25, 4, 17, 30, 9, 1, 7, 23, 13, 31, 26, 2, 8, 18, 12,
    29, 5, 21, 10, 3, 24,
];

pub(crate) fn apply(block: u64) -> u64 {
    let mut new_block = 0;
    for (i, from) in ORDER.iter().enumerate() {
        new_block |= (block.overflowing_shr(*from as u32).0 & 0b1) << i;
    }
    new_block
}
