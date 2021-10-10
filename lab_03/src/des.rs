use helpers::{align_message, decode, encode, generate_keys, unalign_message};
use std::convert::TryInto;

mod helpers;

pub(crate) fn cipher(input: &mut Vec<u8>, key: u64) {
    align_message(input);
    process_input(input, key, encode);
}

pub(crate) fn decipher(input: &mut Vec<u8>, key: u64) {
    process_input(input, key, decode);
    unalign_message(input);
}

fn process_input<F>(input: &mut Vec<u8>, key: u64, proc_fn: F)
where
    F: Fn(u64, &[u64]) -> u64,
{
    let keys = generate_keys(key);
    input.chunks_mut(8).for_each(|chunk| {
        if chunk.len() != 8 {
            return;
        }
        let arr: [u8; 8] = chunk.as_ref().try_into().expect("block is not 8 bytes");
        let block = u64::from_be_bytes(arr);
        let block = proc_fn(block, &keys);
        let block = block.to_be_bytes();
        chunk.copy_from_slice(&block);
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_be_bytes() {
        assert_eq!(
            u64::from_be_bytes([0xde, 0xad, 0xbe, 0xaf, 0x12, 0x23, 0x48, 0x23]),
            0xdead_beaf_1223_4823_u64
        );
        assert_eq!(
            [0xde, 0xad, 0xbe, 0xaf, 0x12, 0x23, 0x48, 0x23],
            0xdead_beaf_1223_4823_u64.to_be_bytes()
        );
    }
}
