const C: [[u8; 4]; 4] = [
    [0x02, 0x03, 0x01, 0x01],
    [0x01, 0x02, 0x03, 0x01],
    [0x01, 0x01, 0x02, 0x03],
    [0x03, 0x01, 0x01, 0x02],
];

const CINV: [[u8; 4]; 4] = [
    [0x0e, 0x0b, 0x0d, 0x09],
    [0x09, 0x0e, 0x0b, 0x0d],
    [0x0d, 0x09, 0x0e, 0x0b],
    [0x0b, 0x0d, 0x09, 0x0e],
];

pub(crate) fn mix_columns(block: &mut [u8]) {
    mix_columns_impl(block, &C);
}

pub(crate) fn mix_columns_inv(block: &mut [u8]) {
    mix_columns_impl(block, &CINV);
}

pub(crate) fn mix_columns_impl(block: &mut [u8], c: &[[u8; 4]; 4]) {
    let mut new_block = vec![0; 16];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                new_block[i * 4 + k] ^= mul(c[i][j], block[j * 4 + k]);
            }
        }
    }
    block.copy_from_slice(&new_block);
}

pub(crate) fn mul(a: u8, b: u8) -> u8 {
    let mut current = a;
    let mut res = 0;
    for i in 0..8 {
        if b & (1 << i) != 0 {
            res ^= current;
        }
        current = timex(current);
    }
    res
}

pub(crate) fn timex(a: u8) -> u8 {
    let a = (a as u16) << 1;
    let res = if a & (1 << 8) != 0 {
        (a & 0xff) ^ 0x1b
    } else {
        a
    };
    res as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn check_symmetry() {
        let mut rand = StdRng::seed_from_u64(17);
        let block: Vec<_> = (0..16).map(|_| rand.gen()).collect();
        let mut new_block = block.to_vec();
        mix_columns(&mut new_block);
        mix_columns_inv(&mut new_block);
        assert_eq!(block, new_block, "mixing columns isn't symmetric");
    }

    #[test]
    fn check_mul() {
        assert_eq!(mul(0x57, 0x13), 0xfe);
    }

    #[test]
    fn check_timex() {
        assert_eq!(timex(0x57), 0xae);
        assert_eq!(timex(0xae), 0x47);
        assert_eq!(timex(0x47), 0x8e);
        assert_eq!(timex(0x8e), 0x07);
    }
}
