use super::*;

fn build_codes(root: Rc<Option<TreeNode<u8>>>) -> Vec<BitVec> {
    let mut codes: Vec<BitVec> = vec![BitVec::new(); 256];
    let mut empty_prefix = BitVec::new();
    walk_tree(root, &mut empty_prefix, &mut codes);
    codes
}

pub(crate) fn get_codes(input: &[u8]) -> Vec<BitVec> {
    let mut freqs = vec![0 as FreqType; 256];
    input.iter().for_each(|&e| freqs[e as usize] += 1);
    let tree = build_tree(&freqs);
    build_codes(tree)
}

pub(crate) fn serialize_codes(codes: &[BitVec]) -> BitVec {
    let mut res = BitVec::new();
    for c in codes {
        res.push_u8(c.len() as u8);
        res.concat(c);
    }
    res
}

pub(crate) fn deserialize_codes(source: &BitVec) -> Vec<BitVec> {
    let mut offset = 0;
    let mut codes = vec![BitVec::new(); 256];
    for i in 0..=255 {
        let size = source.get_range_u8(offset);
        offset += 8;
        codes[i] = source.get_range(offset, size as usize);
        offset += size as usize;
    }
    codes
}
