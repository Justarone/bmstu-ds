use common::*;
pub(crate) use key::generate_keys;
use mix_columns::{mix_columns, mix_columns_inv};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

mod common;
mod key;
mod mix_columns;
mod sbox;

pub(crate) fn encode(block: &[u8], keys: &[Vec<u8>]) -> Vec<u8> {
    let mut block = block.to_vec();
    xor_with_key(&mut block, &keys[0]);
    for (i, key) in keys.iter().enumerate().skip(1) {
        sub_slice(&mut block);
        rot_block_left(&mut block);
        if i != keys.len() - 1 {
            mix_columns(&mut block);
        }
        xor_with_key(&mut block, key);
    }
    block
}

pub(crate) fn decode(block: &[u8], keys: &[Vec<u8>]) -> Vec<u8> {
    let mut block = block.to_vec();
    xor_with_key(&mut block, &keys.last().unwrap());
    for (i, key) in keys.iter().rev().skip(1).enumerate() {
        if i != 0 {
            mix_columns_inv(&mut block);
        }
        rot_block_right(&mut block);
        sub_slice_inv(&mut block);
        xor_with_key(&mut block, key);
    }
    block
}

pub(crate) fn align_message(msg: &mut Vec<u8>) {
    let additional = ((16 - (msg.len() % 16)) % 16) as u8;
    msg.resize(msg.len() + additional as usize, 0);
    msg.push(additional);
}

pub(crate) fn unalign_message(msg: &mut Vec<u8>) {
    assert!(
        msg.len() >= 16 + 1,
        "minimal aes encrypted message size = 16b + 1b (size of extended part)"
    );
    let to_remove = msg.pop().unwrap();
    msg.resize(msg.len() - to_remove as usize, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_symmetry() {
        let mut rand = StdRng::seed_from_u64(17);
        let key: Vec<_> = (0..16).map(|_| rand.gen()).collect();
        let msg: Vec<_> = (0..16).map(|_| rand.gen()).collect();
        let keys = generate_keys(key.as_slice());
        let c = encode(&msg, &keys);
        let new_msg = decode(&c, &keys);
        assert_eq!(new_msg, msg, "cipher is not symmetric");
    }
}
