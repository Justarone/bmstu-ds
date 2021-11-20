#![feature(map_first_last)]

use common::{generate_matches, get_filenames, get_files, input_from_file};
use std::io::prelude::*;

mod common;
mod huffman;

fn main() {
    let matches = generate_matches();
    let (input_filename, output_filename) = get_filenames(&matches);
    let (input_file, mut output_file) =
        get_files(input_filename, output_filename).expect("Problems with files..");

    let input = input_from_file(input_file).expect("Can't read file..");

    let output = if common::get_decompress(&matches) {
        huffman::decompress(&input)
    } else {
        huffman::compress(&input)
    };

    output_file
        .write_all(&output)
        .expect("Can't write output to file..");
}
