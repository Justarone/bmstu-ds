use separator::Separatable;
use std::{collections::BTreeMap, rc::Rc};

type FreqType = u32;

mod bitvec;
use bitvec::BitVec;

mod treenode;
use treenode::TreeNode;

mod codes;
mod tree;
use codes::{deserialize_codes, get_codes, serialize_codes};
use tree::{build_tree, walk_tree};

mod helpers;
use helpers::{compress_inner, decompress_inner};

fn print_meta_and_data(meta: usize, data: usize) {
    println!(
        "Serialized meta: {} bytes;\nSerialized data: {} bytes;",
        meta.separated_string(),
        data.separated_string()
    );
}

fn print_output_info(output: &[u8]) {
    println!(
        "Decompressed data size: {} bytes;",
        output.len().separated_string()
    );
}

pub(crate) fn compress(input: &[u8]) -> Vec<u8> {
    println!("COMPRESSING...");
    println!("Input data size: {} bytes;", input.len().separated_string());
    let (mut meta, mut data) = compress_inner(input);
    print_meta_and_data(meta.len(), data.len());
    meta.append(&mut data);
    meta
}

pub(crate) fn decompress(input: &[u8]) -> Vec<u8> {
    println!("DECOMPRESSING...");
    println!("Input data size: {} bytes;", input.len().separated_string());
    let meta: BitVec = bincode::deserialize(input).unwrap();
    let size = bincode::serialized_size(&meta).unwrap() as usize;
    let data: BitVec = bincode::deserialize(&input[size..]).unwrap();
    print_meta_and_data(size, input[size..].len());
    let output = decompress_inner(meta, data);
    print_output_info(&output);
    output
}
