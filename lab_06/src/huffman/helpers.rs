use super::*;

#[allow(dead_code)]
fn print_codes(codes: &[BitVec]) {
    codes
        .iter()
        .enumerate()
        .filter(|(_, code)| code.len() != 0)
        .for_each(|(i, code)| {
            println!("{} {}", i, code);
        });
}

#[allow(dead_code)]
fn print_codes_summary(codes: &[BitVec]) {
    let bits = codes.iter().fold(0, |sum, a| a.len() + sum);
    let bytes = (bits - 1) / 8 + 1;
    println!(
        "Codes summary: {} bits ({} bytes);",
        bits.separated_string(),
        bytes.separated_string()
    );
}

fn print_data_info(output: &BitVec) {
    println!(
        "Compressed data-only size: {} bytes ({} bits)",
        output.bytes().separated_string(),
        output.len().separated_string()
    );
}

pub(crate) fn compress_inner(input: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let codes = get_codes(input);
    let mut output = BitVec::new();
    print_codes_summary(&codes);
    input
        .iter()
        .for_each(|&i| output.concat(&codes[i as usize]));
    print_data_info(&output);
    let serialized_codes = serialize_codes(&codes);
    (
        bincode::serialize(&serialized_codes).unwrap(),
        bincode::serialize(&output).unwrap(),
    )
}

fn rev_map(codes: Vec<BitVec>) -> BTreeMap<BitVec, u8> {
    codes
        .into_iter()
        .enumerate()
        .map(|(i, code)| (code, i as u8))
        .collect()
}

pub(crate) fn decompress_inner(meta: BitVec, data: BitVec) -> Vec<u8> {
    let codes = deserialize_codes(&meta);
    let codes_map = rev_map(codes);
    let mut offset = 0;
    let mut output = Vec::new();
    while offset < data.len() {
        let mut current_code = BitVec::new();
        current_code.push_back(data.get(offset));
        offset += 1;
        while let None = codes_map.get(&current_code) {
            current_code.push_back(data.get(offset));
            offset += 1;
        }
        output.push(*codes_map.get(&current_code).unwrap());
    }
    output
}
