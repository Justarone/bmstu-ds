use helpers::{align_message, decode, encode, generate_keys, unalign_message};

mod helpers;

pub(crate) fn cipher(input: &mut Vec<u8>, key: &[u8]) {
    align_message(input);
    process_input(input, key, encode);
}

pub(crate) fn decipher(input: &mut Vec<u8>, key: &[u8]) {
    process_input(input, key, decode);
    unalign_message(input);
}

pub(crate) fn process_input<F>(input: &mut Vec<u8>, key: &[u8], proc_fn: F)
where
    F: Fn(&[u8], &[Vec<u8>]) -> Vec<u8>,
{
    let keys = generate_keys(key);
    input.chunks_mut(16).for_each(|chunk| {
        if chunk.len() != 16 {
            return;
        }
        let block = proc_fn(&chunk, &keys);
        chunk.copy_from_slice(&block);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_symmetry() {
        let input = vec![17; 131];
        let mut modified = input.clone();
        let key = vec![1; 16];
        cipher(&mut modified, &key);
        println!("{:?}", modified);
        decipher(&mut modified, &key);
        assert_eq!(input, modified);
    }
}
