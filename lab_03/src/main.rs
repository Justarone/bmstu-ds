use common::{generate_matches, get_aes_key, get_filenames, get_files, get_key, input_from_file};
use std::io::prelude::*;

mod aes;
mod common;
mod des;

fn main() {
    let matches = generate_matches();
    let (input_filename, output_filename) = get_filenames(&matches);
    let (input_file, mut output_file) =
        get_files(input_filename, output_filename).expect("Problems with files..");

    let mut input = input_from_file(input_file).expect("Can't read file..");
    let des_key = get_key(&matches);
    let aes_key = get_aes_key(&matches);

    match (matches.is_present("aes"), matches.is_present("decode")) {
        (false, false) => des::cipher(&mut input, des_key),
        (false, true) => des::decipher(&mut input, des_key),
        (true, false) => aes::cipher(&mut input, &aes_key),
        (true, true) => aes::decipher(&mut input, &aes_key),
    }

    output_file
        .write_all(&input)
        .expect("Can't write output to file..");
}
