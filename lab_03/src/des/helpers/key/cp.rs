const ORDER: [u8; 48] = [
    13, 16, 10, 23, 0, 4, 2, 27, 14, 5, 20, 9, 22, 18, 11, 3, 25, 7, 15, 6, 26, 19, 12, 1, 40, 51,
    30, 36, 46, 54, 29, 39, 50, 44, 32, 47, 43, 48, 38, 55, 33, 52, 45, 41, 49, 35, 28, 31,
];

pub(crate) fn apply(block: u64) -> u64 {
    let mut new_block = 0;
    for (i, from) in ORDER.iter().enumerate() {
        new_block |= (block.overflowing_shr(*from as u32).0 & 0b1)
            .overflowing_shl(i as u32)
            .0;
    }
    new_block
}
