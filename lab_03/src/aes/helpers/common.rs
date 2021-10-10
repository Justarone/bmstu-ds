use super::sbox;

pub(crate) fn rot_slice_left(src: &mut [u8], shift: usize) {
    src.rotate_left(shift);
}

pub(crate) fn rot_slice_right(src: &mut [u8], shift: usize) {
    src.rotate_right(shift);
}

pub(crate) fn rot_block_right(block: &mut [u8]) {
    for (i, row) in block.chunks_mut(4).enumerate().skip(1) {
        rot_slice_right(row, i);
    }
}

pub(crate) fn rot_block_left(block: &mut [u8]) {
    for (i, row) in block.chunks_mut(4).enumerate().skip(1) {
        rot_slice_left(row, i);
    }
}

pub(crate) fn sub_slice(src: &mut [u8]) {
    src.iter_mut().for_each(|b| *b = sbox::get(*b));
}

pub(crate) fn sub_slice_inv(src: &mut [u8]) {
    src.iter_mut().for_each(|b| *b = sbox::get_inv(*b));
}

pub(crate) fn xor(src: &mut [u8], with: &[u8]) {
    src.iter_mut().zip(with.iter()).for_each(|(b, rc)| *b ^= rc)
}

pub(crate) fn xor_with_key(block: &mut [u8], key: &[u8]) {
    // block[i][j] ^= key[j][i] (key = [word1, word2, word3, word4])
    block
        .iter_mut()
        .enumerate()
        .map(|(i, b)| (i / 4 + (i % 4) * 4, b))
        .for_each(|(i, b)| *b ^= key[i] ^ key[i]);
}
