use super::math;
use rand::prelude::*;
use std::convert::TryInto;

type Block = u64;
const BSIZE: usize = std::mem::size_of::<Block>();
type DataBlock = u32;
const DBSIZE: usize = std::mem::size_of::<DataBlock>();

pub(crate) fn to_prime(rng: &mut ThreadRng) -> u128 {
    let mut num = rng.next_u64() as u128;
    num &= !0u32 as u128;
    let high_limit = u64::MAX as u128 + 1;
    for i in num..high_limit {
        //println!("{}?", i);
        if math::is_prime(i) {
            return i;
        }
    }

    let low_limit = 1 << 64 - 1;
    for i in (low_limit..num).rev() {
        if math::is_prime(i) {
            return i;
        }
    }

    unreachable!("No prime numbers from {} to {}", low_limit, high_limit);
}

fn generate_pq(rng: &mut ThreadRng) -> (u128, u128) {
    let pq: Vec<_> = (0..2).map(|_| to_prime(rng)).collect();
    (pq[0], pq[1])
}

fn generate_e(rng: &mut ThreadRng, phi: u128) -> u128 {
    let e = rng.next_u64() as u128;
    for i in e..phi {
        if math::gcd(i, phi) == 1 {
            return i;
        }
    }

    for i in (0..e).rev() {
        if math::gcd(i, phi) == 1 {
            return i;
        }
    }

    unreachable!("No prime numbers from {} to {}", 0, phi);
}

pub(crate) fn generate_keys(rng: &mut ThreadRng) -> (u128, u128, u128) {
    let (p, q) = generate_pq(rng);
    let phi = (p - 1) * (q - 1);
    let e = generate_e(rng, phi);
    let d = math::rev(e, phi);
    (e, d, p * q)
}

pub(crate) fn run(input: &mut Vec<u8>, (k, n): (u128, u128), decode_flag: bool) -> Vec<u8> {
    if decode_flag {
        decode(input, (k, n))
    } else {
        encode(input, (k, n))
    }
}

// DataBlock -> Block
pub(crate) fn encode(input: &mut Vec<u8>, (k, n): (u128, u128)) -> Vec<u8> {
    align_message(input);
    let mut output = Vec::with_capacity(input.len() * 2);
    input.chunks_mut(DBSIZE).for_each(|chunk| {
        if chunk.len() != DBSIZE {
            return;
        }
        let arr: [u8; DBSIZE] = chunk.as_ref().try_into().expect("wrong block");
        let block = DataBlock::from_be_bytes(arr);
        let block = math::power(block as u128, k, n) as Block;
        let block = block.to_be_bytes();
        output.extend_from_slice(&block);
    });
    output
}

// Block -> DataBlock
pub(crate) fn decode(input: &mut Vec<u8>, (k, n): (u128, u128)) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len() / 2);
    input.chunks_mut(BSIZE).for_each(|chunk| {
        if chunk.len() != BSIZE {
            return;
        }
        let arr: [u8; BSIZE] = chunk.as_ref().try_into().expect("wrong block");
        let block = Block::from_be_bytes(arr);
        let block = math::power(block as u128, k, n);
        assert_eq!(0, block & !(!0u32 as u128));
        let block = (block & !0u32 as u128) as DataBlock;
        let block = block.to_be_bytes();
        output.extend_from_slice(&block);
    });
    unalign_message(&mut output);
    output
}

pub(crate) fn unalign_message(input: &mut Vec<u8>) {
    let align_val: [u8; DBSIZE] = input[(input.len() - DBSIZE)..input.len()]
        .try_into()
        .unwrap();
    let align_val = DataBlock::from_be_bytes(align_val);
    input.resize(input.len() - DBSIZE - align_val as usize, 0);
}

pub(crate) fn align_message(input: &mut Vec<u8>) {
    let align_val = ((DBSIZE - (input.len() % DBSIZE)) % DBSIZE) as DataBlock;
    let add = align_val.to_be_bytes();
    input.resize(input.len() + align_val as usize, 0);
    input.extend_from_slice(&add);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_all() {
        let mut rng = thread_rng();
        let mut input: Vec<_> = (0..64).map(|_| (rng.next_u64() & 0xff) as u8).collect();
        let input_clone = input.clone();
        let (e, d, n) = (
            6681871146242263643,
            2869256749771299779,
            12823143692123701271,
        );
        let mut output = encode(&mut input, (e, n));
        let input_from_output = decode(&mut output, (d, n));
        assert_eq!(input_from_output, input_clone);
    }

    #[test]
    fn check_align() {
        let mut rng = thread_rng();
        for i in 64..68 {
            let mut input: Vec<_> = (0..i).map(|_| (rng.next_u64() & 0xff) as u8).collect();
            let input_clone = input.clone();
            align_message(&mut input);
            unalign_message(&mut input);
            assert_eq!(input, input_clone, "for size {}", i);
        }
    }
}
