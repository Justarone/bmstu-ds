const ORDER: [u8; 56] = [
    56, 48, 40, 32, 24, 16, 8, 0, 57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59,
    51, 43, 34, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29, 21, 13, 5, 60, 52, 44, 36, 28,
    20, 12, 4, 27, 19, 11, 3,
];

pub(crate) fn apply(block: u64) -> u64 {
    let mut new_block = 0;
    for (i, from) in ORDER.iter().enumerate() {
        new_block |= (block.overflowing_shr(*from as u32).0 & 0b1) << i;
    }
    new_block
}
