const ORDER: [u64; 48] = [
    31, 0, 1, 2, 3, 4, 3, 4, 5, 6, 7, 8, 7, 8, 9, 10, 11, 12, 11, 12, 13, 14, 15, 16, 15, 16, 17,
    18, 19, 20, 19, 20, 21, 22, 23, 24, 23, 24, 25, 26, 27, 28, 27, 28, 29, 30, 31, 0,
];

pub(crate) fn apply(block: u64) -> u64 {
    let mut new_block = 0;
    for (i, from) in ORDER.iter().enumerate() {
        new_block |= (block.overflowing_shr(*from as u32).0 & 0b1) << i;
    }
    new_block
}
