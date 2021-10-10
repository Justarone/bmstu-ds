mod b;
mod cp;

pub(crate) fn generate_keys(mut key: u64) -> Vec<u64> {
    key = extend_key(key);
    key = compress_key(key);
    let keys = generate_more(key);
    keys
}

fn extend_key(key: u64) -> u64 {
    let mut new_key = 0;
    for i in 0..8 {
        let mut tmp = (key & (0b111_1111 << (i * 7))) << i;
        if tmp.count_ones() % 2 == 0 {
            tmp |= 1 << (i * 8 + 7);
        }
        new_key |= tmp;
    }
    new_key
}

fn compress_key(key: u64) -> u64 {
    b::apply(key)
}

#[allow(dead_code)]
fn compress_key_bad(mut key: u64) -> u64 {
    let mut new_k = 0;
    for i in 0..8 {
        new_k |= (key & 0b1111_1110).overflowing_shl(i * 7).0;
        key = key.overflowing_shr(8).0;
    }
    new_k
}

fn generate_more(key: u64) -> Vec<u64> {
    let mut keys = Vec::with_capacity(16);
    let mut ci = key & 0x0f_ff_ff_ff;
    let mut di = key.overflowing_shr(28).0 & 0x0fff_ffff;
    for i in [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1] {
        ci = (ci.overflowing_shl(i).0) & (0x0fff_ffff) | (key.overflowing_shr(28 - i).0);
        di = (di.overflowing_shl(i).0) & (0x0fff_ffff) | (key.overflowing_shr(28 - i).0);
        keys.push(di << 28 | ci);
    }
    keys.iter_mut().for_each(|k| *k = cp::apply(*k));
    keys
}
