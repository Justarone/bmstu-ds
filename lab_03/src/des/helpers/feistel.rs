mod e;
mod p;
mod sblock;

pub(crate) fn f(block: u64, key: u64) -> u64 {
    assert!(block <= !0u32 as u64, "block is {:#x}", block);
    let new_block = e::apply(block);
    let new_block = key ^ new_block;
    let mut res = 0;
    for i in 0..8 {
        let val = new_block.overflowing_shr(6 * i).0 & 0b11_1111;
        let tmp = sblock::get(val as u8, i as usize);
        res |= tmp << (4 * i);
    }
    p::apply(res)
}
