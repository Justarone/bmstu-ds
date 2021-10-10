mod align;
mod feistel;
mod ip;
mod key;

pub(crate) use align::{align_message, unalign_message};
use feistel::f;
pub(crate) use key::generate_keys;

pub(crate) fn encode(block: u64, keys: &[u64]) -> u64 {
    let block = ip::apply(block);
    let mut l = block.overflowing_shr(32).0;
    let mut r = block & (!0u32 as u64);
    for k in keys.iter() {
        let new_l = r;
        r = l ^ f(r, *k);
        l = new_l;
    }
    let block = (l << 32) | r;
    ip::revapply(block)
}

pub(crate) fn decode(block: u64, keys: &[u64]) -> u64 {
    let block = ip::apply(block);
    let mut l = block.overflowing_shr(32).0;
    let mut r = block & (!0u32 as u64);
    for k in keys.iter().rev() {
        let new_r = l;
        l = r ^ f(l, *k);
        r = new_r;
    }
    let block = (l << 32) | r;
    ip::revapply(block)
}

#[cfg(test)]
mod tests {
    use super::{decode, encode, generate_keys};

    #[test]
    fn check_symmetry() {
        let msg = 0x4839_fa27_fdca_1938;
        let key = 0xfca2_db21_10f3_d4ca;
        let keys = generate_keys(key);
        let c = encode(msg, &keys);
        let new_msg = decode(c, &keys);
        assert_eq!(msg, new_msg, "hex: {:#x} != {:#x}", msg, new_msg);
    }
}
